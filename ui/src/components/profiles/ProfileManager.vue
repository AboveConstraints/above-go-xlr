<template>
  <div class="manager-wrap">
    <div class="list-area">
      <ProfileButtonList ref="buttonList">
        <ProfileButton v-for="(name, index) in profileList" :key="index" :button-id="name"
                       :label="name" :is-selected="isSelectedProfile(name)" :is-active="isActiveProfile(name)"
                       @button-clicked="handleButtonPress"
                       @button-double-clicked="handleDoubleClick"
        >
          <template #right v-if="menuList.length > 0">
            <button :ref="getButtonId(name)"
                    :aria-label="$t('message.profileManager.accessibilityDropMenuTitle', { profileName: name })"
                    :id="getButtonId(name)" aria-haspopup="menu" aria-controls="profile_menu"
                    @click.prevent.stop="menuPressed($event, getButtonId(name), name)">
              <font-awesome-icon icon="fa-solid fa-ellipsis-vertical"/>
            </button>
          </template>
        </ProfileButton>
      </ProfileButtonList>
    </div>

    <div class="action-bar">
      <button ref="save" class="action-btn"
              :title="$t('message.profileManager.saveProfileName', {profileName: activeProfile})"
              @click="$refs.saveModal.openModal($refs.focusOk, $refs.save)">
        <font-awesome-icon icon="fa-solid fa-floppy-disk"/>
        <span>{{ $t('message.profileManager.saveProfileName', {profileName: activeProfile}) }}</span>
      </button>
      <button ref="new" class="action-btn action-btn--accent"
              :title="$t('message.profileManager.createProfile')"
              @click="$refs.newModal.openModal($refs.focusDefault, $refs.new)">
        <font-awesome-icon icon="fa-solid fa-file-circle-plus"/>
        <span>{{ $t('message.profileManager.createProfile') }}</span>
      </button>
    </div>
  </div>

  <DropMenu :options="menuList" ref="contextMenu" @option-clicked="optionClicked" menu_id="profile_menu"/>

  <AccessibleModal ref="saveModal" id="saveProfile" :show_close=false>
    <template v-slot:title>{{$t('message.profileManager.overwriteTitle')}}</template>
    <template v-slot:default>{{ $t('message.profileManager.overwriteQuestion', { activeProfile: activeProfile }) }}</template>
    <template v-slot:footer>
      <ModalButton ref="focusOk" @click="saveActiveProfile(); $refs.saveModal.closeModal()">{{ $t('message.profileManager.overwriteYes') }}</ModalButton>
      <ModalButton @click="$refs.saveModal.closeModal()">{{ $t('message.profileManager.overwriteNo') }}</ModalButton>
    </template>
  </AccessibleModal>

  <AccessibleModal ref="newModal" id="newProfile">
    <template v-slot:title>{{$t('message.profileManager.newTitle')}}</template>
    <template v-slot:default>{{$t('message.profileManager.newQuestion')}}</template>
    <template v-slot:footer>
      <ModalButton ref="focusDefault" @click="createNewProfile = true; $refs.newModal.closeModal(); $refs.nameModal.openModal($refs.newName, $refs.new)">{{$t('message.profileManager.newDefaultButton')}}</ModalButton>
      <ModalButton @click="createNewProfile = false; $refs.newModal.closeModal(); $refs.nameModal.openModal($refs.newName, $refs.new)">{{$t('message.profileManager.newCurrentButton')}}</ModalButton>
      <ModalButton @click="$refs.newModal.closeModal()">{{$t('message.profileManager.newCancelButton')}}</ModalButton>
    </template>
  </AccessibleModal>

  <AccessibleModal ref="nameModal" id="nameProfile">
    <template v-slot:title>{{$t('message.profileManager.newNameTitle')}}</template>
    <template v-slot:default>
      <ModalInput ref="newName" v-model="newProfileName"
                  :placeholder="$t('message.profileManager.newNamePlaceHolder')"
                  @on-enter="$refs.nameModal.closeModal(); newProfile(); newProfileName = ''"/>
    </template>
    <template v-slot:footer>
      <ModalButton @click="$refs.nameModal.closeModal(); newProfile(); newProfileName = ''">{{$t('message.profileManager.newNameOk')}}</ModalButton>
      <ModalButton @click="$refs.nameModal.closeModal(); newProfileName = ''">{{$t('message.profileManager.newNameCancel')}}</ModalButton>
    </template>
  </AccessibleModal>
</template>

<script>
import ProfileButtonList from "@/components/profiles/ProfileButtonList.vue";
import ProfileButton from "@/components/profiles/ProfileButton.vue";
import ModalButton from "@/components/design/modal/ModalButton.vue";
import ModalInput from "@/components/design/modal/ModalInput.vue";
import DropMenu from "@/components/design/DropMenu.vue";
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

export default {
  emits: ['new-profile', 'load-profile', 'save-profile', 'save-profile-as', 'menu-item-pressed'],
  name: "ProfileManager",
  components: { FontAwesomeIcon, AccessibleModal, DropMenu, ModalInput, ModalButton, ProfileButton, ProfileButtonList },
  props: {
    activeProfile: String,
    profileList: Array,
    menuList: { type: Array, default: () => ([]) },
  },

  data() {
    return {
      selectedProfile: '',
      createNewProfile: false,
      newProfileName: ''
    }
  },

  methods: {
    isActiveProfile(label) { return label === this.activeProfile; },
    isSelectedProfile(label) { return label === this.selectedProfile; },
    isDeleteDisabled() { return (this.activeProfile === this.selectedProfile) || (this.selectedProfile === ''); },

    handleButtonPress(label) { this.selectedProfile = label; },
    handleDoubleClick(label) { this.$emit("load-profile", label); },

    getButtonId(profile_name) {
      return profile_name.toLowerCase().replace(" ", "_").replace("(", "_").replace(")", "_") + "_profile_button";
    },

    newProfile() {
      if (this.newProfileName === "") return;
      if (this.createNewProfile) {
        this.$emit('new-profile', this.newProfileName);
      } else {
        this.$emit('save-profile-as', this.newProfileName);
      }
      this.newProfileName = "";
      this.createNewProfile = false;
    },

    saveActiveProfile() { this.$emit('save-profile'); },

    menuPressed(event, return_id, item) {
      this.handleButtonPress(item);
      this.$refs.contextMenu.showMenu(event, item, return_id, this.$refs.buttonList.$refs.selectorList.scrollTop);
    },

    optionClicked(event) { this.$emit('menu-item-pressed', event); },
    getMenuButton() { return this.$refs.menuButton; }
  }
}
</script>

<style scoped>
.manager-wrap {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 10px;
}

.list-area {
  flex: 1;
  min-height: 0;
  background: var(--ag-surface-2);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  padding: 6px;
  overflow: hidden;
}

.action-bar {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 7px;
  height: 36px;
  padding: 0 14px;
  background: var(--ag-surface-2);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  color: var(--ag-text-dim);
  font-family: var(--ag-font);
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.04em;
  cursor: pointer;
  transition: background-color var(--ag-speed) ease,
              border-color var(--ag-speed) ease,
              color var(--ag-speed) ease,
              box-shadow var(--ag-speed) ease;
}

.action-btn:hover {
  background: var(--ag-surface);
  border-color: var(--ag-accent-dim);
  color: var(--ag-text);
  box-shadow: 0 0 10px var(--ag-accent-glow);
}

.action-btn--accent {
  background: var(--ag-accent-dim);
  border-color: var(--ag-accent);
  color: #fff;
}

.action-btn--accent:hover {
  background: var(--ag-accent);
  border-color: var(--ag-accent);
  color: #fff;
  box-shadow: 0 0 14px var(--ag-accent-glow);
}
</style>
