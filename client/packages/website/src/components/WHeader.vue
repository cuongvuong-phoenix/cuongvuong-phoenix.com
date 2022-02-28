<template>
  <!-- <header
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
  > -->
  <header
    :class="{
      'fixed inset-x-0 top-0 z-10 border-fg-darkest': uiStore.headerMenuOpenning || !route.meta.staticHeader,
      'border-b bg-bg-default': uiStore.headerMenuOpenning,
      'backdrop-blur-xl ': !route.meta.staticHeader,
      'border-b': !route.meta.staticHeader && scrolledOutDirection,
    }"
    :style="{
      'padding-right': smallerMd && uiStore.headerMenuOpenning ? 'var(--scrollbar--width)' : undefined,
    }"
  >
    <div
      class="container flex items-center mx-auto space-x-8"
      :style="{
        height: uiStore.headerHeightString,
      }"
    >
      <!-- "(2xl)" -->
      <template v-if="!smallerMd">
        <!-- "Left" -->
        <div class="flex flex-1 space-x-12">
          <nav v-for="nav in leftNavs" :key="nav">
            <UButton :link="{ name: nav, params: { locale } }" size="lg" class="!p-0">
              {{ t(`nav.${nav}`).toUpperCase() }}
            </UButton>
          </nav>
        </div>
        <!-- END "Left" -->

        <!-- "Middle" -->
        <nav>
          <UButton :link="{ name: RouteName.HOME }" link-active-exact rounded unified class="!p-1">
            <ULogo colored class="w-[4.5rem] h-[4.5rem]" />
          </UButton>
        </nav>
        <!-- END "Middle" -->

        <!-- "Right" -->
        <div class="flex justify-end flex-1 space-x-12">
          <nav v-for="nav in rightNavs" :key="nav">
            <UButton :link="{ name: nav, params: { locale } }" size="lg" class="!p-0">
              {{ t(`nav.${nav}`).toUpperCase() }}
            </UButton>
          </nav>
        </div>
        <!-- END "Right" -->
      </template>
      <!-- END "(2xl)" -->

      <!-- "(md)" -->
      <template v-else>
        <!-- "Left" -->
        <nav>
          <UButton
            :link="{ name: RouteName.HOME }"
            link-active-exact
            rounded
            unified
            class="!p-1"
            @click="uiStore.toggleHeaderMenuOpenning(false)"
          >
            <ULogo colored class="w-[4.5rem] h-[4.5rem]" />
          </UButton>
        </nav>
        <!-- END "Left" -->

        <!-- "Right" -->
        <div class="flex justify-end flex-1">
          <nav>
            <UButton rounded unified @click="uiStore.toggleHeaderMenuOpenning()">
              <UIcon v-if="!uiStore.headerMenuOpenning" icon="fluent:navigation-24-regular" />
              <UIcon v-else icon="fluent:dismiss-24-regular" />
            </UButton>
          </nav>
        </div>
        <!-- END "Right" -->

        <Teleport v-if="uiStore.headerMenuOpenning" to="#app">
          <header
            class="fixed inset-x-0 bottom-0 z-10 px-6 py-8 bg-bg-default"
            :style="{
              top: `${uiStore.headerHeight + 1}px`,
            }"
          >
            <nav v-for="nav in [...leftNavs, ...rightNavs]" :key="nav">
              <UButton
                :link="{ name: nav, params: { locale } }"
                size="lg"
                class="w-full"
                @click="uiStore.toggleHeaderMenuOpenning()"
              >
                {{ t(`nav.${nav}`).toUpperCase() }}
              </UButton>
            </nav>
          </header>
        </Teleport>
      </template>
      <!-- END "(md)" -->
    </div>
  </header>
</template>

<script setup lang="ts">
  import { type WatchStopHandle, ref, shallowRef, watch } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useBreakpoints, useDark, useEventListener, useStyleTag } from '@vueuse/core';
  import { UButton, UIcon, ULogo } from '@cvp-web-client/ui';
  import { useUiStore } from '~/store/ui';
  import { RouteName, appBreakpoints } from '~/utils/constants';

  const route = useRoute();
  const uiStore = useUiStore();
  const { t, locale } = useI18n();

  /* ----------------------------------------------------------------
  Breakpoints
  ---------------------------------------------------------------- */
  const breakpoints = useBreakpoints(appBreakpoints);

  const smallerMd = breakpoints.smaller('md');

  /* ----------------------------------------------------------------
  (md) Menu
  ---------------------------------------------------------------- */
  const { load: loadBodyStyleTag, unload: unloadBodyStyleTag } = useStyleTag('body { overflow: hidden }');

  watch(
    [smallerMd, () => uiStore.headerMenuOpenning],
    ([smallerMdValue, headerMenuOpenning]) => {
      if (smallerMdValue && headerMenuOpenning) {
        loadBodyStyleTag();
      } else {
        unloadBodyStyleTag();
      }
    },
    { immediate: true }
  );

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
