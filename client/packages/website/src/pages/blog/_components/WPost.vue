<template>
  <div class="relative px-12 py-8 transition-colors group hover:bg-bg-lighter">
    <!-- "Row 1" -->
    <RouterLink :to="{ name: RouteName.BLOG_POST, params: { locale, post: post.slug } }" class="text-xl font-medium">{{
      post.title
    }}</RouterLink>
    <!-- END "Row 1" -->

    <!-- "Row 2" -->
    <div class="flex items-center gap-3 mt-4 overflow-hidden text-sm text-fg-darker sm:flex-wrap">
      <span>{{ formatDatetime(post.updatedAt || post.createdAt, locale) }}</span>

      <span class="dot-1"></span>
      <span>{{ post.readingTime }} {{ t('common.minute', post.readingTime).toLowerCase() }}</span>

      <!-- "Tags" -->
      <template v-if="post.tags.length > 0">
        <span class="dot-1 sm:hidden"></span>
        <div class="inline-flex flex-wrap items-center flex-1 min-w-0 gap-2 sm:basis-full">
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
    <!-- END "Row 2" -->

    <!-- "Hover indicator" -->
    <div
      class="absolute inset-y-0 right-0 w-2 transition-opacity opacity-0 bg-primary-default group-hover:opacity-100"
    ></div>
    <!-- END "Hover indicator" -->
  </div>
</template>

<script setup lang="ts">
  import { useI18n } from 'vue-i18n';
  import { RouterLink } from 'vue-router';
  import { UIcon } from '@cvp-web-client/ui';
  import { RouteName } from '~/utils/constants';
  import { formatDatetime } from '~/utils/helpers';
  import { type Post, type Tag } from '~/types/graphql';

  interface WPost extends Omit<Post, 'visible' | 'content' | 'createdAt' | 'updatedAt' | 'tags'> {
    createdAt: Date;
    updatedAt?: Date;
    tags: Omit<Tag, 'createdAt' | 'updatedAt'>[];
  }

  defineProps<{
    post: WPost;
  }>();

  const { t, locale } = useI18n();
</script>
