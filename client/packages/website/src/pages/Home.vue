<template>
  <div>
    <!-- "Top" -->
    <div class="font-serif font-bold text-center">
      <h1 class="text-5xl">{{ t('common.whoami') }}</h1>
      <h2 class="mt-4 text-4xl">{{ t('common.my-expertise') }}</h2>
    </div>
    <!-- END "Top" -->

    <!-- "Next" -->
    <!-- "(2xl)" -->
    <template v-if="!smallerLg">
      <div class="flex mt-12 space-x-32">
        <!-- "Left" -->
        <div class="flex flex-col justify-between flex-1 min-w-0">
          <div v-for="section in leftKebabSections" :key="section.label" class="space-y-4">
            <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
            <div
              v-if="section.body"
              class="font-serif text-xl font-medium break-words line-clamp-6"
              v-html="section.body"
            ></div>
          </div>
        </div>
        <!-- END "Left" -->

        <!-- "Middle - Avatar" -->
        <div
          class="w-64 h-[28rem] p-4 self-center transition duration-300 rounded-[8rem] ring-1 ring-fg-darkest hover:ring-primary-default"
        >
          <div class="w-full h-full rounded-[8rem] border border-fg-darkest overflow-hidden">
            <img :src="selfImageSrc" :alt="t('common.self-image')" class="object-cover w-full h-full" />
          </div>
        </div>
        <!-- END "Middle - Avatar" -->

        <!-- "Right" -->
        <div class="flex flex-col justify-between flex-1 min-w-0">
          <div v-for="section in rightKebabSections" :key="section.label" class="space-y-4 text-right">
            <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
            <div v-if="section.body" class="font-serif text-5xl font-medium truncate" v-html="section.body"></div>
          </div>
        </div>
        <!-- END "Right" -->
      </div>
    </template>
    <!-- END "(2xl)" -->

    <!-- "(lg)" -->
    <template v-else>
      <!-- "Self Image" -->
      <div class="flex justify-center mt-12">
        <div
          class="w-64 h-[28rem] p-4 self-center transition duration-300 rounded-[8rem] ring-1 ring-fg-darkest hover:ring-primary-default"
        >
          <div class="w-full h-full rounded-[8rem] border border-fg-darkest overflow-hidden">
            <img :src="selfImageSrc" :alt="t('common.self-image')" class="object-cover w-full h-full" />
          </div>
        </div>
      </div>
      <!-- END "Self Image" -->

      <!-- "Left Sections" -->
      <div class="grid grid-cols-2 gap-16 mt-12 text-center md:grid-cols-1 md:gap-8">
        <div
          v-for="section in leftKebabSections"
          :key="section.label"
          class="flex flex-col items-center justify-between space-y-4"
        >
          <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
          <div v-if="section.body" class="font-serif text-xl font-medium line-clamp-6" v-html="section.body"></div>
        </div>
      </div>
      <!-- END "Left Sections" -->

      <!-- "Right Sections" -->
      <div class="grid grid-cols-3 gap-16 mt-12 text-center md:grid-cols-1 md:gap-8">
        <div
          v-for="section in rightKebabSections"
          :key="section.label"
          class="flex flex-col items-center justify-between space-y-4"
        >
          <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
          <div v-if="section.body" class="font-serif text-5xl font-medium truncate" v-html="section.body"></div>
        </div>
      </div>
      <!-- END "Right Sections" -->
    </template>
    <!-- END "(lg)" -->
    <!-- END "Next" -->
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { useBreakpoints } from '@vueuse/core';
  import { useQuery } from '@vue/apollo-composable';
  import { gql } from 'graphql-tag';
  import selfImageSrc from '~/assets/images/avatar.jpg';
  import { appBreakpoints } from '~/utils/constants';
  import type { PostsCountQuery } from '~/types/graphql';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Breakpoints
  ---------------------------------------------------------------- */
  const breakpoints = useBreakpoints(appBreakpoints);

  const smallerLg = breakpoints.smaller('lg');

  /* ----------------------------------------------------------------
  READ Home Content
  ---------------------------------------------------------------- */
  const {
    result: postsCountResult,
    loading: postsCountLoading,
    error: postsCountError,
  } = useQuery<PostsCountQuery>(gql`
    query postsCount {
      postsCount
    }
  `);

  interface KebabSection {
    label: string;
    body?: string | number;
  }

  const leftKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.biography.label'),
      body: t('pages.home.kebab-sections.biography.body'),
    },
    {
      label: t('pages.home.kebab-sections.contact.label'),
      body: t('pages.home.kebab-sections.contact.body'),
    },
  ]);

  const rightKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.years-of-experience.label'),
      body: t('pages.home.kebab-sections.years-of-experience.body'),
    },
    {
      label: t('pages.home.kebab-sections.blog.label'),
      body: postsCountResult.value?.postsCount,
    },
    {
      label: t('pages.home.kebab-sections.projects.label', 2),
      body: t('pages.home.kebab-sections.projects.body', 2),
    },
  ]);
</script>
