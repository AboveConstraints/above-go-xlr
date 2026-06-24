<template>
  <input ref="inputBox" class="text" :value="modelValue" type="text"
         @input="$emit('update:modelValue', $event.target.value)"
         v-on:keyup.enter="$emit('on-enter')"
         :placeholder="placeholder" />
</template>

<script>
export default {
  name: "ModalInput",

  props: {
    modelValue: String,
    placeholder: String,
  },

  methods: {
    focus() {
      this.$refs.inputBox.focus();
    },
  }
}
</script>

<style scoped>
input[type=text] {
  width: calc(100% - 10px);
  background-color: var(--ag-surface-2);
  border: 1px solid var(--ag-divider);
  padding: 5px;
  color: var(--ag-text);
}
</style>
