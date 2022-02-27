<template>
  <div class="py-12">
    <div
      class="container flex items-center justify-between mx-auto space-x-8 md:!block md:space-x-0 md:space-y-8 md:text-center"
    >
      <!-- "Left" -->
      <i18n-t keypath="footer.about" tag="p" class="text-fg-darker">
        <span class="text-fg-default">{{ t('common.whoami') }}</span>
        <br />
        <span class="text-fg-default">Vue 3, GraphQL, Rust (Axum), PostgreSQL</span>
      </i18n-t>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="flex items-center justify-center space-x-2">
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
      <!-- END "Right" -->
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n';
  import { UButton, UIcon } from '@cvp-web-client/ui';

  const { t } = useI18n();

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
