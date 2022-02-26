<template>
  <div>
    <h1 class="font-serif text-5xl font-bold text-center">
      {{ t('common.blog').toUpperCase() }}
    </h1>

    <!-- "Body" -->
    <div class="grid grid-cols-3 gap-8 mt-16">
      <!-- "Left" -->
      <div>
        <div
          class="sticky space-y-4"
          :style="{
            top: `${uiStore.headerHeight + 16}px`,
          }"
        >
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
            ><span v-if="gqlPostsTotalCount" class="italic"
              >{{ gqlPostsTotalCount }} {{ t('common.post', gqlPostsTotalCount).toLowerCase() }}</span
            >
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
                <UPill :icon="tag.icon" :name="tag.name" :active="tag.active" dim @click="toggleTag(tag.id)" />
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
            v-for="(post, i) in gqlPosts"
            :key="post.id"
            :post="post"
            :class="{
              'border-t border-fg-darkest': i > 0,
            }"
          />
        </div>
        <!-- END "Posts" -->

        <!-- "Pagination" -->
        <div v-if="gqlPostsPageInfo" class="flex items-center mt-8 space-x-8">
          <div class="flex items-center flex-1 min-w-0">
            <UButton
              v-if="gqlPostsPageInfo.hasPreviousPage"
              :link="
                gqlPostsPageInfo.hasPreviousPage
                  ? {
                      name: RouteName.BLOG,
                      params: { locale },
                      query: { last: postsPageSize, before: gqlPostsPageInfo.startCursor, tags: route.query.tags },
                    }
                  : undefined
              "
              link-active-type="none"
            >
              <UIcon icon="fluent:arrow-left-24-regular" />
              <span>{{ t('common.previous') }}</span>
            </UButton>
          </div>

          <!-- TODO: Paginator -->
          <div></div>

          <div class="flex items-center justify-end flex-1 min-w-0">
            <UButton
              v-if="gqlPostsPageInfo.hasNextPage"
              :link="
                gqlPostsPageInfo.hasNextPage
                  ? {
                      name: RouteName.BLOG,
                      params: { locale },
                      query: { first: postsPageSize, after: gqlPostsPageInfo.endCursor, tags: route.query.tags },
                    }
                  : undefined
              "
              link-active-type="none"
            >
              <span>{{ t('common.next') }}</span>
              <UIcon icon="fluent:arrow-right-24-regular" />
            </UButton>
          </div>
        </div>
        <!-- END "Pagination" -->
      </div>
      <!-- END "Right" -->
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, computed, ref, watch } from 'vue';
  import { useRoute, useRouter } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { useQuery, useResult } from '@vue/apollo-composable';
  import { gql } from 'graphql-tag';
  import { parseISO } from 'date-fns';
  import { type Option, UButton, UIcon, UInput, UListbox, UPill } from '@cvp-web-client/ui';
  import WPost from './_components/WPost.vue';
  import { useUiStore } from '~/store/ui';
  import { RouteName } from '~/utils/constants';
  import { type PostsQuery, type PostsQueryVariables, type Tag, type TagsQuery } from '~/types/graphql';

  const router = useRouter();
  const route = useRoute();
  const uiStore = useUiStore();
  const { t, locale } = useI18n();

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
  const {
    result: tagsResult,
    loading: tagsLoading,
    error: tagsError,
  } = useQuery<TagsQuery>(gql`
    query tags {
      tags(paginationParams: { first: 4 }) {
        edges {
          cursor
          node {
            id
            name
            icon
          }
        }
      }
    }
  `);

  const gqlTags = useResult(tagsResult, [], (data) => data.tags.edges.map((edge) => edge.node));

  // Check for activing tags based on `route.query.tags` query params.
  const activeTagIds = ref<string[]>([]);

  watch(
    () => route.query.tags,
    (currTagsRouteQuery, prevTagsRouteQuery) => {
      if (JSON.stringify(currTagsRouteQuery) !== JSON.stringify(prevTagsRouteQuery)) {
        if (Array.isArray(currTagsRouteQuery) && currTagsRouteQuery.length > 0) {
          activeTagIds.value = currTagsRouteQuery as string[];
        } else if (currTagsRouteQuery && !Array.isArray(currTagsRouteQuery)) {
          activeTagIds.value = [currTagsRouteQuery];
        } else {
          activeTagIds.value = [];
        }
      }
    },
    {
      immediate: true,
    }
  );

  // Tags to render.
  interface WTag extends Omit<Tag, 'createdAt' | 'updatedAt'> {
    active?: boolean;
  }
  const tags = ref([]) as Ref<WTag[]>;

  watch(
    [gqlTags, activeTagIds],
    ([gqlTagValues, activeTagIdValues]) => {
      tags.value = gqlTagValues.map((gqlTag) => ({
        ...gqlTag,
        active: activeTagIdValues.includes(gqlTag.id),
      }));
    },
    { immediate: true }
  );

  // Functions.
  function toggleTag(id: string) {
    let tagsQueryParams: string[];

    if (activeTagIds.value) {
      tagsQueryParams = activeTagIds.value.slice() as string[];

      const activedTagIndex = tagsQueryParams.findIndex((activeTagId) => activeTagId === id);

      if (activedTagIndex === -1) {
        tagsQueryParams.push(id);
      } else {
        tagsQueryParams.splice(activedTagIndex, 1);
      }
    } else {
      tagsQueryParams = [id];
    }

    router.push({
      name: RouteName.BLOG,
      params: { locale: locale.value },
      query: {
        tags: tagsQueryParams,
      },
    });
  }

  /* ----------------------------------------------------------------
  READ posts
  ---------------------------------------------------------------- */
  const {
    result: postsResult,
    loading: postsLoading,
    error: postsError,
  } = useQuery<PostsQuery, PostsQueryVariables>(
    gql`
      query posts($tagIds: [UUID!]!, $after: String, $before: String, $first: Int, $last: Int) {
        posts(tagIds: $tagIds, paginationParams: { after: $after, before: $before, first: $first, last: $last }) {
          totalCount
          edges {
            node {
              id
              title
              slug
              readingTime
              createdAt
              updatedAt
              tags {
                id
                name
                icon
              }
            }
          }
          pageInfo {
            hasPreviousPage
            hasNextPage
            startCursor
            endCursor
          }
        }
      }
    `,
    () => ({
      tagIds: activeTagIds.value,
      after: route.query.after as string | undefined,
      before: route.query.before as string | undefined,
      first: route.query.first ? parseInt(route.query.first as string) : undefined,
      last: route.query.last ? parseInt(route.query.last as string) : undefined,
    })
  );

  const gqlPosts = useResult(postsResult, [], (data) =>
    data.posts.edges.map((edge) => {
      return {
        ...edge.node,
        createdAt: parseISO(edge.node.createdAt),
        updatedAt: edge.node.updatedAt ? parseISO(edge.node.updatedAt) : undefined,
      };
    })
  );

  /* ----------------------------------------------------------------
  Pagination
  ---------------------------------------------------------------- */
  const postsPageSize = ref(8);

  const gqlPostsTotalCount = useResult(postsResult, undefined, (data) => data.posts.totalCount);

  const gqlPostsPageInfo = useResult(postsResult, undefined, (data) => data.posts.pageInfo);
</script>
