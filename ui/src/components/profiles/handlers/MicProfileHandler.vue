<template>
  <div class="handler-wrap">
    <div class="handler-header">
      <div class="active-profile-chip" v-if="getActiveProfile()">
        <font-awesome-icon icon="fa-solid fa-circle" class="chip-dot"/>
        <span>{{ getActiveProfile() }}</span>
      </div>
      <button class="open-btn" :aria-label="$t('message.profileManager.accessibilityOpenMicProfileDirectory')" @click="openProfiles">
        <font-awesome-icon icon="fa-solid fa-folder-open"/>
        <span>{{ $t('message.microphone.profiles.title') }}</span>
      </button>
    </div>

    <div class="list-wrap">
      <ProfileManager ref="manager" :profile-list="getProfileList()" :active-profile="getActiveProfile()"
                      :menu-list="getMenuList()" @new-profile="newProfile" @load-profile="loadProfile"
                      @save-profile="saveProfile" @save-profile-as="saveProfileAs"
                      @menu-item-pressed="menuItemPressed" />
    </div>
  </div>

  <AccessibleModal ref="deleteMicModal" id="delMicProfile">
    <template v-slot:title>{{ $t('message.profileManager.deleteTitle') }}</template>
    <template v-slot:default>{{ $t('message.profileManager.deleteQuestion', { profileName: selectedProfile }) }}</template>
    <template v-slot:footer>
      <ModalButton @click="$refs.deleteMicModal.closeModal(); deleteProfile(this.selectedProfile)">{{ $t('message.profileManager.deleteYes') }}</ModalButton>
      <ModalButton ref="focusDelDefault" @click="$refs.deleteMicModal.closeModal()">{{ $t('message.profileManager.deleteNo') }}</ModalButton>
    </template>
  </AccessibleModal>

  <AccessibleModal ref="noDelete" id="delMicProfile">
    <template v-slot:title>{{ $t('message.profileManager.deleteCurrentErrorTitle') }}</template>
    <template v-slot:default>{{ $t('message.profileManager.deleteCurrentErrorMessage') }}</template>
  </AccessibleModal>
</template>

<script>
import { store } from "@/store";
import { sendHttpCommand, websocket } from "@/util/sockets";
import ProfileManager from "@/components/profiles/ProfileManager.vue";
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import ModalButton from "@/components/design/modal/ModalButton.vue";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

export default {
  name: "MicProfileHandler",
  components: { FontAwesomeIcon, ModalButton, AccessibleModal, ProfileManager },

  data() {
    return { selectedProfile: '' }
  },

  methods: {
    getMenuList() {
      return [
        { name: this.$t('message.profileManager.menuLoadProfile'), slug: 'load' },
        { name: this.$t('message.profileManager.menuDeleteProfile'), slug: 'delete' }
      ];
    },

    getProfileList() { return store.getMicProfileFiles().sort(Intl.Collator().compare); },
    getActiveProfile() { return store.getActiveDevice().mic_profile_name; },

    menuItemPressed(event) {
      if (event.option.slug === "load") this.loadProfile(event.item);
      if (event.option.slug === "delete") {
        if (event.item === this.getActiveProfile()) {
          this.$refs.noDelete.openModal(this.$refs.focusDelDefault, this.$refs.manager.$refs[this.$refs.manager.getButtonId(event.item)][0]);
        } else {
          this.selectedProfile = event.item;
          this.$refs.deleteMicModal.openModal(this.$refs.focusDelDefault, this.$refs.manager.$refs[this.$refs.manager.getButtonId(event.item)][0]);
        }
      }
    },

    loadProfile(label) {
      sendHttpCommand(store.getActiveSerial(), { LoadMicProfile: [label, true] }).catch(console.log);
    },
    newProfile(name) { sendHttpCommand(store.getActiveSerial(), { NewMicProfile: name }); },
    saveProfile() { sendHttpCommand(store.getActiveSerial(), { SaveMicProfile: [] }); },
    saveProfileAs(name) { sendHttpCommand(store.getActiveSerial(), { SaveMicProfileAs: name }); },
    deleteProfile(name) { sendHttpCommand(store.getActiveSerial(), { DeleteMicProfile: name }); },
    openProfiles() { websocket.open_path("MicProfiles"); }
  }
}
</script>

<style scoped>
.handler-wrap {
  display: flex;
  flex-direction: column;
  height: 340px;
  gap: 12px;
}

.handler-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.active-profile-chip {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: var(--ag-surface-2);
  border: 1px solid var(--ag-divider);
  border-left: 2px solid var(--ag-accent);
  border-radius: var(--ag-radius);
  font-size: 12px;
  font-weight: 600;
  color: var(--ag-text);
  max-width: 200px;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  flex: 1;
  min-width: 0;
}

.chip-dot {
  font-size: 6px;
  color: var(--ag-accent);
  flex-shrink: 0;
  filter: drop-shadow(0 0 4px var(--ag-accent-glow));
}

.open-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  background: transparent;
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  color: var(--ag-text-dim);
  font-family: var(--ag-font);
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.05em;
  cursor: pointer;
  white-space: nowrap;
  flex-shrink: 0;
  transition: border-color var(--ag-speed) ease, color var(--ag-speed) ease;
}

.open-btn:hover {
  border-color: var(--ag-accent-dim);
  color: var(--ag-text);
}

.list-wrap {
  flex: 1;
  min-height: 0;
}
</style>
