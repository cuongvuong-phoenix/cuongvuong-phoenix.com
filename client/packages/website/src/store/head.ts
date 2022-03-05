import { defineStore } from 'pinia';

export interface HeadStoreState {
  title: string;
  ogTitle: string;
  description: string;
}

export const useHeadStore = defineStore('head', {
  state: () =>
    ({
      title: '',
      ogTitle: '',
      description: '',
    } as HeadStoreState),
});
