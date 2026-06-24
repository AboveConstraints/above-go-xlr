<template>
  <div ref="selectorList" class="selectorList">
    <slot/>
  </div>
</template>

<script>
export default {
  name: "ProfileButtonList"
}
</script>

<style scoped>
.selectorList {
  height: 100%;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.selectorList::-webkit-scrollbar {
  width: 3px;
}

.selectorList::-webkit-scrollbar-track {
  background-color: transparent;
}

.selectorList::-webkit-scrollbar-thumb {
  background-color: var(--ag-divider);
  border-radius: 3px;
}
</style>
