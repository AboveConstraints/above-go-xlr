//! Audio-reactive lighting — DSP analysis core.
//!
//! This module is the pure, I/O-free heart of the reactive lighting feature
//! ([[feature-reactive-lighting]]). You feed it raw audio samples (mono f32,
//! interleaved frames already down-mixed) and it produces:
//!
//!   * `intensity()`  — a smoothed 0.0..=1.0 level, auto-gained so quiet and
//!                       loud tracks both use the full range. Drives brightness.
//!   * `beat()`       — true on the frame a transient/onset is detected. Can be
//!                       used to flash/pulse on hits.
//!
//! Audio *capture* (cpal loopback/input) and LED *output* (pushing a transient
//! colour-map override to the device) are deliberately kept out of here so this
//! can be unit-tested and tuned without hardware. See the daemon wiring notes in
//! the feature plan.
//!
//! Everything is std-only (no extra deps) so it always compiles.

use std::time::Duration;

/// Tunable parameters for the analyser. Defaults are a reasonable starting
/// point for music; expose these in the UI later for taste.
#[derive(Debug, Clone, Copy)]
pub struct ReactiveConfig {
    /// Envelope attack time — how fast brightness rises toward a louder signal.
    pub attack: Duration,
    /// Envelope release time — how slowly brightness falls after a peak.
    pub release: Duration,
    /// How quickly the auto-gain ceiling adapts to the loudest recent peak.
    /// Larger = slower adaptation (steadier), smaller = snappier.
    pub agc_release: Duration,
    /// Energy must exceed `moving_average * beat_sensitivity` to count as a beat.
    pub beat_sensitivity: f32,
    /// Minimum time between detected beats (debounce / refractory period).
    pub beat_refractory: Duration,
    /// Noise floor below which we treat the signal as silence (0.0..1.0 of full
    /// scale). Prevents idle hiss from lighting things up.
    pub noise_floor: f32,
}

impl Default for ReactiveConfig {
    fn default() -> Self {
        Self {
            attack: Duration::from_millis(8),
            release: Duration::from_millis(180),
            agc_release: Duration::from_millis(2500),
            beat_sensitivity: 1.4,
            beat_refractory: Duration::from_millis(120),
            noise_floor: 0.02,
        }
    }
}

/// Streaming analyser. Create once per capture session with the stream's sample
/// rate, then call [`ReactiveAnalyser::push`] with each captured block.
#[derive(Debug)]
pub struct ReactiveAnalyser {
    sample_rate: f32,

    // Envelope follower (peak amplitude tracker).
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,

    // Auto-gain: slowly-decaying record of the loudest envelope we've seen, used
    // to normalise `intensity` into 0..1 regardless of source loudness.
    agc_peak: f32,
    agc_coeff: f32,

    // Beat / onset detection over short energy blocks.
    energy_avg: f32,
    beat_sensitivity: f32,
    samples_since_beat: u32,
    beat_refractory_samples: u32,

    noise_floor: f32,

    // Latest outputs.
    intensity: f32,
    beat: bool,
}

impl ReactiveAnalyser {
    pub fn new(sample_rate: u32, config: ReactiveConfig) -> Self {
        let sr = sample_rate.max(1) as f32;
        Self {
            sample_rate: sr,
            envelope: 0.0,
            attack_coeff: time_to_coeff(config.attack, sr),
            release_coeff: time_to_coeff(config.release, sr),
            agc_peak: config.noise_floor.max(1e-4),
            agc_coeff: time_to_coeff(config.agc_release, sr),
            energy_avg: 0.0,
            beat_sensitivity: config.beat_sensitivity,
            samples_since_beat: u32::MAX / 2,
            beat_refractory_samples: dur_to_samples(config.beat_refractory, sr),
            noise_floor: config.noise_floor,
            intensity: 0.0,
            beat: false,
        }
    }

    /// Feed a block of mono samples (already down-mixed, range roughly -1.0..1.0).
    /// Updates the internal state; read results via [`Self::intensity`] / [`Self::beat`].
    pub fn push(&mut self, samples: &[f32]) {
        if samples.is_empty() {
            self.beat = false;
            return;
        }

        let mut block_energy = 0.0f32;
        let mut beat_this_block = false;

        for &s in samples {
            let mag = s.abs();

            // One-pole envelope follower with separate attack/release.
            let coeff = if mag > self.envelope {
                self.attack_coeff
            } else {
                self.release_coeff
            };
            self.envelope += (mag - self.envelope) * coeff;

            block_energy += s * s;

            self.samples_since_beat = self.samples_since_beat.saturating_add(1);
        }

        // Track the loudest peak for auto-gain, decaying slowly back down so we
        // re-normalise if the music gets quieter.
        if self.envelope > self.agc_peak {
            self.agc_peak = self.envelope;
        } else {
            self.agc_peak += (self.envelope - self.agc_peak) * self.agc_coeff;
            self.agc_peak = self.agc_peak.max(self.noise_floor);
        }

        // Normalised, gated intensity.
        let norm = (self.envelope / self.agc_peak).clamp(0.0, 1.0);
        self.intensity = if self.envelope < self.noise_floor { 0.0 } else { norm };

        // Beat detection: compare this block's mean energy against a moving
        // average. A sufficiently large jump (and past the refractory period) is
        // a beat.
        let block_mean = block_energy / samples.len() as f32;
        if self.energy_avg <= 0.0 {
            self.energy_avg = block_mean;
        }
        if block_mean > self.energy_avg * self.beat_sensitivity
            && self.envelope >= self.noise_floor
            && self.samples_since_beat >= self.beat_refractory_samples
        {
            beat_this_block = true;
            self.samples_since_beat = 0;
        }
        // Slow-track the running energy average.
        self.energy_avg += (block_mean - self.energy_avg) * 0.1;

        self.beat = beat_this_block;
    }

    /// Smoothed, auto-gained loudness in 0.0..=1.0. Drives LED brightness.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// True for the block in which a beat/onset was detected.
    pub fn beat(&self) -> bool {
        self.beat
    }

    /// Sample rate this analyser was built for (Hz).
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

/// Down-mix an interleaved multi-channel buffer to mono in-place-ish, returning
/// a Vec the analyser can consume. Helper for the capture side.
pub fn downmix_to_mono(interleaved: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return interleaved.to_vec();
    }
    interleaved
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f32>() / frame.len() as f32)
        .collect()
}

/// Convert a one-pole time constant (time to reach ~63% of a step) into a
/// per-sample smoothing coefficient.
fn time_to_coeff(time: Duration, sample_rate: f32) -> f32 {
    let secs = time.as_secs_f32();
    if secs <= 0.0 {
        return 1.0;
    }
    1.0 - (-1.0 / (secs * sample_rate)).exp()
}

fn dur_to_samples(time: Duration, sample_rate: f32) -> u32 {
    (time.as_secs_f32() * sample_rate).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(freq: f32, amp: f32, sample_rate: u32, samples: usize) -> Vec<f32> {
        (0..samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                amp * (2.0 * std::f32::consts::PI * freq * t).sin()
            })
            .collect()
    }

    #[test]
    fn silence_is_zero_intensity() {
        let mut a = ReactiveAnalyser::new(48_000, ReactiveConfig::default());
        a.push(&vec![0.0; 4800]);
        assert!(a.intensity() < 0.01, "got {}", a.intensity());
        assert!(!a.beat());
    }

    #[test]
    fn loud_tone_drives_intensity_up() {
        let mut a = ReactiveAnalyser::new(48_000, ReactiveConfig::default());
        // Feed a few blocks so the envelope settles.
        for _ in 0..10 {
            a.push(&tone(220.0, 0.8, 48_000, 1024));
        }
        assert!(a.intensity() > 0.5, "intensity should rise, got {}", a.intensity());
    }

    #[test]
    fn agc_normalises_quiet_signal() {
        let mut a = ReactiveAnalyser::new(48_000, ReactiveConfig::default());
        // A quiet but steady tone should still climb toward full scale once the
        // auto-gain ceiling adapts.
        for _ in 0..200 {
            a.push(&tone(220.0, 0.1, 48_000, 1024));
        }
        assert!(a.intensity() > 0.4, "AGC should lift quiet signal, got {}", a.intensity());
    }

    #[test]
    fn downmix_averages_channels() {
        let stereo = [1.0, 0.0, 0.5, 0.5];
        let mono = downmix_to_mono(&stereo, 2);
        assert_eq!(mono, vec![0.5, 0.5]);
    }
}
