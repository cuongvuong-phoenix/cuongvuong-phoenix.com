<template>
  <div>
    <!-- "Top" -->
    <div class="text-center font-serif">
      <h1 class="text-5xl font-bold">{{ t('common.whoami') }}</h1>
      <div class="text-fg-darker mt-4 text-4xl">{{ t('common.my-expertise') }}</div>
    </div>
    <!-- END "Top" -->

    <!-- "Next" -->
    <!-- "(2xl)" -->
    <template v-if="!smallerLg">
      <div class="mt-12 flex justify-between space-x-32">
        <!-- "Left" -->
        <div class="flex min-w-0 max-w-[16rem] flex-1 flex-col justify-between space-y-8">
          <div v-for="section in leftKebabSections" :key="section.label" class="space-y-4">
            <h2 class="text-fg-darker font-bold">{{ section.label.toUpperCase() }}</h2>
            <div
              v-if="section.body !== undefined && !section.loading"
              class="line-clamp-6 break-words font-serif text-xl font-medium"
              v-html="section.body"
            ></div>
            <USkeleton v-else-if="section.loading" type="paragraph" :num-lines="6" class="text-xl" />
          </div>
        </div>
        <!-- END "Left" -->

        <!-- "Middle - SelfImage" -->
        <div
          class="ring-fg-darkest hover:ring-primary-default h-[28rem] w-64 self-center rounded-[8rem] p-4 ring-1 transition duration-300"
        >
          <div class="border-fg-darkest h-full w-full overflow-hidden rounded-[8rem] border">
            <img :src="selfImageSrc" :alt="t('common.self-image')" class="h-full w-full object-cover" />
          </div>
        </div>
        <!-- END "Middle - SelfImage" -->

        <!-- "Right" -->
        <div class="flex min-w-0 max-w-[16rem] flex-1 flex-col justify-between space-y-8">
          <div v-for="section in rightKebabSections" :key="section.label" class="space-y-4 text-right">
            <h2 class="text-fg-darker font-bold">{{ section.label.toUpperCase() }}</h2>
            <div
              v-if="section.body !== undefined && !section.loading"
              class="truncate font-serif text-5xl font-medium"
              v-html="section.body"
            ></div>
            <USkeleton v-else-if="section.loading" type="line" font-size="5xl" />
          </div>
        </div>
        <!-- END "Right" -->
      </div>
    </template>
    <!-- END "(2xl)" -->

    <!-- "(lg)" -->
    <template v-else>
      <!-- "Self Image" -->
      <div class="mt-12 flex justify-center">
        <div
          class="ring-fg-darkest hover:ring-primary-default h-[28rem] w-64 self-center rounded-[8rem] p-4 ring-1 transition duration-300"
        >
          <div class="border-fg-darkest h-full w-full overflow-hidden rounded-[8rem] border">
            <img :src="selfImageSrc" :alt="t('common.self-image')" class="h-full w-full object-cover" />
          </div>
        </div>
      </div>
      <!-- END "Self Image" -->

      <!-- "Left Sections" -->
      <div class="mt-12 grid grid-cols-2 gap-16 text-center md:grid-cols-1 md:gap-8">
        <div v-for="section in leftKebabSections" :key="section.label" class="flex flex-col items-center space-y-4">
          <h2 class="text-fg-darker font-bold">{{ section.label.toUpperCase() }}</h2>
          <div
            v-if="section.body !== undefined"
            class="line-clamp-6 font-serif text-xl font-medium"
            v-html="section.body"
          ></div>
          <USkeleton v-else-if="section.loading" type="paragraph" :num-lines="6" class="w-full text-xl" />
        </div>
      </div>
      <!-- END "Left Sections" -->

      <!-- "Right Sections" -->
      <div class="mt-12 grid grid-cols-3 gap-16 text-center md:grid-cols-1 md:gap-8">
        <div v-for="section in rightKebabSections" :key="section.label" class="flex flex-col items-center space-y-4">
          <h2 class="text-fg-darker font-bold">{{ section.label.toUpperCase() }}</h2>
          <div
            v-if="section.body !== undefined && !section.loading"
            class="truncate font-serif text-5xl font-medium"
            v-html="section.body"
          ></div>
          <USkeleton v-else-if="section.loading" type="line" font-size="5xl" />
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
  import { USkeleton } from '@cuongvuong-phoenix-com-client/ui';
  import selfImageSrc from '~/assets/images/self-image.webp';
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
      postsCount(search: "", tagIds: [])
    }
  `);

  interface KebabSection {
    label: string;
    body?: string | number;
    loading?: boolean;
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
      loading: postsCountLoading.value,
    },
    {
      label: t('pages.home.kebab-sections.projects.label', 2),
      body: t('pages.home.kebab-sections.projects.body', 2),
    },
  ]);
</script>
