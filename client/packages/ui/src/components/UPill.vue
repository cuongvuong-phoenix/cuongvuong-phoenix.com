<template>
  <button
    v-if="!loading"
    v-bind="$attrs"
    type="button"
    :disabled="disabled"
    class="rounded-4xl inline-flex items-center justify-center space-x-1 border px-2.5 py-1.5 transition duration-300 disabled:opacity-50"
    :class="classes"
  >
    <UIcon v-if="icon" :icon="icon" />
    <span>{{ name }}</span>
  </button>

  <!-- "Skeleton" -->
  <div
    v-else
    v-bind="$attrs"
    class="border-fg-darkest rounded-4xl inline-flex animate-pulse items-center justify-center border px-2.5 py-1.5"
  >
    <USkeleton type="line" class="!rounded-4xl w-[8ch]" />
  </div>
  <!-- END "Skeleton" -->
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import UIcon from './UIcon.vue';
  import USkeleton from './USkeleton.vue';

  const { dim, active } = defineProps<{
    name?: string;
    icon?: string | null;
    active?: boolean;
    dim?: boolean;
    disabled?: boolean;
    loading?: boolean;
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
