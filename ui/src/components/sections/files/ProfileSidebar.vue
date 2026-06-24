<template>
  <div class="rail-wrap">
    <nav class="profile-rail" :aria-label="$t('message.navigation.accessibilityProfileSection')">
      <!-- Brand -->
      <div class="rail-head">
        <img class="rail-logo" src="@/assets/above-logo.png" alt="Above" />
        <span class="rail-brand">Above<span class="rail-brand-accent">GoXLR</span></span>
      </div>

      <div class="rail-scroll">
        <!-- Profile entries (open in modal) -->
        <div class="rail-group">
          <button v-for="entry in entries" :key="entry.key" class="rail-item"
                  @click="openEntry(entry)" :title="entry.label" :aria-label="entry.label">
            <span class="rail-icon">
              <font-awesome-layers v-if="entry.key === 'mic'" class="mic-profile-icon" fixed-width>
                <font-awesome-icon icon="fa-solid fa-microphone-lines" />
                <font-awesome-icon icon="fa-solid fa-user" transform="shrink-9 down-7 right-7" class="mic-user-badge" />
              </font-awesome-layers>
              <font-awesome-icon v-else :icon="entry.icon" />
            </span>
            <span class="rail-label">{{ entry.label }}</span>
          </button>
        </div>

        <div class="rail-divider"></div>

        <!-- System options (each opens its own modal) -->
        <div class="rail-group rail-system">
          <SoundboardButton />
          <MicSetupButton />
          <SwitchDeviceButton v-if="store.getDeviceCount() > 1" />
          <FirmwareUpdateButton />
          <DeviceSettingsButton />
          <PowerButton />
          <AboutButton />
          <LicenseButton />
          <HelpButton />
        </div>
      </div>

      <!-- Footer: utility Settings (gear) pinned at the bottom -->
      <div class="rail-foot">
        <SettingsButton />
      </div>
    </nav>

    <AccessibleModal ref="modal" :width="'620px'" :show_footer="false">
      <template v-slot:title>
        <span class="modal-heading"><font-awesome-icon :icon="activeIcon" /> {{ activeLabel }}</span>
      </template>
      <div class="modal-handler">
        <component :is="activeComponent" v-if="activeComponent" />
      </div>
    </AccessibleModal>
  </div>
</template>

<script>
import { markRaw } from "vue";
import { FontAwesomeLayers } from "@fortawesome/vue-fontawesome";
import { store } from "@/store";
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import ProfileHandler from "@/components/profiles/handlers/ProfileHandler.vue";
import MicProfileHandler from "@/components/profiles/handlers/MicProfileHandler.vue";
import PresetHandler from "@/components/sections/files/PresetHandler.vue";
import SampleHandler from "@/components/sections/files/SampleHandler.vue";
import { isDeviceMini } from "@/util/util";

// System option buttons (each wraps its own trigger + modal)
import MicSetupButton from "@/components/sections/system/modals/MicSetupButton.vue";
import SoundboardButton from "@/components/sections/system/modals/SoundboardButton.vue";
import SwitchDeviceButton from "@/components/sections/system/modals/SwitchDeviceButton.vue";
import FirmwareUpdateButton from "@/components/sections/system/modals/FirmwareUpdateButton.vue";
import PowerButton from "@/components/sections/system/modals/PowerButton.vue";
import DeviceSettingsButton from "@/components/sections/system/modals/DeviceSettingsButton.vue";
import AboutButton from "@/components/sections/system/modals/AboutButton.vue";
import LicenseButton from "@/components/sections/system/modals/LicenseButton.vue";
import HelpButton from "@/components/sections/system/HelpButton.vue";
import SettingsButton from "@/components/sections/system/modals/SettingsButton.vue";

export default {
  name: "ProfileSidebar",
  components: {
    AccessibleModal, FontAwesomeLayers,
    SoundboardButton, MicSetupButton, SwitchDeviceButton, FirmwareUpdateButton, PowerButton,
    DeviceSettingsButton, AboutButton, LicenseButton, HelpButton, SettingsButton,
  },

  data() {
    return {
      activeComponent: null,
      activeLabel: "",
      activeIcon: "fa-solid fa-folder",
    };
  },

  computed: {
    store() {
      return store;
    },
    entries() {
      const all = [
        { key: "profiles", label: this.$t('message.navigation.profiles'), icon: "fa-solid fa-folder", component: ProfileHandler },
        { key: "mic", label: this.$t('message.microphone.profiles.title'), icon: "fa-solid fa-microphone-lines", component: MicProfileHandler },
        { key: "samples", label: this.$t('message.navigation.samples'), icon: "fa-solid fa-music", component: SampleHandler, fullOnly: true },
        { key: "presets", label: this.$t('message.navigation.presets'), icon: "fa-solid fa-book-open", component: PresetHandler, fullOnly: true },
      ];
      return all.filter(e => !(e.fullOnly && isDeviceMini()));
    },
  },

  methods: {
    openEntry(entry) {
      this.activeComponent = markRaw(entry.component);
      this.activeLabel = entry.label;
      this.activeIcon = entry.icon;
      this.$nextTick(() => this.$refs.modal.openModal());
    },
  },
};
</script>

<style scoped>
/* In-flow placeholder reserves the collapsed width; the nav is fixed full-height
   and expands over the content on hover. */
.rail-wrap {
  width: 48px;
  flex-shrink: 0;
}

.profile-rail {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  z-index: 40;
  display: flex;
  flex-direction: column;
  width: 48px;
  padding: 8px 6px;
  background-color: var(--ag-surface);
  border-right: 1px solid var(--ag-divider);
  overflow: hidden;
  transition: width 160ms ease, box-shadow 160ms ease;
}

.profile-rail:hover {
  width: 212px;
  box-shadow: 6px 0 28px rgba(0, 0, 0, 0.45);
}

/* Brand */
.rail-head {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 36px;
  margin-bottom: 8px;
  flex-shrink: 0;
}

.rail-logo {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  flex-shrink: 0;
  border-radius: 7px;
  /* Logo art is transparent with red slashes + white mark, so use a dark
     surface (not the accent fill) so the red reads, with a subtle glow. */
  background-color: var(--ag-surface);
  object-fit: contain;
  padding: 4px;
  box-sizing: border-box;
  box-shadow: 0 0 12px var(--ag-accent-glow);
}

.rail-brand {
  font-size: 15px;
  font-weight: 700;
  letter-spacing: 0.3px;
  color: var(--ag-text);
  white-space: nowrap;
}

.rail-brand-accent {
  color: var(--ag-accent);
  margin-left: 2px;
}

.rail-scroll {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.rail-scroll::-webkit-scrollbar {
  width: 3px;
}

.rail-scroll::-webkit-scrollbar-thumb {
  background-color: var(--ag-divider);
  border-radius: 3px;
}

.rail-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.rail-divider {
  height: 1px;
  margin: 8px 4px;
  background-color: var(--ag-divider);
}

.rail-foot {
  flex-shrink: 0;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--ag-divider);
}

.rail-item {
  display: flex;
  align-items: center;
  gap: 14px;
  width: 100%;
  padding: 8px 0;
  border: none;
  border-radius: var(--ag-radius);
  background-color: transparent;
  color: var(--ag-text-dim);
  font-family: var(--ag-font);
  font-size: 13.5px;
  white-space: nowrap;
  cursor: pointer;
  transition: background-color var(--ag-speed) ease, color var(--ag-speed) ease;
}

.rail-item:hover {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}

.rail-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  font-size: 19px;
  flex-shrink: 0;
}

/* Mic Profiles: microphone with a small crimson "user" badge to differentiate
   it from the plain Mic Setup microphone. */
.mic-profile-icon {
  font-size: 19px;
}

.mic-profile-icon .mic-user-badge {
  color: var(--ag-accent);
  /* a thin surface-coloured outline so the badge separates from the mic */
  stroke: var(--ag-surface);
  stroke-width: 22px;
  paint-order: stroke;
}

.rail-item:hover .mic-profile-icon .mic-user-badge {
  stroke: var(--ag-surface-2);
}

/* ---- Restyle the System BigButtons into rail items ---- */
.rail-system :deep(button.content),
.rail-foot :deep(button.content) {
  display: flex !important;
  flex-direction: row !important;
  align-items: center !important;
  gap: 14px !important;
  width: 100% !important;
  height: auto !important;
  min-width: 0 !important;
  max-width: none !important;
  padding: 8px 0 !important;
  background: transparent !important;
  border: none !important;
  box-shadow: none !important;
  border-radius: var(--ag-radius) !important;
  cursor: pointer !important;
}

.rail-system :deep(.img-section),
.rail-foot :deep(.img-section) {
  height: auto !important;
  width: 36px !important;
  font-size: 19px !important;
  background: transparent !important;
  color: var(--ag-text-dim);
  flex-shrink: 0 !important;
}

.rail-system :deep(.iconTitle),
.rail-foot :deep(.iconTitle) {
  height: auto !important;
  line-height: 1.2 !important;
  padding: 0 !important;
  background: transparent !important;
  color: var(--ag-text-dim);
  font-size: 13.5px !important;
  text-align: left !important;
  border-top: none !important;
}

.rail-system :deep(button.content:hover),
.rail-foot :deep(button.content:hover) {
  background-color: var(--ag-surface-2) !important;
  border-radius: var(--ag-radius);
}

.rail-system :deep(button.content:hover .img-section),
.rail-system :deep(button.content:hover .iconTitle),
.rail-foot :deep(button.content:hover .img-section),
.rail-foot :deep(button.content:hover .iconTitle) {
  color: var(--ag-text);
}

/* Collapsed (not hovered): centre icons, hide labels everywhere */
.profile-rail:not(:hover) .rail-head,
.profile-rail:not(:hover) .rail-item {
  justify-content: center;
  gap: 0;
}

.profile-rail:not(:hover) .rail-label,
.profile-rail:not(:hover) .rail-brand {
  width: 0;
  opacity: 0;
  overflow: hidden;
}

.profile-rail:not(:hover) .rail-system :deep(.iconTitle),
.profile-rail:not(:hover) .rail-foot :deep(.iconTitle) {
  display: none;
}

.profile-rail:not(:hover) .rail-system :deep(button.content),
.profile-rail:not(:hover) .rail-foot :deep(button.content) {
  justify-content: center;
  gap: 0;
}

/* Expanded: align icons with the logo */
.profile-rail:hover .rail-item,
.profile-rail:hover .rail-system :deep(button.content),
.profile-rail:hover .rail-foot :deep(button.content) {
  padding-left: 6px;
}

.rail-label,
.rail-brand {
  transition: opacity 120ms ease;
}

.modal-heading {
  display: inline-flex;
  align-items: center;
  gap: 10px;
}

.modal-handler {
  padding-top: 4px;
}
</style>
