<script>
export default {
  name: "GroupContainer",
  props: {
    label: String,
    title: String,
    level: { type: Number, default: 2 },
    role: { type: String, default: "region" },
    sidePadding: { type: String, required: false, default: "20px" }
  },
  // rightWidth kept for any callers that still depend on it (now unused internally)
  computed: {
    rightWidth() { return "0px"; }
  }
}
</script>

<template>
  <div class="container" :role="role" :aria-label="label || title || ''">
    <div class="card-header">
      <div v-if="title !== '' && title !== undefined"
           class="title" role="heading" :aria-level="level">
        <span class="title-accent">▌</span>{{ title }}
      </div>
      <div class="right">
        <slot name="right"></slot>
      </div>
    </div>
    <div class="content">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
* { margin: 0; padding: 0; }

.container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  background: linear-gradient(170deg, var(--ag-surface) 0%, var(--ag-surface-2) 100%);
  border: 1px solid var(--ag-divider);
  border-top: 2px solid var(--ag-accent);
  border-radius: 0 0 10px 10px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.35),
              inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.card-header {
  display: flex;
  align-items: center;
  padding: 9px 14px;
  background: rgba(14, 16, 21, 0.5);
  border-bottom: 1px solid var(--ag-divider);
  gap: 8px;
}

.title {
  flex: 1;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--ag-text-dim);
  text-align: center;
}

.title-accent {
  color: var(--ag-accent);
  margin-right: 5px;
  opacity: 0.85;
}

.right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.content {
  display: flex;
  flex-direction: row;
  gap: 8px;
  height: fit-content;
  width: fit-content;
  align-self: center;
  padding: 18px v-bind(sidePadding);
}
</style>
