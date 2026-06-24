<template>
  <div class="mixer-wrap">
    <!-- Sub-tab switcher (same pattern as LightingTab) -->
    <div class="sections" role="tablist" :aria-label="$t('message.navigation.mixer')">
      <button class="button" :class="{ active: activeTab === 'inputs' }"
              role="tab" :aria-selected="activeTab === 'inputs'"
              @click="activeTab = 'inputs'">
        {{ $t('message.mixer.inputs') }}
      </button>
      <button class="button" :class="{ active: activeTab === 'outputs' }"
              role="tab" :aria-selected="activeTab === 'outputs'"
              @click="activeTab = 'outputs'">
        {{ $t('message.mixer.outputs') }}
      </button>
    </div>

    <CenteredContainer>
    <!-- Inputs tab -->
    <template v-if="activeTab === 'inputs'">
      <MixAssignment v-if="submixEnabled()"/>

      <GroupContainer v-if="!submixEnabled()" :title="$t('message.mixer.inputs')">
        <template v-if="isSubMixSupported()" #right>
          <label for="submix_enabled">{{ $t('message.mixer.submix') }}</label>
          <input id="submix_enabled" type="checkbox" :checked="submixEnabled()" @change="setSubmixEnabled"/>
        </template>
        <Slider v-for="item in inputMixer" :key="item" :id="channelNames.indexOf(item)" :title="getChannelName(item)"
                :slider-min-value="0" :slider-max-value="255" :text-min-value="0" :text-max-value="100"
                :text-suffix="$t('message.suffixes.percentage')" :slider-value="getValue(item)"
                :store-path="getStorePath(item)" @value-changed="valueChange"
        />
      </GroupContainer>
      <GroupContainer v-else :title="$t('message.mixer.inputs')">
        <template v-if="isSubMixSupported()" #right>
          <label for="submix_enabled">{{ $t('message.mixer.submix') }}</label>
          <input id="submix_enabled" type="checkbox" :checked="submixEnabled()" @change="setSubmixEnabled"/>
        </template>
        <SubmixSlider v-for="item in inputMixer" :key="item" :id="channelNames.indexOf(item)"
                      :title="getChannelName(item)" :slider-min-value="0" :slider-max-value="255"
                      :text-min-value="0" :text-max-value="100" :text-suffix="$t('message.suffixes.percentage')"
                      :slider-a-value="getValue(item)" :slider-b-value="getSubmixValue(item)"
                      :submix-linked="isSubMixLinked(item)" :dimmed="isSubmixDimmed(item)"
                      :store-path="getSubmixPaths(item)" @value-changed="submixValueChange"
                      @submix-linked="submixLinkChanged"
        />
      </GroupContainer>
    </template>

    <!-- Outputs tab -->
    <template v-if="activeTab === 'outputs'">
      <GroupContainer :title="$t('message.mixer.outputs')">
        <Slider v-for="item in outputMixer" :key="item" :id="channelNames.indexOf(item)"
                :title="getChannelName(item)" :slider-min-value="0" :slider-max-value="255"
                :text-min-value="0" :text-max-value="100" :text-suffix="$t('message.suffixes.percentage')"
                :slider-value="getValue(item)" :store-path="getStorePath(item)" @value-changed="valueChange"
                v-show="!submixEnabled() || !submixHide.includes(item)"
        />
      </GroupContainer>
    </template>
  </CenteredContainer>
  </div>
</template>

<script>
import Slider from "../slider/Slider.vue";
import {
  ChannelName,
  channelNameToInputDevice,
  InputMixer,
  OutputDevice,
  OutputMixer,
  OutputMixerSubmixHidden,
} from "@/util/mixerMapping";
import {store} from "@/store";
import {websocket} from "@/util/sockets";
import GroupContainer from "@/components/containers/GroupContainer.vue";
import ExpandoGroupContainer from "@/components/containers/ExpandoGroupContainer.vue";
import SubmixSlider from "@/components/slider/SubmixSlider.vue";
import CenteredContainer from "@/components/containers/CenteredContainer.vue";
import MixAssignment from "@/components/sections/mixer/MixAssignment.vue";

export default {
  name: "MixerTop",
  components: {MixAssignment, CenteredContainer, SubmixSlider, GroupContainer, ExpandoGroupContainer, Slider},

  data() {
    return {
      outputMixer: OutputMixer,
      submixHide: OutputMixerSubmixHidden,
      outputDevices: OutputDevice,
      channelNames: ChannelName,

      activeTab: 'inputs',
      updatesPaused: false,
      volumes: [],
    }
  },

  methods: {
    getChannelName(channel) {
      return this.$t(`message.channels.${channel}`);
    },

    valueChange(id, volume, last) {
      let str_id = this.channelNames[id];
      let command = undefined;

      command = {
        "SetVolume": [
          str_id,
          volume
        ]
      };

      if (!this.updatesPaused || last) {
        this.updatesPaused = true;
        websocket.send_command(store.getActiveSerial(), command).then(() => this.updatesPaused = false);
      }

      store.getActiveDevice().levels.volumes[str_id] = volume;
    },

    submixValueChange(id, volume, side, last) {
      let str_id = this.channelNames[id];
      let command = undefined;
      if (side === 'A') {
        command = {
          "SetVolume": [str_id, volume]
        };
        store.getActiveDevice().levels.volumes[str_id] = volume;
      } else {
        command = {
          "SetSubMixVolume": [str_id, volume]
        };
        store.getActiveDevice().levels.submix.inputs[str_id].volume = volume;
      }

      if (!this.updatesPaused || last) {
        this.updatesPaused = true;
        websocket.send_command(store.getActiveSerial(), command).then(() => this.updatesPaused = false);
      }

      if (store.getActiveDevice().levels.submix.inputs[str_id].linked) {
        if (side === 'A') {
          this.syncSubmix(str_id, volume);
        } else {
          this.syncMix(str_id, volume);
        }
      }
    },

    syncMix(name, volume) {
      let ratio = store.getActiveDevice().levels.submix.inputs[name].ratio;

      // Calculate the Main volume..
      let calculated_volume = parseInt(volume) / ratio;
      store.getActiveDevice().levels.volumes[name] = Math.min(Math.floor(calculated_volume), 255);
    },

    syncSubmix(name, volume) {
      let ratio = store.getActiveDevice().levels.submix.inputs[name].ratio;

      // Calculate the Submix Volume..
      let calculated_volume = Math.min(parseInt(volume) * ratio, 255);
      store.getActiveDevice().levels.submix.inputs[name].volume = Math.max(Math.floor(calculated_volume), 0);
    },

    getValue(id) {
      if (id === 11) {
        return store.getActiveDevice().levels.bleep;
      }
      return store.getActiveDevice().levels.volumes[id];
    },

    getSubmixValue(name) {
      return store.getActiveDevice().levels.submix.inputs[name].volume;
    },

    getStorePath(id) {
      return "/mixers/" + store.getActiveSerial() + "/levels/volumes/" + id;
    },

    getSubmixPaths(id) {
      return this.getStorePath(id) + ";/mixers/" + store.getActiveSerial() + "/levels/submix/inputs/" + id + "/volume"
    },

    isSubMixLinked(name) {
      return store.getActiveDevice().levels.submix.inputs[name].linked;
    },

    isSubmixDimmed(name) {
      // Get the Active Monitor...
      let monitor = store.getActiveDevice().levels.output_monitor;
      return !store.getActiveDevice().router[channelNameToInputDevice(name)][monitor];
    },

    submixEnabled() {
      if (!this.isSubMixSupported()) {
        return false;
      }

      return store.getActiveDevice().levels.submix !== null;
    },

    setSubmixEnabled(e) {
      let command = {
        "SetSubMixEnabled": e.target.checked
      };

      websocket.send_command(store.getActiveSerial(), command);
    },

    isSubMixSupported() {
      return store.getActiveDevice().levels.submix_supported;
    },

    getOutputMix(name) {
      return store.getActiveDevice().levels.submix.outputs[name];
    },

    submixLinkChanged(id, value) {
      let str_id = this.channelNames[id];
      let command = {
        "SetSubMixLinked": [str_id, value]
      };
      websocket.send_command(store.getActiveSerial(), command);
    }
  },

  computed: {
    /**
     * Orders the Inputs panel to mirror the physical device: channels currently
     * assigned to faders A, B, C, D appear first (in that order), followed by any
     * remaining input channels in their default order.
     */
    inputMixer() {
      const device = store.getActiveDevice();
      if (!device || !device.fader_status) {
        return InputMixer;
      }

      const ordered = [];
      const seen = new Set();

      for (const fader of ["A", "B", "C", "D"]) {
        const channel = device.fader_status[fader]?.channel;
        if (channel && InputMixer.includes(channel) && !seen.has(channel)) {
          seen.add(channel);
          ordered.push(channel);
        }
      }

      for (const channel of InputMixer) {
        if (!seen.has(channel)) {
          seen.add(channel);
          ordered.push(channel);
        }
      }

      return ordered;
    }
  }
}
</script>

<style scoped>
label {
  color: var(--ag-text);
}

.mixer-wrap {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
}

.sections {
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 2px;
  margin-bottom: 15px;
}

.sections > button:first-child {
  border-radius: 5px 0 0 5px;
}

.sections > button:last-child {
  border-radius: 0 5px 5px 0;
}

.button {
  min-width: 100px;
  padding: 6px 16px;
  color: var(--ag-text);
  background-color: var(--ag-surface);
  border: none;
  cursor: pointer;
}

.button:not(.active):hover {
  background-color: var(--ag-divider);
}

.button.active {
  color: var(--ag-surface-2);
  background-color: var(--ag-accent);
}
</style>
