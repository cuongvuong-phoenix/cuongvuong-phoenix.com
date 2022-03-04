<template>
  <!-- "Rect" -->
  <div v-if="type === 'rect'" v-bind="$attrs" class="rounded-lg bg-bg-lightest animate-pulse"></div>
  <!-- END "Rect" -->

  <!-- "Circle" -->
  <div
    v-else-if="type === 'circle'"
    v-bind="$attrs"
    class="inline-block rounded-full bg-bg-lightest animate-pulse"
  ></div>
  <!-- END "Circle" -->

  <!-- "Line" -->
  <div
    v-else-if="type === 'line'"
    v-bind="$attrs"
    class="min-w-[2ch] bg-bg-lightest rounded inline-block align-top animate-pulse"
    :class="fontSizeClasses"
  ></div>
  <!-- END "Line" -->

  <!-- "Paragraph" -->
  <div v-else v-bind="$attrs" class="animate-pulse">
    <div
      v-for="num in numLines"
      :key="num"
      class="h-[1em] bg-bg-lightest rounded inline-block"
      :style="{
        width: numLines > 1 && num === numLines ? lastLineStyle : '100%',
      }"
    ></div>
  </div>
  <!-- END "Paragraph" -->
</template>

<script lang="ts">
  import { computed } from 'vue';

  export const skeletonTypes = ['rect', 'circle', 'line', 'paragraph'] as const;
  export type SkeletonType = typeof skeletonTypes[number];

  export const skeletonFontSizes = [
    'base',
    'xs',
    'sm',
    'lg',
    'xl',
    '2xl',
    '3xl',
    '4xl',
    '5xl',
    '6xl',
    '7xl',
    '8xl',
  ] as const;
  export type SkeletonFontSize = typeof skeletonFontSizes[number];
</script>

<script setup lang="ts">
  const {
    type = skeletonTypes[0],
    fontSize = skeletonFontSizes[0],
    numLines = 2,
  } = defineProps<{
    type?: SkeletonType;
    fontSize?: SkeletonFontSize;
    numLines?: number;
  }>();

  /* ----------------------------------------------------------------
  Styles
  ---------------------------------------------------------------- */
  const fontSizeClasses = computed(() => {
    switch (fontSize) {
      case 'xs':
        return 'h-4 text-xs';
      case 'sm':
        return 'h-5 text-sm';
      case 'base':
        return 'h-6 text-base';
      case 'lg':
        return 'h-7 text-lg';
      case 'xl':
        return 'h-7 text-xl';
      case '2xl':
        return 'h-8 text-2xl';
      case '3xl':
        return 'h-9 text-3xl';
      case '4xl':
        return 'h-10 text-4xl';
      case '5xl':
        return 'h-12 text-5xl';
      case '6xl':
        return 'h-[3.75rem] text-6xl';
      case '7xl':
        return 'h-[4.5rem] text-7xl';
      case '8xl':
        return 'h-24 text-8xl';
      default:
        return undefined;
    }
  });

  const lastLineStyle = `${Math.floor(Math.random() * (75 - 25 + 1) + 25)}%`;
</script>
