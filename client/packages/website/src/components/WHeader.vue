<template>
  <div
    :class="{
      'fixed inset-x-0 z-10 backdrop-blur-xl transition-[top] duration-500 border-b border-fg-dark-darkest':
        !route.meta.staticHeader,
      'ease-out': !route.meta.staticHeader && scrolledOutDirection !== 'down',
      'ease-in': !route.meta.staticHeader && scrolledOutDirection === 'down',
    }"
    :style="{
      top: !route.meta.staticHeader ? (scrolledOutDirection !== 'down' ? 0 : 'calc(-6rem - 1px)') : undefined,
    }"
  >
    <div class="container flex items-center justify-between h-24 mx-auto space-x-8">
      <!-- "Left" -->
      <div class="flex flex-1 space-x-12">
        <UButton v-for="nav in leftNavs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
          {{ t(`nav.${nav}`).toUpperCase() }}
        </UButton>
      </div>
      <!-- END "Left" -->

      <!-- "Middle" -->
      <UButton
        :link="{ name: RouteName.HOME }"
        variant="outlined"
        link-active-exact
        class="flex-shrink-0 w-16 h-16 rounded-full"
      ></UButton>
      <!-- END "Middle" -->

      <!-- "Right" -->
      <div class="flex justify-end flex-1 space-x-12">
        <UButton v-for="nav in rightNavs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
          {{ t(`nav.${nav}`).toUpperCase() }}
        </UButton>
      </div>
      <!-- END "Right" -->
    </div>
  </div>
</template>

<script setup lang="ts">
  import { type WatchStopHandle, ref, shallowRef, watch } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useDark, useEventListener } from '@vueuse/core';
  import { UButton } from '@vcp-web-client/ui';
  import { RouteName } from '~/utils/constants';

  const route = useRoute();
  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Scroll based on `dimHeaderFooter`.
  ---------------------------------------------------------------- */
  const scrolledOutDirection = ref<'down' | 'up'>();
  const scrolledOutWatcher = ref<WatchStopHandle>();

  watch(
    () => route.meta.staticHeader,
    (dimHeaderFooter) => {
      if (scrolledOutWatcher.value) {
        scrolledOutWatcher.value();
        scrolledOutWatcher.value = undefined;
      }

      if (!dimHeaderFooter) {
        let prev = window.scrollY;

        scrolledOutWatcher.value = useEventListener('scroll', () => {
          const curr = window.scrollY;

          if (curr > prev && curr > 16 * 6) {
            scrolledOutDirection.value = 'down';
          } else if (curr < prev && curr > 16 * 6) {
            scrolledOutDirection.value = 'up';
          } else {
            scrolledOutDirection.value = undefined;
          }

          prev = curr;
        });
      }
    },
    {
      immediate: true,
    }
  );

  /* ----------------------------------------------------------------
    Dark theme
    ---------------------------------------------------------------- */
  const isDark = useDark({
    storageKey: 'theme',
  });

  // TODO: Implement light theme.
  isDark.value = true;

  /* ----------------------------------------------------------------
    Navigations
    ---------------------------------------------------------------- */
  const leftNavs = shallowRef([RouteName.ABOUT, RouteName.CONTACT]);

  const rightNavs = shallowRef([RouteName.BLOG, RouteName.NOTES]);
</script>
