<template>
  <header
    class="fixed inset-x-0 top-0 z-10 border-fg-darkest"
    :class="{
      'border-b bg-bg-default': uiStore.headerMenuOpenning,
      'border-b backdrop-blur-xl': scrolledOut,
    }"
    :style="{
      'padding-right': smallerMd && uiStore.headerMenuOpenning ? 'var(--scrollbar--width)' : undefined,
    }"
  >
    <div
      class="container flex items-center mx-auto space-x-8 transition-[height] duration-300"
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
          <UButton
            :link="{ name: RouteName.HOME }"
            link-active-exact
            rounded
            unified
            class="!p-1 duration-300"
            :class="{
              'rotate-[-360deg]': scrolledOut,
            }"
          >
            <ULogo
              colored
              class="w-[4.5rem] h-[4.5rem] transition-[height_width] duration-300"
              :class="{
                'wh-12': scrolledOut,
              }"
            />
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
            class="!p-1 duration-300"
            :class="{
              'rotate-[-360deg]': scrolledOut,
            }"
            @click="uiStore.toggleHeaderMenuOpenning(false)"
          >
            <ULogo
              colored
              class="w-[4.5rem] h-[4.5rem] transition-[height_width] duration-300"
              :class="{
                'wh-12': scrolledOut,
              }"
            />
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
  import { HeaderHeight, useUiStore } from '~/store/ui';
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
  const scrolledOut = ref<boolean>(false);

  useEventListener('scroll', () => {
    const curr = window.scrollY;

    if (curr > uiStore.headerHeight) {
      scrolledOut.value = true;
      uiStore.setHeaderHeight(HeaderHeight.NARROW);
    } else {
      scrolledOut.value = false;
      uiStore.setHeaderHeight(HeaderHeight.DEFAULT);
    }
  });

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
