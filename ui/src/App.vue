<template>
  <GoXLR/>
</template>

<script>
import GoXLR from "@/components/GoXLR.vue";

export default {
  name: 'App',
  components: {
    GoXLR,
  },
}
</script>

<style>
@import url("https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap");

/* ===== Above Constraints design tokens ===== */
:root {
  /* Surfaces (near-black navy) */
  --ag-bg: #0e1015;
  --ag-surface: #181a20;
  --ag-surface-2: hsl(225, 14%, 11%);
  --ag-divider: #262a33;

  /* Accent (crimson) */
  --ag-accent: hsl(343, 69%, 45%);
  --ag-accent-dim: hsl(343, 69%, 32%);
  --ag-accent-hover: rgb(227, 29, 85);
  --ag-accent-glow: rgba(227, 29, 85, 0.55);

  /* Text */
  --ag-text: #f2f4f8;
  --ag-text-dim: #8b93a3;

  /* Density scale (compact) */
  --ag-gap: 6px;
  --ag-gap-lg: 12px;
  --ag-radius: 6px;
  --ag-radius-sm: 4px;

  /* Motion */
  --ag-speed: 120ms;

  /* Brand fonts */
  --ag-font: "Inter", system-ui, sans-serif;
}

body {
  background-color: var(--ag-bg);
  background-image:
    /* Signature crimson radial glow at top */
    radial-gradient(
      ellipse 80% 40% at 50% -5%,
      rgba(163, 38, 41, 0.18) 0%,
      transparent 70%
    ),
    /* Subtle noise grain for texture */
    url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='200' height='200'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='200' height='200' filter='url(%23n)' opacity='0.03'/%3E%3C/svg%3E");
  background-repeat: no-repeat, repeat;
  background-size: 100% 100%, 200px 200px;
  padding: 0;
  margin: 0;
}

#app {
  font-family: var(--ag-font);

  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: var(--ag-text);
  margin-top: 0;
  user-select: none;
}

button {
  font-family: var(--ag-font);
}

/* ===== Above themed checkboxes ===== */
input[type="checkbox"] {
  appearance: none;
  -webkit-appearance: none;
  width: 15px;
  height: 15px;
  border: 1.5px solid var(--ag-divider);
  border-radius: 3px;
  background: var(--ag-surface-2);
  cursor: pointer;
  position: relative;
  flex-shrink: 0;
  transition: border-color var(--ag-speed) ease, background var(--ag-speed) ease, box-shadow var(--ag-speed) ease;
  vertical-align: middle;
}

input[type="checkbox"]:hover {
  border-color: var(--ag-accent-dim);
}

input[type="checkbox"]:checked {
  background: var(--ag-accent);
  border-color: var(--ag-accent);
  box-shadow: 0 0 8px var(--ag-accent-glow);
}

input[type="checkbox"]:checked::after {
  content: '';
  position: absolute;
  left: 3px;
  top: 0px;
  width: 5px;
  height: 9px;
  border: 2px solid #fff;
  border-top: none;
  border-left: none;
  transform: rotate(45deg);
}
</style>
