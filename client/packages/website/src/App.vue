<template>
  <WHeader />

  <main
    class="container mx-auto pt-8 pb-16"
    :style="{
      'margin-top': `${HeaderHeight.DEFAULT}px`,
    }"
  >
    <AnimatedRouterView />
  </main>

  <WFooter />
</template>

<script setup lang="ts">
  import { computed, watch } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useHead } from '@vueuse/head';
  import { HeaderHeight } from '~/store/ui';
  import { useHeadStore } from '~/store/head';
  import AnimatedRouterView from '~/components/AnimatedRouterView.vue';
  import WHeader from '~/components/WHeader.vue';
  import WFooter from '~/components/WFooter.vue';

  const route = useRoute();
  const { t, locale } = useI18n();
  const headStore = useHeadStore();

  /* ----------------------------------------------------------------
  Head
  ---------------------------------------------------------------- */
  // Initialize.
  headStore.title = t(`head.home.title`);
  headStore.ogTitle = t('common.my-app.title');
  headStore.description = t('common.my-app.description');

  watch(
    [() => route.name, locale, () => route.meta.dynamicHeadTitle, () => route.meta.customHead],
    ([name, _, dynamicHeadTitle, customHead]) => {
      if (!dynamicHeadTitle) {
        headStore.title = t(`head.${String(name)}.title`);
      }

      if (!customHead) {
        headStore.ogTitle = t('common.my-app.title');
        headStore.description = t('common.my-app.description');
      }
    }
  );

  useHead({
    title: computed(() => headStore.title),
    meta: [
      {
        name: 'description',
        content: computed(() => headStore.description),
      },
      // Open Graph protocol (https://ogp.me/).
      {
        property: 'og:title',
        content: computed(() => headStore.ogTitle),
      },
      {
        property: 'og:description',
        content: computed(() => headStore.description),
      },
      {
        property: 'og:image',
        content: '/apple-touch-icon.png',
      },
    ],
  });
</script>
