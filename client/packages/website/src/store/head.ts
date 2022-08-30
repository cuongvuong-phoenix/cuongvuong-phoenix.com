import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useHeadStore = defineStore('head', () => {
  const title = ref('');
  const ogTitle = ref('');
  const description = ref('');

  return { title, ogTitle, description };
});
