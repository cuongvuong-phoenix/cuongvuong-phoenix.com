<template>
  <button
    type="button"
    class="px-2.5 py-1.5 space-x-1 inline-flex items-center justify-center transition duration-300 rounded-4xl border"
    :class="classes"
  >
    <UIcon v-if="icon" :icon="icon" />
    <span>{{ name }}</span>
  </button>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import UIcon from './UIcon.vue';

  const { dim, active } = defineProps<{
    name: string;
    icon?: string | null;
    active?: boolean;
    dim?: boolean;
  }>();

  /* ----------------------------------------------------------------
  Styles
  ---------------------------------------------------------------- */
  const classes = computed(() => {
    if (!active) {
      const base = 'border-current hover:text-primary-default';

      if (!dim) {
        return base;
      } else {
        return `${base} text-fg-darker`;
      }
    } else {
      return `border-transparent text-bg-default bg-primary-default hover:bg-primary-darker`;
    }
  });
</script>
