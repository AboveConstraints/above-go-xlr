<template>
  <button ref="button" role="button" :aria-labelledby="`button_title_${id}`" class="content" @click="clicked">
    <div class="img-section">
      <slot/>
    </div>
    <div class="iconTitle"><span :id="`button_title_${id}`" class="filler">{{ title }}</span></div>
  </button>
</template>

<script>
export default {
  name: "BigButton",

  props: {
    id: { type: String, required: true },
    title: String,
  },

  methods: {
    clicked: function (eventData){
      this.$emit("button-clicked", eventData)
    },
    focus: function() {
      this.$refs.button.focus();
    }
  }
}
</script>

<style scoped>
button.content {
  min-width: 110px;
  max-width: min-content;
  height: 120px;
  background: var(--ag-surface-2);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  margin: 0;
  padding: 0;
  cursor: pointer;
  transition: border-color var(--ag-speed) ease,
              background-color var(--ag-speed) ease,
              box-shadow var(--ag-speed) ease;
}

button.content:hover {
  border-color: var(--ag-accent-dim);
  background-color: var(--ag-surface);
  box-shadow: 0 0 14px var(--ag-accent-glow);
}

button .img-section {
  text-align: center;
  height: 78px;
  font-size: 36px;
  color: var(--ag-text-dim);
  display: flex;
  justify-content: center;
  align-items: center;
  transition: color var(--ag-speed) ease;
}

button.content:hover .img-section {
  color: var(--ag-text);
}

button .iconTitle {
  height: 38px;
  line-height: 38px;
  background-color: transparent;
  color: var(--ag-text-dim);
  font-family: var(--ag-font);
  font-size: 12px;
  text-align: center;
  white-space: nowrap;
  padding: 0 8px;
  border-top: 1px solid var(--ag-divider);
  transition: color var(--ag-speed) ease;
}

button.content:hover .iconTitle {
  color: var(--ag-text);
}
</style>
