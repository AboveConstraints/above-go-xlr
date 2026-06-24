<template>
  <div class="profile-row" :class="{ selected: isSelected, active: isActive }" @click="handleClick">
    <div class="row-left">
      <span class="active-dot" v-if="isActive" aria-label="Active Profile">
        <font-awesome-icon icon="fa-solid fa-circle" />
      </span>
      <span class="row-label">{{ label }}</span>
    </div>
    <div ref="right" class="row-right">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "ProfileButton",

  props: {
    buttonId: String,
    label: String,
    isActive: Boolean,
    isSelected: Boolean,
    showCheck: { type: Boolean, default: true },
    padding: { type: String, required: false, default: "8px" },
  },

  data() {
    return { timeout: null }
  },

  methods: {
    handleClick() {
      let self = this;
      if (!this.timeout) {
        self.$emit('button-clicked', self.buttonId);
        this.timeout = setTimeout(() => { self.timeout = null; }, 350);
      } else {
        clearTimeout(this.timeout);
        this.timeout = null;
        this.$emit('button-double-clicked', this.buttonId);
      }
    },
  },

  computed: {
    right_width() {
      return this.$refs.right ? this.$refs.right.clientWidth + "px" : "32px";
    }
  }
}
</script>

<style scoped>
.profile-row {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 9px 10px 9px 12px;
  border-radius: var(--ag-radius);
  border: 1px solid transparent;
  cursor: pointer;
  transition: background-color var(--ag-speed) ease,
              border-color var(--ag-speed) ease,
              color var(--ag-speed) ease;
  box-sizing: border-box;
}

.profile-row:hover {
  background-color: var(--ag-surface-2);
  border-color: var(--ag-divider);
}

.profile-row.selected {
  background-color: var(--ag-surface-2);
  border-color: var(--ag-divider);
  border-left-color: var(--ag-accent);
  border-left-width: 2px;
}

.profile-row.active .row-label {
  color: var(--ag-text);
  font-weight: 600;
}

.row-left {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  flex: 1;
  overflow: hidden;
}

.active-dot {
  font-size: 6px;
  color: var(--ag-accent);
  flex-shrink: 0;
  filter: drop-shadow(0 0 4px var(--ag-accent-glow));
}

.row-label {
  font-size: 13px;
  color: var(--ag-text-dim);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color var(--ag-speed) ease;
}

.profile-row:hover .row-label {
  color: var(--ag-text);
}

.row-right {
  flex-shrink: 0;
  margin-left: 8px;
  opacity: 0;
  transition: opacity var(--ag-speed) ease;
}

.profile-row:hover .row-right,
.profile-row.selected .row-right {
  opacity: 1;
}

/* ⋮ context button */
.row-right :deep(button) {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: var(--ag-surface);
  border: 1px solid var(--ag-divider) !important;
  border-radius: var(--ag-radius-sm);
  color: var(--ag-text-dim);
  cursor: pointer;
  font-size: 12px;
  transition: background-color var(--ag-speed) ease,
              color var(--ag-speed) ease,
              border-color var(--ag-speed) ease;
}

.row-right :deep(button:hover) {
  background: var(--ag-accent-dim);
  border-color: var(--ag-accent) !important;
  color: #fff;
}
</style>
