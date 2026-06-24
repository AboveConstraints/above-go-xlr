<template>
  <div>
    <!--    <div v-if="!editable" class="sliderInput">-->
    <!--      <input type="text" v-on:blur="reset" :value="displayValue()" :min="minValue" :max="maxValue"-->
    <!--             :disabled="!editable" :aria-label="title" :aria-description="title" :aria-valuetext="getDisplayValue()"/>-->
    <!--      <div class="suffix"><span class="filler">{{ displayValue() }}</span><span v-html="getSuffix()"></span></div>-->
    <!--    </div>-->
    <div class="sliderInput" :class="{ 'has-unit': !!textSuffix }">
      <input type="number" v-on:input="update" v-on:focus="focus" v-on:blur="reset" v-model="localTextValue"
             :min="minValue"
             :max="maxValue" :aria-label="title" :aria-description="title"
             :aria-valuetext="getDisplayValue()" :disabled="disabled"/>
      <span v-if="textSuffix" class="unit" aria-hidden="true">{{ textSuffix }}</span>
    </div>
  </div>
</template>

<script>
export default {
  name: "TextInput",
  emits: ["value-updated"],

  data() {
    return {
      localTextValue: 0,
      lastTextValue: 0,

      focused: false,
    }
  },

  props: {
    id: {type: String, required: false, default: ""},
    editable: Boolean,
    currentTextValue: [Number, String],
    allowFloat: {type: Boolean, default: false},

    // Handlers for ValueMap..
    currentFieldValue: Number,
    valueMap: Array,

    // Handlers for Non ValueMaps..
    minValue: {type: Number, default: 0},
    maxValue: {type: Number, default: 100},

    // Display Related Settings
    textSuffix: {type: String, default: ""},
    colour: {type: String, required: false, default: 'var(--ag-accent)'},
    backgroundColour: {type: String, required: false, default: 'var(--ag-surface-2)'},
    title: {type: String, required: false, default: ''},

    disabled: {type: Boolean, required: false, default: false},
  },

  methods: {

    getSuffix() {
      let output = "";
      for (let i = 0; i < this.textSuffix.length; i++) {
        output += "&nbsp;";
      }
      return output + this.textSuffix;
    },

    getDisplayValue() {
      return this.localTextValue + this.textSuffix;
    },

    update(e) {
      let newValue = e.target.value;

      if (newValue === "-" || newValue === "") {
        // Cleared box, or starting negative value..
        return;
      }

      if (this.valueMap !== undefined) {
        // We need to find the closest index which matches this value...
        let base = undefined;
        for (let i = 0; i < this.valueMap.length; i++) {
          if (this.valueMap[i] >= newValue) {
            // Ok, it's between this value and the previous..
            base = i;
            break;
          }
        }

        let result = 0;
        if (base === undefined) {
          // We got to the end of the loop, and this value was higher!
          result = this.valueMap.length - 1;
        } else if (base === 0) {
          // The first value higher was at the base of the list, do nothing.
          result = 0;
        } else if (this.valueMap[base] === newValue) {
          result = base;
        } else {
          let lower = this.valueMap[base - 1];
          let upper = this.valueMap[base];

          // Calculate which value this is nearest to..
          let middle = (upper - lower) / 2;
          let ours = newValue - lower;

          result = (ours < middle) ? base - 1 : base;
        }
        this.$emit("value-updated", result, this.id);
        return;
      }

      if (e.target.value > this.maxValue || e.target.value < this.minValue) {
        // We're outside the range of this input, don't trigger an event until either
        // blur, or we're inside.
        return;

      }

      // Value has changed, emit something upwards..
      let value = (this.allowFloat) ? parseFloat(newValue) : parseInt(newValue);
      this.$emit("value-updated", value, this.id);
      this.$emit("blur");
    },

    focus() {
      this.focused = true;
    },

    reset(e) {
      this.focused = false;

      let newValue = e.target.value;
      if (!this.isNumber(newValue)) {
        this.localTextValue = this.lastTextValue;
        return;
      }

      if (e.target.value < this.minValue) {
        this.localTextValue = this.minValue;
        this.$emit("value-updated", this.minValue, this.id);
        return;
      }

      if (e.target.value > this.maxValue) {
        this.localTextValue = this.maxValue;
        this.$emit("value-updated", this.maxValue, this.id);
      }

      this.$emit("blur");
    },

    isNumber(str) {
      // This isn't perfect, but will catch *MOST* cases where values aren't numbers..
      if (typeof str != "string") {
        return false;
      }
      return !isNaN(str) && !isNaN(parseFloat(str));
    },

    displayValue() {
      if (this.valueMap !== undefined) {
        return this.valueMap[this.currentFieldValue];
      }
      return this.localTextValue;
    },
  },

  watch: {
    currentFieldValue: function () {
      if (this.focused) {
        return;
      }

      if (this.valueMap !== undefined) {
        this.localTextValue = this.displayValue();
      }
    },

    currentTextValue: function (newValue) {
      if (this.focused) {
        return;
      }

      if (this.valueMap === undefined) {
        this.localTextValue = newValue;
        this.lastTextValue = newValue;
      } else {
        this.localTextValue = this.displayValue();
      }
    }
  },
}
</script>

<style scoped>
.sliderInput {
  position: relative;
  display: flex;
  align-items: center;
}

.sliderInput input[type=number], .sliderInput input[type=text] {
  font-family: var(--ag-font);
  font-weight: 700;
  font-size: 13px;
  font-variant-numeric: tabular-nums;

  background-color: v-bind(backgroundColour);
  color: v-bind(colour);
  padding: 6px 8px;
  box-sizing: border-box;

  text-align: center;
  width: 100%;

  border: 1px solid transparent;
  border-radius: var(--ag-radius-sm);
  background-image: none;
  box-shadow: none;
  outline: none;

  transition: border-color var(--ag-speed) ease, background-color var(--ag-speed) ease;

  -moz-appearance: textfield;
}

/* Leave room on the right for the unit so it never overlaps the value */
.sliderInput.has-unit input[type=number],
.sliderInput.has-unit input[type=text] {
  padding-right: 26px;
}

.sliderInput input:hover {
  background-color: var(--ag-surface);
}

.sliderInput input:focus {
  border-color: var(--ag-accent);
}

.sliderInput input:disabled {
  color: var(--ag-text-dim);
}

/* Unit pinned to the right, dimmed — no more overlap with the value */
.sliderInput .unit {
  position: absolute;
  right: 9px;
  top: 50%;
  transform: translateY(-50%);
  pointer-events: none;
  user-select: none;
  color: var(--ag-text-dim);
  font-size: 0.8em;
  font-weight: 500;
}

.sliderInput input[type=number]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
