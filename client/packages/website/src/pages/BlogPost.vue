<template>
  <div>
    <!-- "Top" -->
    <!-- "Row 1 - Tags" -->
    <div v-if="post.tags.length > 0" class="flex flex-wrap items-center justify-center space-x-2">
      <UPill v-for="tag in post.tags" :key="tag.id" :icon="tag.icon" :name="tag.name" dim />
    </div>
    <!-- END "Row 1 - Tags" -->

    <!-- "Row 2 - Title" -->
    <h1 class="mt-6 font-serif text-4xl font-bold text-center">{{ post.title }}</h1>
    <!-- END "Row 2 - Title" -->

    <!-- "Row 3" -->
    <div class="flex items-center justify-center mt-6 space-x-4 text-fg-darker">
      <div class="flex items-center space-x-2">
        <UIcon icon="fluent:calendar-ltr-24-regular" />
        <span>{{ formatDatetime(post.createdAt, locale) }}</span>
      </div>

      <template v-if="post.updatedAt">
        <span class="dot-1"></span>
        <div class="flex items-center space-x-2">
          <UIcon icon="fluent:calendar-edit-24-regular" />
          <span>{{ formatDatetime(post.updatedAt, locale) }}</span>
        </div>
      </template>

      <span class="dot-1"></span>
      <div class="flex items-center space-x-2">
        <UIcon icon="fluent:book-clock-24-regular" />
        <span
          >{{ post.readingTime }}
          {{ `${t('common.minute', post.readingTime)} ${t('common.read')}`.toLowerCase() }}</span
        >
      </div>
    </div>
    <!-- END "Row 3" -->
    <!-- END "Top" -->

    <!-- "Body" -->
    <div class="mx-auto mt-16 prose prose-light">
      {{ post.body }}
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import { useRoute } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import faker from '@faker-js/faker/locale/en';
  import { UIcon, UPill } from '@cvp-web-client/ui';
  import { postTags } from '~/utils/mocks';
  import { formatDatetime } from '~/utils/helpers';

  const route = useRoute();
  const { t, locale } = useI18n();

  const slug = computed(() => route.params.post as string);

  /* ----------------------------------------------------------------
  READ post
  ---------------------------------------------------------------- */
  const post = computed(
    () =>
      ({
        id: faker.datatype.uuid(),
        slug: slug.value,
        title: faker.lorem.words(8),
        createdAt: faker.date.past(1),
        updatedAt: faker.random.arrayElement([faker.date.past(0.5), undefined]),
        readingTime: faker.datatype.number(60),
        tags: faker.random.arrayElements(postTags, faker.datatype.number(4)),
        body: faker.lorem.paragraphs(16),
      } as Model.PostR)
  );
</script>
