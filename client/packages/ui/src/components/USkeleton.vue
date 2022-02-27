<template>
  <!-- "Rect" -->
  <div v-if="type === 'rect'" v-bind="$attrs" class="rounded-lg bg-bg-lighter animate-pulse"></div>
  <!-- END "Rect" -->

  <!-- "Circle" -->
  <div
    v-else-if="type === 'circle'"
    v-bind="$attrs"
    class="inline-block rounded-full bg-bg-lighter animate-pulse"
  ></div>
  <!-- END "Circle" -->

  <!-- "Lines" -->
  <div v-else v-bind="$attrs" class="animate-pulse">
    <div
      v-for="num in numLines"
      :key="num"
      class="h-[1em] bg-bg-lighter rounded inline-block align-middle w-full"
      :class="{
        'w-3/4': numLines > 1 && num === numLines,
      }"
    ></div>
  </div>
  <!-- END "Lines" -->
</template>

<script lang="ts">
  export const skeletonTypes = ['rect', 'circle', 'lines'] as const;
  export type SkeletonType = typeof skeletonTypes[number];
</script>

<script setup lang="ts">
  const { type = skeletonTypes[0], numLines = 1 } = defineProps<{
    type?: SkeletonType;
    numLines?: number;
  }>();
</script>
