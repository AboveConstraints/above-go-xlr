<template>
  <div>
    <BigButton id="about_button" ref="button" :title="$t('message.system.aboutButton')"
               @button-clicked="$refs.aboutModal.openModal(undefined, $refs.button)">
      <font-awesome-icon icon="fa-solid fa-circle-info"/>
    </BigButton>

    <AccessibleModal ref="aboutModal" id="about_modal" :show_footer="false" width="480px">
      <template v-slot:title>{{ $t('message.system.aboutButton') }}</template>

      <div class="about-body">

        <!-- Brand hero -->
        <div class="brand-hero">
          <div class="brand-logo">A</div>
          <div class="brand-text">
            <div class="brand-name">Above<span class="brand-accent">GoXLR</span></div>
            <div class="brand-sub">Above Constraints Edition</div>
          </div>
        </div>

        <!-- Info cards -->
        <div class="info-grid">
          <div class="info-card">
            <div class="info-label">{{ $t('message.system.about.serial') }}</div>
            <div class="info-value mono">{{ getSerial() }}</div>
          </div>

          <div class="info-card">
            <div class="info-label">{{ $t('message.system.about.utilityVersion') }}</div>
            <div class="info-value mono">{{ getUtilityVersion() }}</div>
          </div>

          <div class="info-card">
            <div class="info-label">{{ $t('message.system.about.driverVersion') }}</div>
            <div class="info-value mono">{{ getDriverVersion() }}</div>
          </div>

          <div class="info-card full">
            <div class="info-label">{{ $t('message.system.about.hardwareVersion') }}</div>
            <div class="hw-rows">
              <div class="hw-row">
                <span class="hw-key">{{ $t('message.system.about.firmware') }}</span>
                <span class="info-value mono">{{ getFirmwareVersion() }}</span>
              </div>
              <div class="hw-row">
                <span class="hw-key">{{ $t('message.system.about.dice') }}</span>
                <span class="info-value mono">{{ getDice() }}</span>
              </div>
              <div class="hw-row">
                <span class="hw-key">{{ $t('message.system.about.fpga') }}</span>
                <span class="info-value mono">{{ getFPGACount() }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Fork attribution -->
        <div class="fork-notice">
          <font-awesome-icon icon="fa-brands fa-github" class="fork-icon"/>
          <span>
            This is a fork of
            <a href="https://github.com/GoXLR-on-Linux/goxlr-utility" target="_blank" rel="noopener" class="fork-link">
              goxlr-utility
            </a>
            by GoXLR-on-Linux, modified by Above Constraints.
          </span>
        </div>

      </div>
    </AccessibleModal>
  </div>
</template>

<script>
import BigButton from "@/components/buttons/BigButton.vue";
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import { store } from "@/store";
import { isWindowsDriver } from "@/util/util";

export default {
  name: "AboutButton",
  components: { AccessibleModal, BigButton },

  methods: {
    getSerial() { return store.getActiveSerial(); },
    getUtilityVersion() { return store.getVersion(); },

    getDriverVersion() {
      let version = "Unknown";
      if (store.getConfig() === undefined) return version;
      if (store.getConfig().driver_interface.version !== null) {
        version = this.buildVersionString(store.getConfig().driver_interface.version);
      }
      return (isWindowsDriver() ? "TC-Helicon Driver" : "libUSB") + " (" + version + ")";
    },

    getFirmwareVersion() { return this.buildVersionString(store.getActiveDevice().hardware.versions.firmware); },
    getDice() { return this.buildVersionString(store.getActiveDevice().hardware.versions.dice); },
    getFPGACount() { return store.getActiveDevice().hardware.versions.fpga_count; },

    buildVersionString(version) {
      let out = "";
      for (let i = 0; i < version.length; i++) {
        if (version[i] == null) return out;
        if (out !== "") out += ".";
        out += version[i];
      }
      return out;
    },
  }
}
</script>

<style scoped>
.about-body {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 4px 0 0;
}

/* Brand hero */
.brand-hero {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px;
  background: linear-gradient(135deg, var(--ag-surface) 0%, rgba(193,35,80,0.06) 100%);
  border: 1px solid var(--ag-divider);
  border-left: 3px solid var(--ag-accent);
  border-radius: var(--ag-radius);
}

.brand-logo {
  width: 44px;
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--ag-accent);
  border-radius: 10px;
  font-size: 22px;
  font-weight: 800;
  color: #fff;
  flex-shrink: 0;
  box-shadow: 0 0 18px var(--ag-accent-glow);
}

.brand-name {
  font-size: 18px;
  font-weight: 700;
  color: var(--ag-text);
  letter-spacing: 0.3px;
}

.brand-accent {
  color: var(--ag-accent);
  margin-left: 2px;
}

.brand-sub {
  font-size: 11px;
  color: var(--ag-text-dim);
  margin-top: 2px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

/* Info grid */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.info-card {
  background: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  padding: 10px 12px;
}

.info-card.full {
  grid-column: 1 / -1;
}

.info-label {
  font-size: 9px;
  font-weight: 700;
  letter-spacing: 0.15em;
  text-transform: uppercase;
  color: var(--ag-text-dim);
  margin-bottom: 4px;
}

.info-value {
  font-size: 13px;
  color: var(--ag-text);
  font-weight: 500;
}

.mono {
  font-family: "Courier New", Courier, monospace;
  font-size: 12px;
  color: var(--ag-accent);
}

/* Hardware sub-rows */
.hw-rows {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.hw-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.hw-key {
  font-size: 12px;
  color: var(--ag-text-dim);
  font-weight: 500;
  min-width: 70px;
}

/* Fork attribution */
.fork-notice {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 10px 12px;
  background: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  font-size: 11px;
  color: var(--ag-text-dim);
  line-height: 1.5;
}

.fork-icon {
  margin-top: 1px;
  font-size: 13px;
  flex-shrink: 0;
  color: var(--ag-text-dim);
}

.fork-link {
  color: var(--ag-accent);
  text-decoration: none;
  font-weight: 600;
}

.fork-link:hover {
  text-decoration: underline;
}
</style>
