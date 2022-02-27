<template>
  <div>
    <!-- "Top" -->
    <!-- "Row 1 - Tags" -->
    <div v-if="gqlPost && gqlPost.tags.length > 0" class="flex flex-wrap items-center justify-center space-x-2">
      <UPill v-for="tag in gqlPost?.tags" :key="tag.id" :icon="tag.icon" :name="tag.name" dim />
    </div>
    <!-- END "Row 1 - Tags" -->

    <!-- "Row 2 - Title" -->
    <h1 class="mt-6 font-serif text-4xl font-bold text-center">{{ gqlPost?.title }}</h1>
    <!-- END "Row 2 - Title" -->

    <!-- "Row 3" -->
    <div class="flex items-center justify-center mt-6 space-x-4 text-fg-darker">
      <div class="flex items-center space-x-2">
        <UIcon icon="fluent:calendar-ltr-24-regular" />
        <span v-if="gqlPost">{{ formatDatetime(gqlPost.createdAt, locale) }}</span>
      </div>

      <template v-if="gqlPost?.updatedAt">
        <span class="dot-1"></span>
        <div class="flex items-center space-x-2">
          <UIcon icon="fluent:calendar-edit-24-regular" />
          <span>{{ formatDatetime(gqlPost?.updatedAt, locale) }}</span>
        </div>
      </template>

      <span class="dot-1"></span>
      <div class="flex items-center space-x-2">
        <UIcon icon="fluent:book-clock-24-regular" />
        <span
          >{{ gqlPost?.readingTime }}
          {{ `${t('common.minute', gqlPost ? gqlPost.readingTime : 0)} ${t('common.read')}`.toLowerCase() }}</span
        >
      </div>
    </div>
    <!-- END "Row 3" -->
    <!-- END "Top" -->

    <!-- "Content" -->
    <div class="mx-auto mt-12 prose prose-light">
      {{ gqlPost?.content }}
    </div>
    <!-- END "Content" -->
  </div>
</template>

<script setup lang="ts">
  import { reactive, watch } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useQuery, useResult } from '@vue/apollo-composable';
  import { gql } from 'graphql-tag';
  import { parseISO } from 'date-fns';
  import { UIcon, UPill } from '@cvp-web-client/ui';
  import { formatDatetime } from '~/utils/helpers';
  import type { PostQuery, PostQueryVariables } from '~/types/graphql';

  const route = useRoute();
  const { t, locale } = useI18n();

  /* ----------------------------------------------------------------
  READ post
  ---------------------------------------------------------------- */
  // Vairables.
  const postQueryVariables = reactive({}) as PostQueryVariables;

  watch(
    () => route.params.post,
    (postParam) => {
      if (postParam) {
        postQueryVariables.slug = postParam as string;
      }
    },
    { immediate: true }
  );

  // GraphQL.
  const {
    result: postResult,
    loading: postLoading,
    error: postError,
  } = useQuery<PostQuery, PostQueryVariables>(
    gql`
      query post($slug: String!) {
        post(slug: $slug) {
          id
          title
          slug
          readingTime
          content
          createdAt
          updatedAt
          tags {
            id
            name
            icon
          }
        }
      }
    `,
    postQueryVariables
  );

  const gqlPost = useResult(postResult, undefined, (data) => ({
    ...data.post,
    createdAt: parseISO(data.post.createdAt),
    updatedAt: data.post.updatedAt ? parseISO(data.post.updatedAt) : undefined,
  }));
</script>
