<template>
  <div ref="el" class="min-h-[1.5em] min-w-[1.5em] shrink-0"></div>
</template>

<script setup lang="ts">
  import { nextTick, ref, watch } from 'vue';
  import Iconify from '@purge-icons/generated';

  const { icon } = defineProps<{
    icon: string;
  }>();

  const el = ref<SVGElement | null>(null);

  watch(
    () => icon,
    async () => {
      await nextTick();

      if (el.value) {
        el.value.textContent = null;

        const svg = Iconify.renderSVG(icon, {});

        if (svg) {
          el.value.appendChild(svg);
        } else {
          const span = document.createElement('span');
          span.className = 'iconify';
          span.dataset.icon = icon;
          el.value.appendChild(span);
        }
      }
    },
    { flush: 'post', immediate: true }
  );
</script>

<style lang="postcss">
  .iconify {
    @apply h-full w-full;
  }
</style>
