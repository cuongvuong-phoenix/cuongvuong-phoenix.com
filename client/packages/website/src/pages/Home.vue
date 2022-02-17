<!-- TestComponent.vue -->

<template>
  <div class="py-16">
    <!-- "Top" -->
    <div class="font-serif font-bold text-center">
      <h1 class="text-5xl">{{ t('common.whoami') }}</h1>
      <h2 class="mt-4 text-4xl">{{ t('common.my-expertise') }}</h2>
    </div>
    <!-- END "Top" -->

    <!-- "Next" -->
    <div class="flex justify-between mt-16 space-x-32">
      <!-- "Left" -->
      <div class="flex flex-col justify-between flex-1">
        <div v-for="section in leftKebabSections" :key="section.label">
          <div class="font-bold dark:text-fg-dark-darker">{{ section.label.toUpperCase() }}</div>
          <div class="mt-4 font-serif text-xl font-medium line-clamp-5">{{ section.body }}</div>
        </div>
      </div>
      <!-- END "Left" -->

      <!-- "Middle" -->
      <div class="w-64 h-[28rem] rounded-[8rem] border dark:border-fg-dark-darkest p-4 self-center">
        <div class="w-full h-full rounded-[8rem] border dark:border-fg-dark-darkest overflow-hidden">
          <img :src="avatarUrl" :alt="t('common.avatar')" class="object-cover w-full h-full" />
        </div>
      </div>
      <!-- END "Middle" -->

      <!-- "Right" -->
      <div class="flex flex-col justify-between flex-1">
        <div v-for="section in rightKebabSections" :key="section.label" class="text-right">
          <div class="font-bold dark:text-fg-dark-darker">{{ section.label.toUpperCase() }}</div>
          <div class="mt-4 font-serif text-5xl font-medium text-ellipsis">{{ section.body }}</div>
        </div>
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Next" -->
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import { useI18n } from 'vue-i18n';
  import avatarUrl from '~/assets/images/avatar.jpg';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Kebab-sections
  ---------------------------------------------------------------- */
  interface KebabSection {
    label: string;
    body: string | number;
  }

  const leftKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.biography'),
      body: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Rutrum quisque non tellus orci ac. Turpis nunc eget lorem dolor sed. Semper quis lectus nulla at volutpat diam ut venenatis tellus. Libero volutpat sed cras ornare arcu. Malesuada fames ac turpis egestas sed. Pellentesque pulvinar pellentesque habitant morbi tristique senectus et netus et. Nisl purus in mollis nunc sed id semper risus in. Ut eu sem integer vitae justo. Ipsum dolor sit amet consectetur adipiscing elit duis. A iaculis at erat pellentesque adipiscing commodo elit at. Cursus in hac habitasse platea dictumst quisque sagittis. Nulla posuere sollicitudin aliquam ultrices.',
    },
    {
      label: t('pages.home.kebab-sections.contact'),
      body: 'Est pellentesque elit ullamcorper dignissim. Libero volutpat sed cras ornare arcu dui vivamus arcu. Ultrices eros in cursus turpis massa tincidunt dui ut ornare. Ac tincidunt vitae semper quis lectus nulla at volutpat diam. Sed odio morbi quis commodo. Posuere ac ut consequat semper viverra. Sed ullamcorper morbi tincidunt ornare massa eget. Molestie nunc non blandit massa enim nec. In cursus turpis massa tincidunt dui ut ornare. Non arcu risus quis varius quam quisque. Diam volutpat commodo sed egestas egestas fringilla phasellus faucibus scelerisque.',
    },
  ]);

  const rightKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.years-of-experience'),
      body: 2,
    },
    {
      label: t('pages.home.kebab-sections.blog'),
      body: 16,
    },
    {
      label: t('pages.home.kebab-sections.notes', 2),
      body: 8,
    },
  ]);
</script>
