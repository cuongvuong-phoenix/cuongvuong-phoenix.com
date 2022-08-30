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
  import { storeToRefs } from 'pinia';
  import { useI18n } from 'vue-i18n';
  import { useHead } from '@vueuse/head';
  import { HeaderHeight } from '~/store/ui';
  import { useHeadStore } from '~/store/head';
  import AnimatedRouterView from '~/components/AnimatedRouterView.vue';
  import WHeader from '~/components/WHeader.vue';
  import WFooter from '~/components/WFooter.vue';

  const route = useRoute();
  const headStore = useHeadStore();
  const { title, ogTitle, description } = storeToRefs(headStore);
  const { t, locale } = useI18n();

  /* ----------------------------------------------------------------
  Head
  ---------------------------------------------------------------- */
  // Initialize.
  title.value = t(`head.home.title`);
  ogTitle.value = t('common.my-app.title');
  description.value = t('common.my-app.description');

  watch(
    [() => route.name, locale, () => route.meta.dynamicHeadTitle, () => route.meta.customHead],
    ([name, _, dynamicHeadTitle, customHead]) => {
      if (!dynamicHeadTitle) {
        title.value = t(`head.${String(name)}.title`);
      }

      if (!customHead) {
        ogTitle.value = t('common.my-app.title');
        description.value = t('common.my-app.description');
      }
    }
  );

  useHead({
    title,
    meta: [
      {
        name: 'description',
        content: description,
      },
      // Open Graph protocol (https://ogp.me/).
      {
        property: 'og:title',
        content: ogTitle,
      },
      {
        property: 'og:description',
        content: description,
      },
      {
        property: 'og:image',
        content: '/apple-touch-icon.png',
      },
    ],
  });
</script>
