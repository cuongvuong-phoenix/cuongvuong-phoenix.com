import { defineStore } from 'pinia';

export const useUiStore = defineStore('ui', {
  state: () => ({
    headerFloat: false,
  }),
  actions: {
    setHeaderFloat(value: boolean) {
      this.headerFloat = value;
    },
  },
});
