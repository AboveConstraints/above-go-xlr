<template>
  <div>
    <button ref="button" :aria-label="(label === undefined) ? text : label" :class="{ centered: centered, disabled: disabled }" @click="clicked"><slot>{{ text }}</slot></button>
  </div>
</template>

<script>
export default {
  name: "ButtonItem",

  props: {
    text: {type: String, required: true },
    label: { type: String, required: false },
    id: {type: String, required: true},

    background: {type: String, required: false, default: "var(--ag-divider)" },
    disabled: {type: Boolean, required: false, default: false},
    padding: {type: String, required: false, default: "8px"},
    centered: { type: Boolean, required: false, default: false }
  },

  methods: {
    focus() {
      this.$refs.button.focus()
    },

    clicked() {
      this.$emit('on-click', this.id);
    }
  }
}
</script>

<style scoped>
button {
  font-family: var(--ag-font);

  display: block;
  box-sizing: border-box;

  border: 0;

  width: calc(100% - 12px);
  margin: auto;
  background-color: v-bind(background);

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  /*
   We sneak the padding 4 pixels to the left, because we can't completely hide the radio button..
   */
  padding: v-bind(padding);
  text-align: left;
  color: var(--ag-text);
}

button:focus {
  background-color: var(--ag-divider);
  outline: none;
}

button:not(.disabled):hover {
  background-color: var(--ag-divider);
}

button.centered {
  text-align: center;
}

button.disabled {
  background-color: var(--ag-surface-2);
  color: var(--ag-text-dim);
}
</style>