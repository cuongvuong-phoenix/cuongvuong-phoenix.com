import { defineStore } from 'pinia';

export const useUiStore = defineStore('ui', {
  state: () => ({
    // Header height in pixel.
    headerHeight: 96,
    // Header menu openning state.
    headerMenuOpenning: false,
  }),
  getters: {
    headerHeightString: (state) => {
      return `${state.headerHeight}px`;
    },
  },
  actions: {
    setHeaderHeight(value: number) {
      this.headerHeight = value;
    },
    toggleHeaderMenuOpenning(value?: boolean) {
      if (typeof value === 'boolean') {
        this.headerMenuOpenning = value;
      } else {
        this.headerMenuOpenning = !this.headerMenuOpenning;
      }
    },
  },
});
