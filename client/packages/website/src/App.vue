<template>
  <WHeader />

  <div
    class="container py-8 mx-auto"
    :style="{
      'margin-top': !route.meta.staticHeader ? uiStore.headerHeightString : undefined,
    }"
  >
    <RouterView />
  </div>

  <WFooter />
</template>

<script setup lang="ts">
  import { reactive, toRef, watch } from 'vue';
  import { RouterView, useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useHead } from '@vueuse/head';
  import { useUiStore } from '~/store/ui';
  import WHeader from '~/components/WHeader.vue';
  import WFooter from '~/components/WFooter.vue';

  const route = useRoute();
  const uiStore = useUiStore();
  const { t } = useI18n();

  /* ----------------------------------------------------------------
  App title
  ---------------------------------------------------------------- */
  const titleBase = 'CVP-Web';

  const title = reactive({
    full: titleBase,
    short: '',
  });

  // Auto-change `<title>` based on route name and locale.
  watch([() => route.name, () => route.params.locale], ([name, _]) => {
    const titleRoute = t(`nav.${String(name)}`);

    title.full = name === 'home' ? titleBase : `${titleBase} · ${titleRoute}`;
    title.short = titleRoute;
  });

  /* ----------------------------------------------------------------
  <head>
  ---------------------------------------------------------------- */
  useHead({
    title: toRef(title, 'full'),
    meta: [
      // Open Graph protocol (https://ogp.me/).
      {
        property: 'og:title',
        content: toRef(title, 'short'),
      },
    ],
  });
</script>
