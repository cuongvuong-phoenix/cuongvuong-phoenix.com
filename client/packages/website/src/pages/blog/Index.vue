<template>
  <div>
    <h1 class="text-center font-serif text-5xl font-bold">
      {{ t('common.blog').toUpperCase() }}
    </h1>

    <!-- "Body" -->
    <div class="mt-12 grid grid-cols-3 gap-8 lg:grid-cols-1">
      <!-- "Left" -->
      <div>
        <div
          class="sticky space-y-4"
          :style="{
            top: `${headerHeight + 16}px`,
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
          <div class="truncate">
            <span class="text-fg-darker font-bold">{{ t('common.total').toUpperCase() }}&colon;&nbsp;</span
            ><span v-if="gqlPostsTotalCount !== undefined && !postsLoading" class="italic"
              >{{ gqlPostsTotalCount }} {{ t('common.post', gqlPostsTotalCount).toLowerCase() }}</span
            >
            <USkeleton v-else-if="postsLoading" type="line" class="w-[8ch]" />
          </div>
          <!-- END "Total results count" -->

          <!-- "Sort" -->
          <!-- <div class="flex items-center space-x-4">
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
          </div> -->
          <!-- END "Sort" -->

          <!-- "Filter (by Tags)" -->
          <div class="space-y-2">
            <div class="text-fg-darker font-bold">{{ t('pages.blog.filters.by-tags', 2).toUpperCase() }}</div>

            <div class="flex flex-wrap items-center gap-2">
              <template v-if="tags.length > 0 && !tagsLoading">
                <UPill
                  v-for="tag in tags"
                  :key="tag.name"
                  :icon="tag.icon"
                  :name="tag.name"
                  :active="tag.active"
                  dim
                  @click="toggleTag(tag.id)"
                />
              </template>
              <template v-else-if="tagsLoading">
                <UPill v-for="num in 8" :key="num" loading />
              </template>
            </div>
          </div>
          <!-- END "Filter (by Tags)" -->
        </div>
      </div>
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="col-span-2 lg:col-auto">
        <!-- "Posts" -->
        <div class="border-fg-darkest overflow-hidden rounded-lg border">
          <template v-if="gqlPosts.length > 0 && !postsLoading">
            <WPost
              v-for="(post, i) in gqlPosts"
              :key="post.id"
              :post="post"
              :class="{
                'border-fg-darkest border-t': i > 0,
              }"
            />
          </template>
          <template v-else-if="gqlPosts.length === 0 && !postsLoading">
            <div class="text-fg-darkest flex flex-col items-center space-y-4 px-12 py-8 text-center text-2xl">
              <UIcon icon="fluent:document-bullet-list-off-24-regular" />
              <span>{{ t('placeholders.no-post').toUpperCase() }}</span>
            </div>
          </template>
          <template v-else-if="postsLoading">
            <WPost
              v-for="(num, i) in postsPageSize"
              :key="num"
              loading
              :class="{ 'border-fg-darkest border-t': i > 0 }"
            />
          </template>
        </div>
        <!-- END "Posts" -->

        <!-- "Pagination" -->
        <div v-if="gqlPostsPageInfo" class="mt-8 flex items-center space-x-8">
          <div class="flex min-w-0 flex-1 items-center">
            <UButton
              v-if="gqlPostsPageInfo.hasPreviousPage"
              :link="
                gqlPostsPageInfo.hasPreviousPage
                  ? {
                      name: RouteName.BLOG,
                      query: {
                        last: postsPageSize,
                        before: gqlPostsPageInfo.startCursor,
                        search: route.query.search,
                        tags: route.query.tags,
                      },
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

          <div class="flex min-w-0 flex-1 items-center justify-end">
            <UButton
              v-if="gqlPostsPageInfo.hasNextPage"
              :link="
                gqlPostsPageInfo.hasNextPage
                  ? {
                      name: RouteName.BLOG,
                      query: {
                        first: postsPageSize,
                        after: gqlPostsPageInfo.endCursor,
                        search: route.query.search,
                        tags: route.query.tags,
                      },
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
  import { storeToRefs } from 'pinia';
  import { useI18n } from 'vue-i18n';
  import { useRouteQuery } from '@vueuse/router';
  import { useQuery } from '@vue/apollo-composable';
  import { gql } from 'graphql-tag';
  import { parseISO } from 'date-fns';
  import { UButton, UIcon, UInput, UPill, USkeleton } from '@cuongvuong-phoenix-com-client/ui';
  import WPost from './_components/WPost.vue';
  import { useUiStore } from '~/store/ui';
  import { RouteName } from '~/utils/constants';
  import { type PostsQuery, type PostsQueryVariables, type TagsQuery } from '~/types/graphql';

  const router = useRouter();
  const route = useRoute();
  const uiStore = useUiStore();
  const { headerHeight } = storeToRefs(uiStore);
  const { t } = useI18n();

  /* ----------------------------------------------------------------
  Search
  ---------------------------------------------------------------- */
  const search = useRouteQuery('search', '', { route, router });

  /* ----------------------------------------------------------------
  TODO: Sort
  ---------------------------------------------------------------- */
  // const sortOptions = computed<Option[]>(() => [
  //   { value: 'updated', text: t('common.updated') },
  //   { value: 'relevance', text: t('common.relevance') },
  //   { value: 'reading-time', text: t('common.reading-time') },
  // ]);
  // const selectedSortOption = ref(sortOptions.value[0]) as Ref<Option>;

  /* ----------------------------------------------------------------
  READ tags
  ---------------------------------------------------------------- */
  const {
    result: tagsResult,
    loading: tagsLoading,
    error: tagsError,
  } = useQuery<TagsQuery>(gql`
    query tags {
      tags(paginationParams: {}) {
        nodes {
          id
          name
          icon
        }
      }
    }
  `);

  const gqlTags = computed(() => tagsResult.value?.tags.nodes ?? []);

  // Check for activing tags based on `route.query.tags` query params.
  const activeTagIds = ref<number[]>([]);

  watch(
    () => route.query.tags,
    (currTagsRouteQuery, prevTagsRouteQuery) => {
      if (JSON.stringify(currTagsRouteQuery) !== JSON.stringify(prevTagsRouteQuery)) {
        if (Array.isArray(currTagsRouteQuery) && currTagsRouteQuery.length > 0) {
          activeTagIds.value = currTagsRouteQuery.map((query) => parseInt(query as string));
        } else if (currTagsRouteQuery && !Array.isArray(currTagsRouteQuery)) {
          activeTagIds.value = [parseInt(currTagsRouteQuery)];
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
  type Tag = TagsQuery['tags']['nodes'][number];
  interface WTag extends Tag {
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
  async function toggleTag(id: number) {
    const newActiveTagIds = activeTagIds.value.slice();

    const currentTagActiveIndex = newActiveTagIds.findIndex((activeTagId) => activeTagId === id);

    if (currentTagActiveIndex === -1) {
      newActiveTagIds.push(id);
    } else {
      newActiveTagIds.splice(currentTagActiveIndex, 1);
    }

    await router.push({
      name: RouteName.BLOG,
      query: {
        search: route.query.search,
        tags: newActiveTagIds,
      },
    });
  }

  /* ----------------------------------------------------------------
  READ posts
  ---------------------------------------------------------------- */
  const postsPageSize = ref(8);

  const {
    result: postsResult,
    loading: postsLoading,
    error: postsError,
  } = useQuery<PostsQuery, PostsQueryVariables>(
    gql`
      query posts($after: String, $before: String, $first: Int, $last: Int, $search: String!, $tagIds: [Int!]!) {
        posts(
          paginationParams: { after: $after, before: $before, first: $first, last: $last }
          search: $search
          tagIds: $tagIds
        ) {
          totalCount
          nodes {
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
          pageInfo {
            hasPreviousPage
            hasNextPage
            startCursor
            endCursor
          }
        }
      }
    `,
    () => {
      let first = route.query.first ? parseInt(route.query.first as string) : undefined;
      const last = route.query.last ? parseInt(route.query.last as string) : undefined;

      if (first === undefined && last === undefined) {
        first = postsPageSize.value;
      }

      return {
        after: route.query.after as string | undefined,
        before: route.query.before as string | undefined,
        first,
        last,
        tagIds: activeTagIds.value,
        search: search.value,
      };
    }
  );

  const gqlPosts = computed(() => {
    if (!postsResult.value) {
      return [];
    }

    return postsResult.value.posts.nodes.map((node) => {
      return {
        ...node,
        createdAt: parseISO(node.createdAt),
        updatedAt: node.updatedAt ? parseISO(node.updatedAt) : undefined,
      };
    });
  });

  /* ----------------------------------------------------------------
  Pagination
  ---------------------------------------------------------------- */
  const gqlPostsTotalCount = computed(() => postsResult.value?.posts.totalCount);

  const gqlPostsPageInfo = computed(() => postsResult.value?.posts.pageInfo);
</script>
