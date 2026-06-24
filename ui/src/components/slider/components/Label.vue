<template>
  <div class="labelText">
    {{ title }}
  </div>
</template>

<script>
export default {
  name: "SliderLabel",

  props: {
    title: String,
    textColour: { type: String, required: false, default: "var(--ag-text)" },
  }
}
</script>

<style scoped>
.labelText {
  padding: 0 4px 10px;
  color: v-bind(textColour);

  font-size: 9px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  text-align: center;
  white-space: nowrap;
  opacity: 0.75;
}
</style>
