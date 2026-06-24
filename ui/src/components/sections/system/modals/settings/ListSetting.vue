<script>
export default {
  emits: ['change'],

  name: "ListSetting",
  props: {
    label: {type: String, required: true},
    description: {type: String, required: true},
    options: {type: Array, required: true},
    value: {type: String, required: true},
  },

  data() {
    return {
      is_open: false,
    };
  },

  computed: {
    selectedLabel() {
      let match = this.options.find((option) => option.key === this.value);
      return match ? match.value : this.value;
    },
  },

  methods: {
    toggle() {
      this.is_open = !this.is_open;
    },

    close() {
      this.is_open = false;
    },

    selectOption(option) {
      this.close();
      if (option.key !== this.value) {
        this.$emit('change', option.key);
      }
    },

    onKeydown(e) {
      switch (e.key) {
        case 'Escape':
          this.close();
          break;
        case 'Enter':
        case ' ':
          e.preventDefault();
          this.toggle();
          break;
        case 'ArrowDown':
        case 'ArrowUp': {
          e.preventDefault();
          let index = this.options.findIndex((o) => o.key === this.value);
          let next = e.key === 'ArrowDown' ? index + 1 : index - 1;
          if (next >= 0 && next < this.options.length) {
            this.selectOption(this.options[next]);
          }
          break;
        }
      }
    },
  }
}
</script>

<template>
  <div class="setting">
    <span class="label">{{ label }}</span>
    <div class="input" v-click-outside="close">
      <button type="button" class="trigger" :aria-label="label" aria-haspopup="listbox" :aria-expanded="is_open"
              @click.stop="toggle" @keydown="onKeydown">
        <span class="value">{{ selectedLabel }}</span>
        <span class="caret" :class="{ open: is_open }" aria-hidden="true"></span>
      </button>
      <ul v-show="is_open" class="dropdown" role="listbox">
        <li v-for="option of options" :key="option.key" role="option" :aria-selected="option.key === value"
            class="item" :class="{ selected: option.key === value }" @click.stop="selectOption(option)">
          {{ option.value }}
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.setting {
  display: flex;
  height: 20px;
  padding: 10px;
  color: var(--ag-text-dim);
}

.setting:focus-within {
  color: var(--ag-text);
}

.label {
  margin: auto;
  width: 100%;
}

.input {
  position: relative;
  max-width: fit-content;
  margin: auto;
}

/* Closed state: keep the previous minimal, transparent look. */
.trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  border: 0;
  background-color: transparent;
  font-family: var(--ag-font);
  font-size: 20px;
  color: var(--ag-text-dim);
  cursor: pointer;
  padding: 0;
}

.trigger:hover,
.trigger:focus {
  color: var(--ag-text);
  outline: none;
}

.trigger .value {
  text-align: right;
}

/* CSS caret so we don't depend on an icon set. */
.caret {
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 6px solid currentColor;
  transition: transform var(--ag-speed) ease;
}

.caret.open {
  transform: rotate(180deg);
}

/* Open list: themed to match the rest of the UI (mirrors DropMenu). */
.dropdown {
  position: absolute;
  right: 0;
  top: calc(100% + 4px);
  min-width: 100%;
  margin: 0;
  padding: 4px 0;
  list-style: none;
  background-color: var(--ag-surface);
  color: var(--ag-text);
  border: 1px solid var(--ag-text-dim);
  border-radius: 4px;
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.45);
  z-index: 1000000;
  max-height: 280px;
  overflow-y: auto;
}

.dropdown .item {
  padding: 6px 15px;
  font-size: 16px;
  white-space: nowrap;
  cursor: pointer;
  color: var(--ag-text);
}

.dropdown .item.selected {
  background-color: var(--ag-surface-2);
}

.dropdown .item:hover {
  background-color: var(--ag-accent);
  color: white;
}
</style>
