<template>
  <div
    class="fixed inset-x-0 top-0"
    :class="{
      'bg-foreground-dark-default/5': !route.meta.dimHeaderFooter,
    }"
  >
    <div class="container flex items-center justify-between h-24 mx-auto space-x-8">
      <!-- "Left" -->
      <div class="flex space-x-12">
        <UButton v-for="nav in leftNavs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
          {{ t(`nav.${nav}`) }}
        </UButton>
      </div>
      <!-- END "Left" -->

      <!-- "Middle" -->
      <div class="w-16 h-16 border rounded-full border-foreground-dark-default"></div>
      <!-- END "Middle" -->

      <!-- "Right" -->
      <div class="flex justify-end space-x-12">
        <UButton v-for="nav in rightNavs" :key="nav" :link="{ name: nav }" size="lg" class="!p-0">
          {{ t(`nav.${nav}`) }}
        </UButton>
      </div>
      <!-- END "Right" -->
    </div>
  </div>
</template>

<script setup lang="ts">
  import { shallowRef } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useDark } from '@vueuse/core';
  import { UButton } from '@vcp-web-client/ui';
  import { useUiStore } from '~/store/ui';
  import { RouteName } from '~/utils/constants';

  const route = useRoute();
  const { t } = useI18n();
  const uiStore = useUiStore();

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
