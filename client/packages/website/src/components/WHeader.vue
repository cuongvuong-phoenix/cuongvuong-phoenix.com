<template>
  <div
    :class="{
      'fixed inset-x-0 z-10 backdrop-blur-xl transition-[top] duration-500 border-fg-darkest': !route.meta.staticHeader,
      'border-b': !route.meta.staticHeader && scrolledOutDirection,
      'ease-out': !route.meta.staticHeader && scrolledOutDirection !== 'down',
      'ease-in': !route.meta.staticHeader && scrolledOutDirection === 'down',
    }"
    :style="{
      top: !route.meta.staticHeader
        ? scrolledOutDirection !== 'down'
          ? 0
          : `${-(uiStore.headerHeight + 1)}px`
        : undefined,
    }"
  >
    <div
      class="container flex items-center justify-between mx-auto space-x-8"
      :style="{
        height: uiStore.headerHeightString,
      }"
    >
      <!-- "Left" -->
      <div class="flex flex-1 space-x-12">
        <UButton v-for="nav in leftNavs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
          {{ t(`nav.${nav}`).toUpperCase() }}
        </UButton>
      </div>
      <!-- END "Left" -->

      <!-- "Middle" -->
      <UButton :link="{ name: RouteName.HOME }" link-active-exact rounded unified class="!p-1">
        <ULogo colored class="w-[4.5rem] h-[4.5rem]" />
      </UButton>
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
  import { UButton, ULogo } from '@cvp-web-client/ui';
  import { useUiStore } from '~/store/ui';
  import { RouteName } from '~/utils/constants';

  const route = useRoute();
  const uiStore = useUiStore();
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
