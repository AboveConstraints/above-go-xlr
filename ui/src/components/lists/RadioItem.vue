<template>
  <div class="wrapper">
    <input ref="check" :name=group :id=id type="radio" @change="change" :value=id :checked="selected"
           :disabled="disabled" :aria-labelledby="`label_${id}`" class="screenreader-only"/>
    <label ref="label" :id="`label_${id}`" :for="id" :class="{ selected: this.selected, disabled: this.disabled }">
      <font-awesome-icon v-if="icon !== undefined" :icon="icon"/>
      {{ text }}
    </label>
    <div ref="right_ref" class="right_side" :class="{ selected: this.selected, disabled: this.disabled }">
      <slot name="right"></slot>
    </div>
  </div>
</template>

<script>
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";

export default {
  components: {FontAwesomeIcon},
  emits: ["radio-selected"],
  name: "RadioItem",

  props: {
    text: {type: String, required: true},
    icon: {type: String, required: false},
    id: {type: String, required: true},
    group: {type: String, required: true},

    selected: {type: Boolean, required: false, default: false},
    disabled: {type: Boolean, required: false, default: false},

    padding: {type: String, required: false, default: "8px"},
    background: {type: String, required: false, default: "var(--ag-divider)" },
  },

  data: function () {
    return {
      local_selected: false,
    }
  },

  methods: {
    change() {
      this.$emit('radio-selected', this.id);
    },

    isSelected() {
      if (this.$refs.check === undefined) {
        return false;
      }
      return this.selected;
    },

    focus() {
      this.$refs.label.focus();
    }
  },

  computed: {
    right_width: function () {
      if (this.$refs.right_ref.clientWidth === 0) {
        return "0px";
      }

      return (this.$refs.right_ref.clientWidth - 12) + "px";
    },
  }
}
</script>

<style scoped>
.wrapper {
  display: flex;
  flex-direction: row;
}

/* We're going to try and make the label look and behave like a legacy 'Button' */
label {
  display: block;
  box-sizing: border-box;

  width: calc(100% - v-bind(right_width));
  margin: auto 0 auto auto;
  background-color: v-bind(background);

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  padding: v-bind(padding);
  text-align: left;
  color: var(--ag-text);
}

.right_side {
  background-color: var(--ag-divider);
  margin-right: auto;
  color: var(--ag-text);
}

:slotted(.right_side > *) {
  color: var(--ag-text);
}

:slotted(.right_side.selected > *) {
  color: var(--ag-text);
}

.right_side.selected {
  background-color: var(--ag-accent);
  color: var(--ag-text);
}

label.selected {
  background-color: var(--ag-accent);
  color: var(--ag-text);
}

label:not(.selected):focus-within {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}

label:not(.selected):hover, label:not(.selected):hover + div {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}

label.disabled, label.disabled + div {
  background-color: var(--ag-surface);
  color: var(--ag-text-dim);
  cursor: not-allowed;
}

</style>
