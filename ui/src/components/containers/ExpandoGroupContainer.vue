<script>
import ExpandoBox from "@/components/design/ExpandoBox.vue";
import GroupContainer from "@/components/containers/GroupContainer.vue";

export default {
  name: "ExpandoGroupContainer",
  components: { GroupContainer, ExpandoBox },
  emits: ['expando-clicked'],
  props: {
    title: String,
    expanded: Boolean,
    // 'expand'  → chevron button on the side (default, used by Mixer outputs)
    // 'toggle'  → Simple / Advanced pill in the card header (used by mic sections)
    variant: { type: String, default: 'expand' },
  }
}
</script>

<template>
  <!-- ── Toggle variant: Simple / Advanced segmented pill in the header ── -->
  <div v-if="variant === 'toggle'" class="toggle-card">
    <div class="toggle-header">
      <span class="toggle-title">
        <span class="toggle-title-accent">▌</span>{{ title }}
      </span>

      <div class="toggle-right"><slot name="right"></slot></div>

      <button class="seg-pill" @click="$emit('expando-clicked')"
              :aria-pressed="expanded" :aria-label="`Toggle ${title} mode`">
        <span class="seg-opt" :class="{ active: !expanded }">Simple</span>
        <span class="seg-opt" :class="{ active: expanded }">Advanced</span>
      </button>
    </div>

    <div class="toggle-body">
      <slot></slot>
    </div>
  </div>

  <!-- ── Expand variant: chevron on the right (Mixer outputs etc.) ── -->
  <div v-else class="expando">
    <GroupContainer :title="title">
      <template #right><slot name="right"></slot></template>
      <slot></slot>
    </GroupContainer>
    <ExpandoBox @expando-clicked="$emit('expando-clicked')" :expanded="expanded"/>
  </div>
</template>

<style scoped>
/* ── Toggle card ── */
.toggle-card {
  display: flex;
  flex-direction: column;
  background: linear-gradient(170deg, var(--ag-surface) 0%, var(--ag-surface-2) 100%);
  border: 1px solid var(--ag-divider);
  border-top: 2px solid var(--ag-accent);
  border-radius: 0 0 10px 10px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.35),
              inset 0 1px 0 rgba(255, 255, 255, 0.03);
}

.toggle-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 9px 14px;
  background: rgba(14, 16, 21, 0.5);
  border-bottom: 1px solid var(--ag-divider);
}

.toggle-title {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--ag-text-dim);
  flex: 1;
}

.toggle-title-accent {
  color: var(--ag-accent);
  margin-right: 6px;
  font-size: 12px;
  opacity: 0.8;
}

.toggle-right {
  display: flex;
  align-items: center;
}

.toggle-body {
  display: flex;
  flex-direction: row;
  gap: 8px;
  padding: 18px 20px;
  align-self: center;
  width: fit-content;
}

/* ── Segmented pill toggle ── */
.seg-pill {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 3px;
  background: var(--ag-bg);
  border: 1px solid var(--ag-divider);
  border-radius: 6px;
  cursor: pointer;
  flex-shrink: 0;
}

.seg-opt {
  display: inline-block;
  padding: 3px 10px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.06em;
  color: var(--ag-text-dim);
  transition: background-color 120ms ease, color 120ms ease,
              box-shadow 120ms ease;
  white-space: nowrap;
}

.seg-opt.active {
  background: var(--ag-accent);
  color: #fff;
  box-shadow: 0 0 8px var(--ag-accent-glow);
}

/* ── Expand variant ── */
.expando {
  display: flex;
  flex-direction: row;
}
</style>
