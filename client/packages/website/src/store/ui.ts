import { defineStore } from 'pinia';

export const useUiStore = defineStore('ui', {
  state: () => ({
    // Header height in pixel.
    headerHeight: 96,
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
  },
});
