<template>
  <header
    class="border-fg-darkest fixed inset-x-0 top-0 z-10"
    :class="{
      'bg-bg-default border-b': headerMenuOpenning,
      'border-b backdrop-blur-xl': scrolledOut,
    }"
    :style="{
      'padding-right': smallerMd && headerMenuOpenning ? 'var(--scrollbar--width)' : undefined,
    }"
  >
    <nav
      class="container mx-auto flex items-center space-x-8 transition-[height] duration-300"
      :style="{
        height: headerHeightString,
      }"
    >
      <!-- "(2xl)" -->
      <template v-if="!smallerMd">
        <!-- "Left - Navs" -->
        <div class="flex flex-1 space-x-12">
          <UButton v-for="nav in navs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
            {{ t(`nav.${nav}`).toUpperCase() }}
          </UButton>
        </div>
        <!-- END "Left - Navs" -->

        <!-- "Middle - Logo" -->
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
            class="h-[4.5rem] w-[4.5rem] transition-[height_width] duration-300"
            :class="{
              'wh-12': scrolledOut,
            }"
          />
        </UButton>
        <!-- END "Middle - Logo" -->

        <!-- "Right - Social Networks" -->
        <div class="flex flex-1 justify-end space-x-2">
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
            class="h-[4.5rem] w-[4.5rem] transition-[height_width] duration-300"
            :class="{
              'wh-12': scrolledOut,
            }"
          />
        </UButton>
        <!-- END "Left - Logo" -->

        <!-- "Right - Menu Toggler" -->
        <div class="flex flex-1 justify-end">
          <UButton rounded unified @click="uiStore.toggleHeaderMenuOpenning()">
            <UIcon v-if="!headerMenuOpenning" icon="fluent:navigation-24-regular" />
            <UIcon v-else icon="fluent:dismiss-24-regular" />
          </UButton>
        </div>
        <!-- END "Right - Menu Toggler" -->

        <!-- "Menu" -->
        <Teleport v-if="headerMenuOpenning" to="#app">
          <nav
            class="bg-bg-default fixed inset-x-0 bottom-0 z-10 px-16 pt-8 pb-16"
            :style="{
              top: `${headerHeight + 1}px`,
            }"
          >
            <!-- "Navs" -->
            <UButton
              v-for="nav in navs"
              :key="nav"
              :link="{ name: nav }"
              size="lg"
              class="w-full"
              @click="uiStore.toggleHeaderMenuOpenning()"
            >
              {{ t(`nav.${nav}`).toUpperCase() }}
            </UButton>
            <!-- END "Navs" -->

            <hr class="border-fg-darkest my-4" />

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
          </nav>
        </Teleport>
        <!-- END "Menu" -->
      </template>
      <!-- END "(md)" -->
    </nav>
  </header>
</template>

<script setup lang="ts">
  import { ref, shallowRef, watch } from 'vue';
  import { storeToRefs } from 'pinia';
  import { useI18n } from 'vue-i18n';
  import { useBreakpoints, useColorMode, useEventListener, useStyleTag } from '@vueuse/core';
  import { UButton, UIcon, ULogo } from '@cuongvuong-phoenix-com-client/ui';
  import { HeaderHeight, useUiStore } from '~/store/ui';
  import { RouteName, appBreakpoints } from '~/utils/constants';

  const uiStore = useUiStore();
  const { headerHeight, headerHeightString, headerMenuOpenning } = storeToRefs(uiStore);
  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Breakpoints
  ---------------------------------------------------------------- */
  const breakpoints = useBreakpoints(appBreakpoints);

  const smallerMd = breakpoints.smaller('md');

  /* ----------------------------------------------------------------
  (md) Menu
  ---------------------------------------------------------------- */
  const { load: loadBodyStyleTag, unload: unloadBodyStyleTag } = useStyleTag('body { overflow: hidden }', {
    immediate: false,
  });

  watch(
    [smallerMd, headerMenuOpenning],
    ([smallerMdValue, headerMenuOpenningValue]) => {
      if (smallerMdValue && headerMenuOpenningValue) {
        loadBodyStyleTag();
      } else {
        unloadBodyStyleTag();
      }
    },
    { immediate: true }
  );

  /* ----------------------------------------------------------------
  Scroll.
  ---------------------------------------------------------------- */
  const scrolledOut = ref<boolean>(false);

  useEventListener('scroll', () => {
    const curr = window.scrollY;

    if (curr > headerHeight.value) {
      scrolledOut.value = true;
      headerHeight.value = HeaderHeight.NARROW;
    } else {
      scrolledOut.value = false;
      headerHeight.value = HeaderHeight.DEFAULT;
    }
  });

  /* ----------------------------------------------------------------
  Dark theme
  ---------------------------------------------------------------- */
  // TODO: Light theme.
  const colorMode = useColorMode({
    storageKey: 'theme',
  });

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
      href: 'https://github.com/cuongvuong-phoenix',
      icon: 'mdi:github',
    },
    {
      href: 'https://www.facebook.com/cuongvuong.phoenix/',
      icon: 'mdi:facebook',
    },
    {
      href: 'https://www.linkedin.com/in/cuongvuong-phoenix/',
      icon: 'mdi:linkedin',
    },
  ] as SocialNetwork[];
</script>
