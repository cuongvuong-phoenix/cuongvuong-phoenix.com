<template>
  <div>
    <!-- "Top" -->
    <div class="flex items-center justify-center gap-x-16 gap-y-12 md:flex-col">
      <!-- "Left" -->
      <div class="w-[37.5%] space-y-8 md:w-auto">
        <h1 class="font-serif text-4xl font-bold sm:text-center" v-html="t('pages.about.top.greeting')"></h1>
        <p v-html="t('pages.about.top.biography')"></p>
      </div>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="p-4 border border-fg-darkest">
        <div class="border border-fg-darkest w-64 h-[28rem] overflow-hidden">
          <img :src="selfImageSrc" :alt="t('common.self-image')" class="object-cover w-full h-full" />
        </div>
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Top" -->

    <!-- "Hobbies Section" -->
    <div class="mt-12 space-y-8">
      <h2 class="font-serif text-3xl font-bold text-center">{{ t('pages.about.hobbies.title', 2).toUpperCase() }}</h2>

      <!-- "Hobbies" -->
      <div class="flex flex-col items-center gap-12">
        <div
          v-for="hobby in hobbies"
          :key="hobby.id"
          class="flex items-center gap-x-8 gap-y-4 w-[62.5%] lg:w-3/4 md:w-[87.5%] sm:w-auto sm:flex-col sm:px-4 sm:text-center"
        >
          <UIcon :icon="hobby.icon" class="text-fg-darkest wh-32 sm:wh-16" />
          <div class="self-stretch w-px bg-fg-darkest sm:hidden"></div>
          <p v-html="hobby.description"></p>
        </div>
      </div>
      <!-- END "Hobbies" -->
    </div>
    <!-- END "Hobbies Section" -->
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { UIcon } from '@cvp-web-client/ui';
  import selfImageSrc from '~/assets/images/self-image.jpg';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Hobbies
  ---------------------------------------------------------------- */
  interface Hobby {
    id: string;
    label: string;
    icon: string;
    description: string;
  }

  const hobbies = computed<Hobby[]>(() => [
    {
      id: 'vuejs',
      label: t('pages.about.hobbies.vuejs.label'),
      icon: 'mdi:vuejs',
      description: t(`pages.about.hobbies.vuejs.description`),
    },
    {
      id: 'rust',
      label: t('pages.about.hobbies.rust.label'),
      icon: 'mdi:language-rust',
      description: t(`pages.about.hobbies.rust.description`),
    },
    {
      id: 'uiux-design',
      label: t('pages.about.hobbies.uiux-design.label'),
      icon: 'fluent:design-ideas-24-regular',
      description: t(`pages.about.hobbies.uiux-design.description`),
    },
    {
      id: 'database',
      label: t('pages.about.hobbies.database.label'),
      icon: 'fluent:database-24-regular',
      description: t(`pages.about.hobbies.database.description`),
    },
    {
      id: 'linux',
      label: t('pages.about.hobbies.linux.label'),
      icon: 'cib:linux',
      description: t(`pages.about.hobbies.linux.description`),
    },
    {
      id: 'neovim',
      label: t('pages.about.hobbies.neovim.label'),
      icon: 'simple-icons:neovim',
      description: t(`pages.about.hobbies.neovim.description`),
    },
    {
      id: 'learning',
      label: t('pages.about.hobbies.learning.label'),
      icon: 'fluent:book-globe-24-regular',
      description: t(`pages.about.hobbies.learning.description`),
    },
    {
      id: 'music',
      label: t('pages.about.hobbies.music.label'),
      icon: 'fluent:music-note-24-regular',
      description: t(`pages.about.hobbies.music.description`),
    },
    {
      id: 'movies',
      label: t('pages.about.hobbies.movies.label'),
      icon: 'fluent:movies-and-tv-24-regular',
      description: t(`pages.about.hobbies.movies.description`),
    },
    {
      id: 'football',
      label: t('pages.about.hobbies.football.label'),
      icon: 'emojione-monotone:soccer-ball',
      description: t(`pages.about.hobbies.football.description`),
    },
  ]);
</script>
