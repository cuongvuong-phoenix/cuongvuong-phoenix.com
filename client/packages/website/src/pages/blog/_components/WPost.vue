<template>
  <div
    class="px-12 py-8"
    :class="[post && !loading ? 'hover:bg-bg-lighter group relative transition-colors' : 'animate-pulse']"
  >
    <!-- "Row 1" -->
    <RouterLink
      v-if="post && !loading"
      :to="{ name: RouteName.BLOG_POST, params: { post: post.slug } }"
      class="text-xl font-medium"
      >{{ post.title }}</RouterLink
    >
    <USkeleton v-else-if="loading" type="line" font-size="xl" class="w-3/4" />
    <!-- END "Row 1" -->

    <!-- "Row 2" -->
    <div
      v-if="post && !loading"
      class="text-fg-darker mt-4 flex items-center gap-3 overflow-hidden text-sm sm:flex-wrap"
    >
      <span>{{ formatDatetime(post.updatedAt || post.createdAt, locale) }}</span>

      <span class="dot-1"></span>
      <span>{{ post.readingTime }} {{ t('common.minute', post.readingTime).toLowerCase() }}</span>

      <!-- "Tags" -->
      <template v-if="post.tags.length > 0">
        <span class="dot-1 sm:hidden"></span>
        <div class="inline-flex min-w-0 flex-1 flex-wrap items-center gap-2 sm:basis-full">
          <div v-for="(tag, i) in post.tags" :key="tag.id" class="inline-flex items-center">
            <div class="inline-flex items-center space-x-1">
              <UIcon v-if="tag.icon" :icon="tag.icon" class="min-wh-5" />
              <span class="whitespace-nowrap">{{ tag.name }}</span>
            </div>

            <span v-if="i < post.tags.length - 1">&comma;</span>
          </div>
        </div>
      </template>
      <!-- END "Tags" -->
    </div>
    <USkeleton v-else-if="loading" type="line" font-size="sm" class="mt-4 w-1/2" />
    <!-- END "Row 2" -->

    <!-- "Hover indicator" -->
    <div
      v-if="post && !loading"
      class="bg-primary-default absolute inset-y-0 right-0 w-2 opacity-0 transition-opacity group-hover:opacity-100"
    ></div>
    <!-- END "Hover indicator" -->
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n';
  import { RouterLink } from 'vue-router';
  import { UIcon, USkeleton } from '@cuongvuong-phoenix-com-client/ui';
  import { RouteName } from '~/utils/constants';
  import { formatDatetime } from '~/utils/helpers';
  import { type Post, type Tag } from '~/types/graphql';

  interface WPost extends Omit<Post, 'visible' | 'content' | 'createdAt' | 'updatedAt' | 'tags'> {
    createdAt: Date;
    updatedAt?: Date;
    tags: Omit<Tag, 'createdAt' | 'updatedAt'>[];
  }

  defineProps<{
    post?: WPost;
    loading?: boolean;
  }>();

  const { t, locale } = useI18n();
</script>
