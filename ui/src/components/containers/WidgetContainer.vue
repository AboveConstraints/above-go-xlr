<script>
export default {
  name: "WidgetContainer",
  props: {
    title: String,
    maxWidth: { type: String, default: "fit-content" }
  }
}
</script>

<template>
  <div class="container">
    <div class="card-header" v-if="title || $slots.title">
      <p class="title" role="heading" aria-level="3">
        <slot name="title">{{ title }}</slot>
      </p>
    </div>
    <div class="body">
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
* { margin: 0; padding: 0; }

.container {
  display: flex;
  flex-direction: column;
  min-width: 200px;
  max-width: v-bind(maxWidth);
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
  justify-content: center;
  padding: 9px 14px;
  background: rgba(14, 16, 21, 0.5);
  border-bottom: 1px solid var(--ag-divider);
}

.title {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--ag-text-dim);
  text-align: center;
}

.body {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
}
</style>
