<template>
  <button ref="button" class="modal-button" :class="{ enabled: enabled }" :disabled="!enabled"><slot /></button>
</template>

<script>
export default {
  name: "ModalButton",

  props: {
    enabled: { type: Boolean, default: true }
  },

  methods: {
    focus() {
      this.$refs.button.focus();
    }
  }
}
</script>

<style scoped>
.modal-button {
  background-color: var(--ag-surface);
  color: var(--ag-text);
  padding: 8px 30px;
  border: none;
  margin: 8px;
  width: 120px;
}

.modal-button:disabled {
  background-color: var(--ag-surface-2);
}

.modal-button:not(:disabled):hover, .modal-button:not(:disabled):focus {
  background-color: var(--ag-text-dim);
}
</style>
