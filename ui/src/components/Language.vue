<script>
export default {
  name: "Language",
  methods: {
    // Turn "it_IT" / "de-DE" into a short "IT" / "DE" tag.
    shortLocale(locale) {
      return locale.split(/[_-]/)[0].toUpperCase();
    },
  },
}
</script>

<template>
  <div class="lang-select">
    <span class="lang-icon" aria-hidden="true">🌐</span>
    <select v-model="$i18n.locale" :aria-label="'Language'">
      <option v-for="locale in $i18n.availableLocales" :key="`locale-${locale}`" :value="locale">
        {{ shortLocale(locale) }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.lang-select {
  position: fixed;
  bottom: 16px;
  right: 16px;
  z-index: 30;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: 999px;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4);
  font-family: var(--ag-font);
}

.lang-icon {
  font-size: 14px;
  line-height: 1;
  opacity: 0.8;
}

.lang-select select {
  appearance: none;
  -webkit-appearance: none;
  border: none;
  outline: none;
  background-color: transparent;
  color: var(--ag-text);
  font-family: var(--ag-font);
  font-size: 13px;
  letter-spacing: 0.5px;
  padding: 2px 18px 2px 2px;
  cursor: pointer;

  /* Custom crimson chevron */
  background-image: linear-gradient(45deg, transparent 50%, var(--ag-accent) 50%),
                    linear-gradient(135deg, var(--ag-accent) 50%, transparent 50%);
  background-position: right 6px center, right 1px center;
  background-size: 5px 5px, 5px 5px;
  background-repeat: no-repeat;
}

.lang-select select:focus-visible {
  color: var(--ag-accent);
}

.lang-select select option {
  background-color: var(--ag-surface);
  color: var(--ag-text);
}
</style>
