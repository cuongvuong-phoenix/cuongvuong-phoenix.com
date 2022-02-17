<template>
  <div
    class="py-12"
    :class="{
      'dark:bg-foreground-dark-default/5': !route.meta.dimHeaderFooter,
    }"
  >
    <div class="container flex items-center justify-between mx-auto space-x-8">
      <!-- "Left" -->
      <i18n-t keypath="footer.about" tag="p" class="dark:text-foreground-dark-darker">
        <span class="dark:text-foreground-dark-default">{{ t('common.whoami') }}</span>
        <br />
        <span class="dark:text-foreground-dark-default">Vue 3, GraphQL, Rust (Axum), PostgreSQL</span>
      </i18n-t>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="flex items-center">
        <UButton link="https://github.com/vuong-cuong-phoenix" unified>
          <i-mdi-github />
        </UButton>

        <UButton link="https://www.facebook.com/vuongcuong.phoenix/" unified>
          <i-mdi-facebook />
        </UButton>

        <UButton link="https://www.linkedin.com/in/vuong-cuong-phoenix/" unified>
          <i-mdi-linkedin />
        </UButton>
      </div>
      <!-- END "Right" -->
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { UButton } from '@vcp-web-client/ui';

  const route = useRoute();
  const { t } = useI18n();
</script>
