<template>
  <div class="device-select">
    <div class="ds-card">
      <div class="ds-brand">Above<span class="ds-brand-accent">GoXLR</span></div>
      <div class="ds-title">Select Device</div>

      <!-- If we've never connected before, and we're not connected now.. -->
      <div v-if="!hasConnected() && !isConnected()" class="ds-state">
        <span class="ds-spinner" aria-hidden="true"></span>
        <span>Connecting to the GoXLR Above…</span>
      </div>

      <!-- We *HAVE* connected before, but we're not connected now.. -->
      <div v-else-if="hasConnected() && !isConnected()" class="ds-state ds-state-error">
        <span>Unable to connect to the GoXLR Above.<br />Please check that it's running.</span>
        <span class="ds-substate"><span class="ds-spinner" aria-hidden="true"></span> Retrying automatically…</span>
      </div>

      <!-- We should be connected here! -->
      <div v-else>
        <div class="ds-list" v-if="deviceCount > 0">
          <Button v-for="(device, key) in getMixers()" :key=key :button-id=key :is-active=false
                  :label="getLabel2(key, device)" @button-pressed="setDevice(key)"/>
        </div>
        <div v-else class="ds-state">No GoXLR devices found.</div>
      </div>

      <div v-if="isConnected() && hasConfig()" class="ds-settings">
        <SettingsButton />
      </div>

      <!-- Dev-only: load a captured device snapshot so the UI can be worked on
           without a daemon / physical GoXLR. Stripped from production builds. -->
      <button v-if="isDev" class="ds-dev-skip" @click="loadDemoDevice">
        ⚡ Skip · load demo device (dev only)
      </button>
    </div>
  </div>
</template>

<script>

import Button from "@/components/buttons/Button.vue";
import {store} from "@/store";
import SettingsButton from "@/components/sections/system/modals/SettingsButton.vue";

export default {
  name: "DeviceSelector",
  components: {SettingsButton, Button},
  data() {
    return {
      devices: [],
      // import.meta.env.DEV is true only under the Vite dev server.
      isDev: import.meta.env.DEV,
    }
  },

  computed: {
    deviceCount() {
      return store.getDeviceCount();
    }
  },

  watch: {
    // This code probably isn't needed, but is for when a GoXLR suddenly appears in the data.
    deviceCount(newCount, oldCount) {
      if (newCount === 1 && oldCount === 0) {
        store.setActiveSerial(Object.keys(this.getMixers())[0]);
      }
    }
  },

  methods: {
    hasConnected() {
      return store.hasConnected();
    },

    isConnected() {
      return store.isConnected();
    },

    hasConfig() {
      return store.getConfig() !== undefined;
    },

    getMixers() {
      return store.status.mixers;
    },

    setDevice(serial) {
      store.setActiveSerial(serial);
    },

    getLabel2(serial, device) {
      return "[" + serial + "] GoXLR " + device.hardware.device_type + " connected to USB bus " + device.hardware.usb_device.bus_number + " address " + device.hardware.usb_device.address;
    },

    // Dev-only: inject a captured status snapshot so the full UI renders with no
    // daemon or device connected. The import is dynamic so it's tree-shaken out
    // of production builds.
    async loadDemoDevice() {
      const data = (await import("@/dev/sampleStatus.json")).default;
      store.socketConnected({ Status: data });
      const serial = Object.keys(data.mixers)[0];
      if (serial) {
        store.setActiveSerial(serial);
      }
    },
  },

  created() {
    if (this.deviceCount === 1) {
      store.setActiveSerial(Object.keys(this.getMixers())[0]);
    }
  }
}
</script>

<style scoped>
.device-select {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 80vh;
  padding: 24px;
}

.ds-card {
  width: 100%;
  max-width: 560px;
  padding: 32px 28px;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.45),
              0 0 80px rgba(163, 38, 41, 0.08);
  text-align: center;
}

.ds-brand {
  font-size: 22px;
  font-weight: 700;
  letter-spacing: 0.5px;
  color: var(--ag-text);
  margin-bottom: 4px;
}

.ds-brand-accent {
  color: var(--ag-accent);
  margin-left: 4px;
}

.ds-title {
  font-size: 13px;
  text-transform: uppercase;
  letter-spacing: 2px;
  color: var(--ag-text-dim);
  margin-bottom: 24px;
}

.ds-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  padding: 28px 12px;
  color: var(--ag-text-dim);
  line-height: 1.5;
}

.ds-state-error {
  color: var(--ag-text);
}

.ds-substate {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--ag-text-dim);
}

.ds-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 280px;
  overflow-y: auto;
  padding: 2px;
}

.ds-list::-webkit-scrollbar {
  width: 4px;
}

.ds-list::-webkit-scrollbar-thumb {
  background-color: var(--ag-divider);
  border-radius: 3px;
}

.ds-settings {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid var(--ag-divider);
}

.ds-dev-skip {
  margin-top: 18px;
  padding: 8px 14px;
  width: 100%;
  font-family: var(--ag-font);
  font-size: 13px;
  color: var(--ag-text-dim);
  background-color: transparent;
  border: 1px dashed var(--ag-accent-dim);
  border-radius: var(--ag-radius);
  cursor: pointer;
  transition: all var(--ag-speed) ease;
}

.ds-dev-skip:hover {
  color: var(--ag-text);
  border-color: var(--ag-accent);
  background-color: rgba(227, 29, 85, 0.08);
}

/* Indeterminate spinner */
.ds-spinner {
  display: inline-block;
  width: 18px;
  height: 18px;
  border: 2px solid var(--ag-divider);
  border-top-color: var(--ag-accent);
  border-radius: 50%;
  animation: ds-spin 0.7s linear infinite;
}

@keyframes ds-spin {
  to { transform: rotate(360deg); }
}
</style>
