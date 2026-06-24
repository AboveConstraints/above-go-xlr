<template>
  <td>
    <div @click="clicked" role="checkbox" :aria-valuenow="enabled" :aria-label="`Routing from ${input} to ${output}`"
         :aria-checked="enabled" tabindex="0" :class="{ disabled: cellDisabled }" :aria-disabled="cellDisabled">
      <font-awesome-icon v-if="enabled" icon="fa-solid fa-circle-check"/>
    </div>
  </td>
</template>

<script>
export default {
  name: "RoutingCell",

  props: {
    input: String,
    output: String,
    enabled: Boolean,
    orange: {type: Boolean, required: false, default: false},
    cellDisabled: {type: Boolean, required: false, default: false}
  },

  computed: {
    textColour() {
      if (this.orange) {
        return "#CC7224";
      }
      return "var(--ag-accent)";
    }
  },

  methods: {
    clicked() {
      if (!this.cellDisabled) {
        this.$emit('clicked', this.output, this.input);
      }
    }
  }
}
</script>

<style scoped>
td {
  padding: 0;
}

div {
  text-align: center;
  font-size: 16px;
  color: v-bind(textColour);
  background-color: var(--ag-surface);
  margin: 0;
  position: relative;
  height: 26px;
  cursor: pointer;
  transition: background-color var(--ag-speed) ease;
}

div.disabled {
  background-color: var(--ag-surface-2);
  cursor: default;
}

div.disabled:hover {
  background-color: var(--ag-surface-2);
}

div svg {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  left: 0;
  right: 0;
  margin: auto;
  text-align: center;
}

div:has(>input[type=checkbox]:focus) {
  background-color: var(--ag-surface-2);
}

div:not(.disabled):hover {
  background-color: var(--ag-divider);
}
</style>
