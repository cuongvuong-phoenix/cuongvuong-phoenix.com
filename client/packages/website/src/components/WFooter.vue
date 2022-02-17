<template>
  <div class="py-12">
    <div class="container flex items-center justify-between mx-auto space-x-8">
      <!-- "Left" -->
      <i18n-t keypath="footer.about" tag="p" class="dark:text-fg-dark-darker">
        <span class="dark:text-fg-dark-default">{{ t('common.whoami') }}</span>
        <br />
        <span class="dark:text-fg-dark-default">Vue 3, GraphQL, Rust (Axum), PostgreSQL</span>
      </i18n-t>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="flex items-center space-x-1">
        <UButton link="https://github.com/vuong-cuong-phoenix" size="lg" unified rounded>
          <i-mdi-github class="square-lg" />
        </UButton>

        <UButton link="https://www.facebook.com/vuongcuong.phoenix/" size="lg" unified rounded>
          <i-mdi-facebook class="square-lg" />
        </UButton>

        <UButton link="https://www.linkedin.com/in/vuong-cuong-phoenix/" size="lg" unified rounded>
          <i-mdi-linkedin class="square-lg" />
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
