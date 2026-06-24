<template>
  <button class="expander" @click="handleClick">
    <font-awesome-icon v-if=expanded title="Collapse Section" :icon="'fa-solid ' + expand_left" />
    <font-awesome-icon v-else title="Expand Section" :icon="'fa-solid ' + expand_right" />
  </button>
</template>

<script>
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
export default {
  name: "ExpandoBox",
  components: {FontAwesomeIcon},
  props: {
    expanded: {type: Boolean, default: false },
    expand_right: { type: String, required: false, default: 'fa-chevron-right'},
    expand_left: { type: String, required: false, default: 'fa-chevron-left'}
  },

  methods: {
    expandText() {
      if (this.expanded) {
        return "<"
      }
      return ">"
    },
    handleClick() {
      this.$emit("expando-clicked");
    }
  }
}
</script>

<style scoped>
.expander {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  width: 32px;

  color: var(--ag-text-dim);
  vertical-align: middle;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-left: none;
  padding: 0;
  cursor: pointer;
  transition: background-color var(--ag-speed) ease, color var(--ag-speed) ease;
}

.expander:hover {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}
</style>

