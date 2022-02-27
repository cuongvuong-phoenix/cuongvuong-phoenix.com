<template>
  <div>
    <!-- "Top" -->
    <div class="font-serif font-bold text-center">
      <h1 class="text-5xl">{{ t('common.whoami') }}</h1>
      <h2 class="mt-4 text-4xl">{{ t('common.my-expertise') }}</h2>
    </div>
    <!-- END "Top" -->

    <!-- "Next" -->
    <div class="flex justify-between mt-12 space-x-32">
      <!-- "Left" -->
      <div class="flex flex-col justify-between flex-1 min-w-0">
        <div v-for="section in leftKebabSections" :key="section.label">
          <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
          <div class="mt-4 font-serif text-xl font-medium line-clamp-5">{{ section.body }}</div>
        </div>
      </div>
      <!-- END "Left" -->

      <!-- "Middle" -->
      <div
        class="w-64 h-[28rem] p-4 self-center transition duration-300 rounded-[8rem] ring-1 ring-fg-darkest hover:ring-primary-default"
      >
        <div class="w-full h-full rounded-[8rem] border border-fg-darkest overflow-hidden">
          <img :src="avatarUrl" :alt="t('common.avatar')" class="object-cover w-full h-full" />
        </div>
      </div>
      <!-- END "Middle" -->

      <!-- "Right" -->
      <div class="flex flex-col justify-between flex-1 min-w-0">
        <div v-for="section in rightKebabSections" :key="section.label" class="text-right">
          <div class="font-bold text-fg-darker">{{ section.label.toUpperCase() }}</div>
          <div class="mt-4 font-serif text-5xl font-medium truncate">{{ section.body }}</div>
        </div>
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Next" -->
  </div>
</template>

<script setup lang="ts">
  import { gql } from 'graphql-tag';
  import { computed } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { useQuery } from '@vue/apollo-composable';
  import avatarUrl from '~/assets/images/avatar.jpg';
  import { type HomeContentQuery } from '~/types/graphql';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  READ Home Content
  ---------------------------------------------------------------- */
  const {
    result: homeContentResult,
    loading: homeContentLoading,
    error: homeContentError,
  } = useQuery<HomeContentQuery>(gql`
    query homeContent {
      homeContent {
        biography
        contact
        yearsOfExperience
        numBlogPosts
        numProjects
      }
    }
  `);

  interface KebabSection {
    label: string;
    body?: string | number;
  }

  const leftKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.biography'),
      body: homeContentResult.value?.homeContent.biography,
    },
    {
      label: t('pages.home.kebab-sections.contact'),
      body: homeContentResult.value?.homeContent.contact,
    },
  ]);

  const rightKebabSections = computed<KebabSection[]>(() => [
    {
      label: t('pages.home.kebab-sections.years-of-experience'),
      body: homeContentResult.value?.homeContent.yearsOfExperience,
    },
    {
      label: t('pages.home.kebab-sections.blog'),
      body: homeContentResult.value?.homeContent.numBlogPosts,
    },
    {
      label: t('pages.home.kebab-sections.projects', 2),
      body: homeContentResult.value?.homeContent.numProjects,
    },
  ]);
</script>
