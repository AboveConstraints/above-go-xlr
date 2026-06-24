<template>
  <div>
    <BigButton id="soundboard" ref="sb_button" title="Soundboard" @click="openModal">
      <font-awesome-icon icon="fa-solid fa-music"/>
    </BigButton>
    <AccessibleModal width="660px" ref="modal" id="soundboard" :show_footer="false">
      <template v-slot:title>Soundboard</template>
      <div class="soundboard">
        <div v-if="pads.length === 0" class="empty">
          No sounds yet — drop audio files into your <strong>samples</strong> folder, then reopen this.
        </div>
        <div v-else class="grid">
          <button v-for="name in pads" :key="name" class="pad" :title="name" @click="play(name)">
            {{ displayName(name) }}
          </button>
        </div>
      </div>
    </AccessibleModal>
  </div>
</template>

<script>
import BigButton from "@/components/buttons/BigButton.vue";
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {store} from "@/store";
import {websocket} from "@/util/sockets";

export default {
  name: "SoundboardButton",
  components: {FontAwesomeIcon, AccessibleModal, BigButton},

  computed: {
    pads() {
      let samples = store.getSampleFiles();
      return samples ? Object.keys(samples) : [];
    }
  },

  methods: {
    openModal() {
      this.$refs.modal.openModal(undefined, this.$refs.sb_button);
    },

    play(name) {
      websocket.send_command(store.getActiveSerial(), {"SoundboardPlay": name});
    },

    displayName(name) {
      return name.replace(/\.[^/.]+$/, "");
    }
  }
}
</script>

<style scoped>
.soundboard {
  padding: 12px;
}

.empty {
  color: var(--ag-text-dim);
  text-align: center;
  padding: 24px 12px;
  font-size: 14px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 10px;
}

.pad {
  border: 1px solid var(--ag-text-dim);
  border-radius: 8px;
  background-color: var(--ag-surface);
  color: var(--ag-text);
  font-family: var(--ag-font);
  padding: 18px 10px;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: background-color var(--ag-speed) ease, box-shadow var(--ag-speed) ease;
}

.pad:hover {
  background-color: var(--ag-accent);
  color: #fff;
  box-shadow: 0 0 12px var(--ag-accent-glow);
}

.pad:active {
  transform: translateY(1px);
}
</style>
