<template>
  <div v-show=is_visible class="modal-mask">
    <div class="modal-wrapper">
      <div ref="dialog" class="modal-container" role="dialog" aria-modal="true" :aria-labelledby="`${id}_label`"
           :aria-describedby="`${id}_body`" @keyup.esc.prevent="closeModalEsc">
        <div class="modal-header" tabindex="0">
          <div :id="`${id}_label`" role="heading" aria-level="2" >
            <slot name="title" ref="title"></slot>
          </div>
          <button v-show=show_close ref="close" @click="closeModal()">
            <font-awesome-icon title="Close" icon="fa-solid fa-xmark"/>
          </button>
        </div>
        <div class="modal-body" :id="`${id}_body`">
          <slot></slot>
        </div>
        <div v-if="show_footer" class="modal-footer">
          <slot name="footer">
            <button ref="ok" class="modal-default-button" @click="closeModal()">{{ $t('message.modalButtons.ok') }}</button>
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import * as focusTrap from "focus-trap";

export default {
  name: "AccessibleModal",
  emits: ["modal-close"],

  props: {
    id: {type: String, required: true},
    show_close: {type: Boolean, default: true},
    show_footer: {type: Boolean, default: true},
    prevent_esc: {type: Boolean, default: false},

    bodyPadding: {type: String, default: "20px"},
    width: {type: String, default: "500px"}
  },

  data() {
    return {
      is_visible: false,
      returnFocus: undefined,
      trap: undefined,
    }
  },

  methods: {
    openModal(focusRef, returnFocus) {
      this.returnFocus = returnFocus;
      this.is_visible = true;

      this.$nextTick(() => {
        if (focusRef === undefined) {
          if (this.$refs.ok !== undefined) {
            this.$refs.ok.focus();
          } else {
            this.$refs.close.focus();
          }
        } else {
          focusRef.focus();
        }

        // Create the focus trap, to prevent moving out..
        this.trap = focusTrap.createFocusTrap(this.$refs.dialog);
        this.trap.activate();
      })
    },

    closeModalEsc() {
      if (this.prevent_esc) {
        return;
      }
      this.closeModal()
    },

    closeModal() {
      // Deactivate the Trap (if active)..
      this.trap.deactivate();

      // Hide the UI..
      this.is_visible = false;

      // Return focus to the requested element.
      if (this.returnFocus !== undefined) {
        this.returnFocus.focus();
      }

      this.$emit('modal-close');
    },

    isOpen() {
      return this.is_visible;
    }
  }
}


</script>

<style scoped>
/* Turn the background of the screen grey */
.modal-mask {
  position: fixed;
  z-index: 9998;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.2);
  display: table;
  transition: opacity 0.3s ease;
}

/* Positions the Modal in the Middle of the Screen */
.modal-wrapper {
  display: table-cell;
  vertical-align: middle;
}

/* The Actual Border / Setup of the Modal */
.modal-container {
  min-width: v-bind(width);
  max-width: min-content;
  margin: 0 auto;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius);
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.55),
              0 0 60px rgba(163, 38, 41, 0.06);
  transition: all 0.2s ease;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px;
  background-color: var(--ag-surface);
  border-bottom: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius) var(--ag-radius) 0 0;
  font-family: var(--ag-font);
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ag-text);
  outline: none;
}

.modal-header div {
  padding: 0;
  float: none;
}

.modal-header button {
  padding: 4px 8px;
  cursor: pointer;
  background-color: transparent;
  color: var(--ag-text-dim);
  border: none;
  border-radius: var(--ag-radius-sm);
  transition: color var(--ag-speed) ease, background-color var(--ag-speed) ease;
}

.modal-header button:hover {
  color: var(--ag-text);
  background-color: var(--ag-surface-2);
}

.modal-body {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
  padding: v-bind(bodyPadding);
  border-radius: 0 0 var(--ag-radius) var(--ag-radius);
}

.modal-footer {
  background-color: var(--ag-surface-2);
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 10px 16px;
  border-top: 1px solid var(--ag-divider);
  border-radius: 0 0 var(--ag-radius) var(--ag-radius);
}

.modal-footer button {
  background-color: transparent;
  color: var(--ag-text-dim);
  padding: 6px 20px;
  border: 1px solid var(--ag-divider);
  border-radius: var(--ag-radius-sm);
  font-family: var(--ag-font);
  cursor: pointer;
  transition: color var(--ag-speed) ease, border-color var(--ag-speed) ease;
}

.modal-footer button:hover {
  color: var(--ag-text);
  border-color: var(--ag-accent);
}

.modal-mask {
  background-color: rgba(0, 0, 0, 0.55);
}
</style>
