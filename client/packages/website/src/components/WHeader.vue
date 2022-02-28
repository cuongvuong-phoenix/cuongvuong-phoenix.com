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
        <!-- "Left - Navs" -->
        <div class="flex flex-1 space-x-12">
          <nav v-for="nav in navs" :key="nav">
            <UButton :link="{ name: nav, params: { locale } }" size="lg" class="!p-0">
              {{ t(`nav.${nav}`).toUpperCase() }}
            </UButton>
          </nav>
        </div>
        <!-- END "Left - Navs" -->

        <!-- "Middle - Logo" -->
        <nav>
          <UButton :link="{ name: RouteName.HOME }" link-active-exact rounded unified class="!p-1">
            <ULogo colored class="w-[4.5rem] h-[4.5rem]" />
          </UButton>
        </nav>
        <!-- END "Middle - Logo" -->

        <!-- "Right - Social Networks" -->
        <div class="flex justify-end flex-1 space-x-2">
          <UButton
            v-for="socialNetwork in socialNetworks"
            :key="socialNetwork.href"
            :link="socialNetwork.href"
            unified
            rounded
          >
            <UIcon :icon="socialNetwork.icon" class="wh-7" />
          </UButton>
        </div>
        <!-- END "Right - Social Networks" -->
      </template>
      <!-- END "(2xl)" -->

      <!-- "(md)" -->
      <template v-else>
        <!-- "Left - Logo" -->
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
        <!-- END "Left - Logo" -->

        <!-- "Right - Menu Toggler" -->
        <div class="flex justify-end flex-1">
          <nav>
            <UButton rounded unified @click="uiStore.toggleHeaderMenuOpenning()">
              <UIcon v-if="!uiStore.headerMenuOpenning" icon="fluent:navigation-24-regular" />
              <UIcon v-else icon="fluent:dismiss-24-regular" />
            </UButton>
          </nav>
        </div>
        <!-- END "Right - Menu Toggler" -->

        <!-- "Menu" -->
        <Teleport v-if="uiStore.headerMenuOpenning" to="#app">
          <header
            class="fixed inset-x-0 bottom-0 z-10 px-16 py-8 bg-bg-default"
            :style="{
              top: `${uiStore.headerHeight + 1}px`,
            }"
          >
            <!-- "Navs" -->
            <nav v-for="nav in navs" :key="nav">
              <UButton
                :link="{ name: nav, params: { locale } }"
                size="lg"
                class="w-full"
                @click="uiStore.toggleHeaderMenuOpenning()"
              >
                {{ t(`nav.${nav}`).toUpperCase() }}
              </UButton>
            </nav>
            <!-- END "Navs" -->

            <hr class="my-4 border-fg-darkest" />

            <!-- "Social Networks" -->
            <div class="flex justify-center space-x-2">
              <UButton
                v-for="socialNetwork in socialNetworks"
                :key="socialNetwork.href"
                :link="socialNetwork.href"
                unified
                rounded
              >
                <UIcon :icon="socialNetwork.icon" class="wh-7" />
              </UButton>
            </div>
            <!-- END "Social Networks" -->
          </header>
        </Teleport>
        <!-- END "Menu" -->
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
  const navs = shallowRef([RouteName.ABOUT, RouteName.BLOG]);

  /* ----------------------------------------------------------------
  Social Networks
  ---------------------------------------------------------------- */
  interface SocialNetwork {
    href: string;
    icon: string;
  }
  const socialNetworks = [
    {
      href: 'https://github.com/vuong-cuong-phoenix',
      icon: 'mdi:github',
    },
    {
      href: 'https://www.facebook.com/vuongcuong.phoenix/',
      icon: 'mdi:facebook',
    },
    {
      href: 'https://www.linkedin.com/in/vuong-cuong-phoenix/',
      icon: 'mdi:linkedin',
    },
  ] as SocialNetwork[];
</script>
