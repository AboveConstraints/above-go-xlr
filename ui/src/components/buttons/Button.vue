<template>
  <div ref="button" style="display: flex" class="button" v-bind:class="{ active: isActive, disabled: isDisabled }"
       @click="setActive">
    <div class="left_side">
      <slot name="left">{{ label }}</slot>
    </div>
    <div ref="right" class="right_side">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "PushButton",

  props: {
    buttonId: String,
    label: String,
    isActive: Boolean,
    isDisabled: Boolean,
    padding: {type: String, required: false, default: "8px"}
  },

  methods: {
    setActive() {
      // Don't emit for a disabled button.
      if (!this.isDisabled) {
        this.$emit('button-pressed', this.buttonId);
      }
    }
  },

  computed: {
    right_width() {
      return this.$refs.right.clientWidth + "px";
    }
  }

}
</script>

<style scoped>
.button {
  box-sizing: border-box;
  width: calc(100% - 16px);
  margin: 8px;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius-sm);
  padding: v-bind(padding);
  text-align: left;
  color: var(--ag-text-dim);
  cursor: pointer;
  transition: background-color var(--ag-speed) ease,
              color var(--ag-speed) ease,
              border-color var(--ag-speed) ease;
}

.button:hover:not(.active):not(.disabled) {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
  border-color: var(--ag-text-dim);
}

.button:first-child {
  margin-top: 0;
}

.button:last-child {
  margin-bottom: 0;
}

.left_side {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: calc(100% - v-bind(right_width));
}

.right_side {
}

.active {
  background-color: var(--ag-accent);
  color: var(--ag-text);
}

.disabled {
  background-color: var(--ag-surface-2);
  color: var(--ag-text-dim);
}
</style>
