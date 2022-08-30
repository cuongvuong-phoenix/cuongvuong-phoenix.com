import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

export enum HeaderHeight {
  DEFAULT = 96,
  NARROW = 64,
}

export const useUiStore = defineStore('ui', () => {
  /* ----------------------------------------------------------------
  Header Height
  ---------------------------------------------------------------- */
  const headerHeight = ref(HeaderHeight.DEFAULT);
  const headerHeightString = computed(() => `${headerHeight.value}px`);

  /* ----------------------------------------------------------------
  Header Menu Opening
  ---------------------------------------------------------------- */
  const headerMenuOpenning = ref(false);

  function toggleHeaderMenuOpenning(value?: boolean) {
    if (typeof value === 'boolean') {
      headerMenuOpenning.value = value;
    } else {
      headerMenuOpenning.value = !headerMenuOpenning.value;
    }
  }

  return { headerHeight, headerHeightString, headerMenuOpenning, toggleHeaderMenuOpenning };
});
