<script>
export default {
  name: "ContentContainer",
  props: {
    noLeftPad: Boolean,
  }
}
</script>

<template>
  <div class="container" :class="{ contentPadNoLeft: noLeftPad }">
    <slot></slot>
  </div>
</template>

<style scoped>
* {
  margin: 0;
  padding: 0;
}

.container {
  display: flex;
  flex-direction: row;
  gap: 12px;

  padding: 20px 24px;

  overflow-x: auto;
  overflow-y: hidden;
}

.container::-webkit-scrollbar {
  height: 6px;
  width: 6px;
}

.container::-webkit-scrollbar-track {
  background-color: transparent;
}

.container::-webkit-scrollbar-thumb {
  background-color: var(--ag-text);
  border-radius: 3px;
}

.contentPadNoLeft {
  padding-left: 0;
}
</style>
