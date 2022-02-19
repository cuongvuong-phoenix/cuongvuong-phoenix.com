<template>
  <div>
    <h1 class="font-serif text-5xl font-bold text-center">
      {{ t('common.blog') }}
    </h1>

    <!-- "Body" -->
    <div class="grid grid-cols-3 gap-8 mt-16">
      <!-- "Left" -->
      <div>
        <div class="sticky space-y-4 top-28">
          <!-- "Search box" -->
          <UInput id="search-box" v-model="search" :placeholder="`${t('common.search')}...`">
            <template #prepended>
              <UIcon icon="fluent:search-24-regular" />
            </template>
          </UInput>
          <!-- END "Search box" -->

          <!-- "Total results count" -->
          <p class="truncate">
            <span class="font-bold text-fg-darker">{{ t('common.total').toUpperCase() }}&colon;&nbsp;</span
            ><span class="italic">{{ totalPosts }} {{ t('common.post', totalPosts).toLowerCase() }}</span>
          </p>
          <!-- END "Total results count" -->

          <!-- "Sort" -->
          <div class="flex items-center space-x-4">
            <div class="flex items-center flex-1 min-w-0">
              <span class="font-bold text-fg-darker">{{ t('common.sort').toUpperCase() }}&colon;&nbsp;</span>

              <UListbox
                v-model="selectedSortOption"
                :options="sortOptions"
                button-variant="flat"
                button-classes="italic"
              />
            </div>

            <UButton class="!p-0">
              <UIcon icon="octicon:sort-desc-24" />
            </UButton>
          </div>
          <!-- END "Sort" -->

          <!-- "Filter (by Tags)" -->
          <div class="space-y-2">
            <div class="font-bold text-fg-darker">{{ t('pages.blog.filters.by-tags', 2).toUpperCase() }}</div>

            <div class="flex flex-wrap items-center -mx-1">
              <div v-for="tag in tags" :key="tag.name" class="p-1">
                <UPill :icon="tag.icon" :name="tag.name" :active="tag.active" dim @click="tag.active = !tag.active" />
              </div>
            </div>
          </div>
          <!-- END "Filter (by Tags)" -->
        </div>
      </div>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="col-span-2">
        <!-- "Posts" -->
        <div class="border rounded-lg border-fg-darkest">
          <WPost
            v-for="(post, i) in posts"
            :key="post.id"
            :post="post"
            :class="{
              'border-t border-fg-darkest': i > 0,
            }"
          />
        </div>
        <!-- END "Posts" -->
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, computed, ref } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { type Option, UButton, UIcon, UInput, UListbox, UPill } from '@vcp-web-client/ui';
  import faker from '@faker-js/faker/locale/en';
  import WPost from './_components/WPost.vue';
  import { postTags } from '~/utils/mocks';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Search
  ---------------------------------------------------------------- */
  const search = ref();

  /* ----------------------------------------------------------------
  Sort
  ---------------------------------------------------------------- */
  const sortOptions = computed<Option[]>(() => [
    { value: 'updated', text: t('common.updated') },
    { value: 'relevance', text: t('common.relevance') },
    { value: 'reading-time', text: t('common.reading-time') },
  ]);
  const selectedSortOption = ref(sortOptions.value[0]) as Ref<Option>;

  /* ----------------------------------------------------------------
  READ tags
  ---------------------------------------------------------------- */
  const tags = ref(postTags) as Ref<Model.PostTagR[]>;

  /* ----------------------------------------------------------------
  READ posts
  ---------------------------------------------------------------- */
  const totalPosts = ref(93);

  const posts = ref(
    Array.from({ length: 16 }).map(
      () =>
        ({
          id: faker.datatype.uuid(),
          slug: faker.lorem.slug(8),
          title: faker.lorem.words(8),
          createdAt: faker.date.past(1),
          updatedAt: faker.random.arrayElement([faker.date.past(0.5), undefined]),
          readingTime: faker.datatype.number(60),
          tags: faker.random.arrayElements(postTags, faker.datatype.number(4)),
        } as Model.PostListItemR)
    )
  ) as Ref<Model.PostListItemR[]>;
</script>
