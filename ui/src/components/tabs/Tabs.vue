<template>
  <div class="tabs-root" :class="`variant-${variant}`" style="margin-left: 8px; margin-right: 8px; font-family: var(--ag-font);">
    <div class="tab" :class="{ centered }" role="TabList" :aria-label="this.label">
      <slot name="lead"></slot>
      <button v-for="tab in tabs" :key="tab.name"
        :class="{ active: tab.isActive }" v-show="!tab.hidden" @click="selectTab(tab)" role="tab"
        :aria-selected="tab.isActive" :tabindex="tab.isActive ? 0 : -1" @keydown="onTabKeydown" :ref="tab.name">
        {{ tab.name }}
      </button>
    </div>
    <div class="tabs-details" role="tabpanel" :aria-label="getActiveTab().name">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  emits: ["on-change"],
  name: "TabList",

  data() {
    return { tabs: [] };
  },
  props: {
    label: {
      type: String,
      required: false,
      default: "Tab list",
    },
    // "default" keeps the classic top-aligned underlined tabs (used by the
    // profiles sidebar). "pill" renders a modern segmented control.
    variant: {
      type: String,
      required: false,
      default: "default",
    },
    // Centre the tab strip horizontally (used by the main device tabs).
    centered: {
      type: Boolean,
      required: false,
      default: false,
    },
  },

  created() {
    window.addEventListener("keydown", this.onTabKeydownGlobal)
  },
  unmounted() {
    window.removeEventListener("keydown", this.onTabKeydownGlobal)
  },
  methods: {
    selectTab(selectedTab) {
      let activeTab = this.tabs.find((tab) => tab.isActive);
      this.tabs.forEach((tab) => {
        tab.isActive = tab.id === selectedTab.id;
      });
      let newActive = this.tabs.find((tab) => tab.isActive);

      if (activeTab !== newActive) {
        // Make sure we mount the tab before we call an update..
        this.$nextTick(() => this.$emit("on-change", selectedTab));
      }
    },

    // This function is generally for external calls, to set directly by id.
    selectTabById(id) {
      let tab = this.tabs.find((tab) => tab.id === id);
      this.selectTab(tab);
    },

    getActiveTab() {
      //return the active tab
      const activeTab = this.tabs.find((tab) => tab.isActive);
      if (activeTab) {
        return activeTab;
      } else {
        return "";
      }
    },
    //keyboard navigation
    onTabKeydown(event) {
      const tabs = this.tabs;
      const activeTab = this.getActiveTab();
      const activeTabIndex = tabs.indexOf(activeTab);
      let nextTab;
      switch (event.key) {
        case "ArrowRight":
        case "ArrowDown":
        case "PageDown":
          nextTab = tabs[(activeTabIndex + 1) % tabs.length];
          break;
        case "ArrowLeft":
        case "ArrowUp":
        case "PageUp":
          nextTab = tabs[(activeTabIndex - 1 + tabs.length) % tabs.length];
          break;
        case "Home":
          nextTab = tabs[0];
          break;
        case "End":
          nextTab = tabs[tabs.length - 1];
          break;
        default:
          break;
      }


      if (nextTab) {
        this.selectTab(nextTab);
        //nextTab.$el is the button element
        //we need a ref on the button element to focus it
        this.$refs[nextTab.name][0].focus();
      }
    },
    onTabKeydownGlobal(event) {
      if (this.label !== "Device Settings") return;
      const tabs = this.tabs;
      // const activeTab = this.getActiveTab();
      // const activeTabIndex = tabs.indexOf(activeTab);
      let nextTab;
      if (event.shiftKey && event.ctrlKey) {
        // Shift(Number) have different symbol between US keyboard and Other language.
        switch (event.code) {
          case "Digit1":
          case "Digit2":
          case "Digit3":
          case "Digit4":
          case "Digit5":
          case "Digit6":
          case "Digit7":
          case "Digit8":
            nextTab = tabs[Number(event.code[5]) - 1];
            break;
          default:
            break;
        }
      }

      if (nextTab) {
        this.selectTab(nextTab);
        //nextTab.$el is the button element
        //we need a ref on the button element to focus it
        this.$refs[nextTab.name][0].focus();
      }
    },
  },
  mounted() {
    this.$emit("on-change", this.getActiveTab());
  }
};
</script>

<style>
.tab {
  border-bottom: 1px solid var(--ag-accent);
  text-align: left;
}

.tab button {
  background-color: inherit;
  border: none;
  outline: none;
  cursor: pointer;
  padding: 7px 14px;
  margin-bottom: -1px;
  min-width: 108px;
  max-width: min-content;

  /*font-family: LeagueMonoVariable, sans-serif;*/
  border-radius: var(--ag-radius) var(--ag-radius) 0 0;
  color: var(--ag-text-dim);
  white-space: nowrap;
  transition: background-color var(--ag-speed) ease,
              color var(--ag-speed) ease;
}

.tab button:hover:not(.active) {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}

.tab button.active {
  border: 1px solid var(--ag-accent);
  border-bottom: 1px solid var(--ag-surface);
  color: var(--ag-text);

  text-shadow: 0 0 3px var(--ag-accent), 0 0 5px var(--ag-accent);
}

.tabs-details {
  border: 1px solid var(--ag-accent);
  border-top: 0;
  padding: 0;
  margin: 0;
  overflow: auto;
  vertical-align: middle;
}

/* ===== Pill / segmented variant (modern, centered) ===== */
.variant-pill .tab {
  border-bottom: none;
  display: inline-flex;
  gap: 2px;
  padding: 4px;
  background-color: var(--ag-surface);
  border: 1px solid var(--ag-divider);
  border-radius: 999px;
}

.variant-pill .tab.centered {
  display: flex;
  width: fit-content;
  margin: 0 auto;
}

.variant-pill .tab button {
  min-width: auto;
  margin-bottom: 0;
  padding: 6px 16px;
  border-radius: 999px;
  color: var(--ag-text-dim);
}

.variant-pill .tab button:hover:not(.active) {
  background-color: var(--ag-surface-2);
  color: var(--ag-text);
}

.variant-pill .tab button.active {
  border: none;
  background-color: var(--ag-accent);
  color: var(--ag-text);
  text-shadow: none;
  box-shadow: 0 0 12px var(--ag-accent-glow);
}

.variant-pill .tabs-details {
  border: none;
  margin-top: 16px;
}
</style>
