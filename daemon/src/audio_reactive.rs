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

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use goxlr_audio::AtomicF64;
use log::{debug, info, warn};

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

/// Number of frequency bands (one per fader strip).
pub const NUM_BANDS: usize = 4;

/// Approx. centre frequencies (Hz) for the bands: bass → treble.
const BAND_CENTRES: [f32; NUM_BANDS] = [80.0, 350.0, 1500.0, 6000.0];
const BAND_Q: f32 = 1.0;

/// Per-band silence floor. Band-pass outputs are much smaller than the full
/// signal, so this is far below the overall `noise_floor` — otherwise weaker
/// bands (mids/treble) get gated to zero and only the bass strip lights up.
const BAND_FLOOR: f32 = 1.0e-3;

/// One RBJ band-pass biquad (transposed direct form II) + per-band envelope and
/// auto-gain. Each drives one fader strip.
#[derive(Debug, Clone)]
struct Band {
    // Biquad coefficients (a0-normalised).
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
    // Envelope + auto-gain.
    envelope: f32,
    agc_peak: f32,
    level: f32,
}

impl Band {
    fn new(centre: f32, sample_rate: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * (centre / sample_rate);
        let (sin, cos) = w0.sin_cos();
        let alpha = sin / (2.0 * BAND_Q);
        let a0 = 1.0 + alpha;
        Self {
            b0: alpha / a0,
            b1: 0.0,
            b2: -alpha / a0,
            a1: (-2.0 * cos) / a0,
            a2: (1.0 - alpha) / a0,
            z1: 0.0,
            z2: 0.0,
            envelope: 0.0,
            agc_peak: BAND_FLOOR,
            level: 0.0,
        }
    }

    #[inline]
    fn filter(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
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

    // Per-band spectrum split (one per fader strip).
    bands: [Band; NUM_BANDS],

    // Latest outputs.
    intensity: f32,
    beat: bool,
    // Beat-flash envelope: jumps to 1.0 on a beat then decays. Drives Pulse mode.
    flash: f32,
}

impl ReactiveAnalyser {
    pub fn new(sample_rate: u32, config: ReactiveConfig) -> Self {
        let sr = sample_rate.max(1) as f32;
        let bands = std::array::from_fn(|i| Band::new(BAND_CENTRES[i], sr));
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
            bands,
            intensity: 0.0,
            beat: false,
            flash: 0.0,
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
        let (att, rel) = (self.attack_coeff, self.release_coeff);

        for &s in samples {
            let mag = s.abs();

            // One-pole envelope follower with separate attack/release.
            let coeff = if mag > self.envelope { att } else { rel };
            self.envelope += (mag - self.envelope) * coeff;

            // Per-band: filter, rectify, envelope-follow.
            for band in &mut self.bands {
                let y = band.filter(s).abs();
                let c = if y > band.envelope { att } else { rel };
                band.envelope += (y - band.envelope) * c;
            }

            block_energy += s * s;

            self.samples_since_beat = self.samples_since_beat.saturating_add(1);
        }

        // Auto-gain + gate each band into a 0..1 level. Uses the much lower
        // BAND_FLOOR so quieter bands still normalise to full range.
        for band in &mut self.bands {
            if band.envelope > band.agc_peak {
                band.agc_peak = band.envelope;
            } else {
                band.agc_peak += (band.envelope - band.agc_peak) * self.agc_coeff;
                band.agc_peak = band.agc_peak.max(BAND_FLOOR);
            }
            let norm = (band.envelope / band.agc_peak).clamp(0.0, 1.0);
            band.level = if band.envelope < BAND_FLOOR { 0.0 } else { norm };
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

        // Beat-flash envelope: snap up on a beat, decay otherwise.
        if beat_this_block {
            self.flash = 1.0;
        } else {
            self.flash *= 0.90;
        }
    }

    /// Per-band auto-gained levels (bass → treble), one per fader strip.
    pub fn bands(&self) -> [f32; NUM_BANDS] {
        std::array::from_fn(|i| self.bands[i].level)
    }

    /// Beat-flash envelope (1.0 on a beat, decaying). Drives Pulse mode.
    pub fn flash(&self) -> f32 {
        self.flash
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

/// Owns a background system-audio (loopback) capture that continuously feeds the
/// [`ReactiveAnalyser`] and publishes the latest `intensity` (0.0..=1.0) via a
/// shared atomic the daemon reads each device tick.
///
/// Fully self-contained: creating one starts capture, dropping one stops it. If
/// the platform can't open a loopback stream (or it's not f32), it logs and the
/// intensity simply stays at 0.0 — the rest of the daemon is unaffected.
pub struct ReactiveController {
    intensity: Arc<AtomicF64>,
    bands: Arc<[AtomicF64; NUM_BANDS]>,
    flash: Arc<AtomicF64>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ReactiveController {
    pub fn start(config: ReactiveConfig) -> Self {
        let intensity = Arc::new(AtomicF64::new(0.0));
        let bands: Arc<[AtomicF64; NUM_BANDS]> =
            Arc::new(std::array::from_fn(|_| AtomicF64::new(0.0)));
        let flash = Arc::new(AtomicF64::new(0.0));
        let stop = Arc::new(AtomicBool::new(false));

        let thread_intensity = intensity.clone();
        let thread_bands = bands.clone();
        let thread_flash = flash.clone();
        let thread_stop = stop.clone();
        let handle = thread::Builder::new()
            .name("reactive-capture".to_string())
            .spawn(move || {
                run_capture(config, thread_intensity, thread_bands, thread_flash, thread_stop)
            })
            .ok();

        if handle.is_none() {
            warn!("Reactive lighting: failed to spawn capture thread");
        }

        Self {
            intensity,
            bands,
            flash,
            stop,
            handle,
        }
    }

    /// Latest smoothed intensity, 0.0..=1.0.
    pub fn intensity(&self) -> f64 {
        self.intensity.load(Ordering::Relaxed)
    }

    /// Latest per-band levels (bass → treble), one per fader strip.
    pub fn bands(&self) -> [f32; NUM_BANDS] {
        std::array::from_fn(|i| self.bands[i].load(Ordering::Relaxed) as f32)
    }

    /// Latest beat-flash value (drives Pulse mode).
    pub fn flash(&self) -> f32 {
        self.flash.load(Ordering::Relaxed) as f32
    }
}

impl Drop for ReactiveController {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        debug!("Reactive lighting: capture stopped");
    }
}

/// Capture-thread body. Opens a WASAPI loopback stream on the default output
/// device, feeds blocks into the analyser, and writes intensity to the atomic.
fn run_capture(
    config: ReactiveConfig,
    intensity: Arc<AtomicF64>,
    bands: Arc<[AtomicF64; NUM_BANDS]>,
    flash: Arc<AtomicF64>,
    stop: Arc<AtomicBool>,
) {
    let host = cpal::default_host();
    let Some(device) = host.default_output_device() else {
        warn!("Reactive lighting: no default output device for loopback capture");
        return;
    };

    let stream_config = match device.default_output_config() {
        Ok(c) => c,
        Err(e) => {
            warn!("Reactive lighting: couldn't get output config: {e}");
            return;
        }
    };

    // We only handle f32 here (the common Windows shared-mode format). Anything
    // else simply no-ops rather than risking bad audio handling.
    if stream_config.sample_format() != cpal::SampleFormat::F32 {
        warn!(
            "Reactive lighting: output format {:?} unsupported (need f32); disabling",
            stream_config.sample_format()
        );
        return;
    }

    let channels = stream_config.channels() as usize;
    let sample_rate = stream_config.sample_rate().0;
    let mut analyser = ReactiveAnalyser::new(sample_rate, config);
    let cb_intensity = intensity.clone();
    let cb_bands = bands.clone();
    let cb_flash = flash.clone();

    let stream = device.build_input_stream(
        &stream_config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mono = downmix_to_mono(data, channels);
            analyser.push(&mono);
            cb_intensity.store(analyser.intensity() as f64, Ordering::Relaxed);
            cb_flash.store(analyser.flash() as f64, Ordering::Relaxed);
            let levels = analyser.bands();
            for (atomic, level) in cb_bands.iter().zip(levels) {
                atomic.store(level as f64, Ordering::Relaxed);
            }
        },
        move |err| warn!("Reactive lighting: capture stream error: {err}"),
        None,
    );

    let stream = match stream {
        Ok(s) => s,
        Err(e) => {
            warn!("Reactive lighting: couldn't open loopback stream: {e}");
            return;
        }
    };

    if let Err(e) = stream.play() {
        warn!("Reactive lighting: couldn't start loopback stream: {e}");
        return;
    }

    info!(
        "Reactive lighting: capturing loopback @ {} Hz, {} ch",
        sample_rate, channels
    );

    // Keep the stream alive on this thread until asked to stop.
    while !stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(50));
    }

    // Reset levels so any final frame settles to dark.
    intensity.store(0.0, Ordering::Relaxed);
    flash.store(0.0, Ordering::Relaxed);
    for atomic in bands.iter() {
        atomic.store(0.0, Ordering::Relaxed);
    }
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
