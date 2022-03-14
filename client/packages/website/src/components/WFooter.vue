<template>
  <footer class="py-12">
    <div class="container mx-auto text-center">
      <!-- "Left" -->
      <i18n-t keypath="footer.about" tag="p" class="text-fg-darker">
        <span class="text-fg-default">{{ t('common.whoami') }}</span>
        <br />
        <span class="text-fg-default">Vue, GraphQL, Rust, PostgreSQL</span>
      </i18n-t>
      <!-- END "Left" -->
    </div>
  </footer>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n';
  import { UButton, UIcon } from '@cuongvuong-phoenix-com-client/ui';

  const { t } = useI18n();
</script>
