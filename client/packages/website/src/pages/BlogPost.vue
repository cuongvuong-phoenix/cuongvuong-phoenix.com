<template>
  <div>
    <!-- "Top" -->
    <!-- "Row 1 - Tags" -->
    <div class="flex flex-wrap items-center justify-center gap-2">
      <template v-if="gqlPost && !postLoading">
        <UPill
          v-for="tag in gqlPost.tags"
          :key="tag.id"
          :icon="tag.icon"
          :name="tag.name"
          dim
          @click="
            router.push({
              name: RouteName.BLOG,
              query: {
                tags: [tag.id],
              },
            })
          "
        />
      </template>
      <template v-else-if="postLoading">
        <UPill v-for="num in 4" :key="num" loading />
      </template>
    </div>
    <!-- END "Row 1 - Tags" -->

    <!-- "Row 2 - Title" -->
    <h1
      v-if="gqlPost && !postLoading"
      class="font-serif text-4xl font-bold text-center"
      :class="{ 'mt-6': gqlPost.tags.length > 0 }"
    >
      {{ gqlPost.title }}
    </h1>
    <USkeleton v-else-if="postLoading" type="paragraph" class="mt-6 text-4xl text-center" />
    <!-- END "Row 2 - Title" -->

    <!-- "Row 3" -->
    <div class="flex flex-wrap items-center justify-center mt-6 gap-x-4 gap-y-2 text-fg-darker">
      <template v-if="gqlPost && !postLoading">
        <div class="flex items-center space-x-2">
          <UIcon icon="fluent:calendar-ltr-24-regular" />
          <span>{{ formatDatetime(gqlPost.createdAt, locale) }}</span>
        </div>

        <template v-if="gqlPost.updatedAt">
          <span class="dot-1"></span>
          <div class="flex items-center space-x-2">
            <UIcon icon="fluent:calendar-edit-24-regular" />
            <span>{{ formatDatetime(gqlPost.updatedAt, locale) }}</span>
          </div>
        </template>

        <!-- "(sm) Row 4" -->
        <span class="dot-1 sm:hidden"></span>
        <div class="flex items-center justify-center space-x-2 sm:basis-full">
          <UIcon icon="fluent:book-clock-24-regular" />
          <span
            >{{ gqlPost.readingTime }}
            {{ `${t('common.minute', gqlPost.readingTime)} ${t('common.read')}`.toLowerCase() }}</span
          >
        </div>
        <!-- END "(sm) Row 4" -->
      </template>
      <USkeleton v-else-if="postLoading" type="line" class="w-[24ch]" />
    </div>
    <!-- END "Row 3" -->
    <!-- END "Top" -->

    <!-- "Content" -->
    <div v-if="gqlPost && !postLoading" v-dompurify-html="gqlPost.content" class="mx-auto mt-12 prose"></div>
    <div v-else-if="postLoading" class="mx-auto mt-12 prose">
      <USkeleton type="paragraph" :num-lines="4" />
      <USkeleton type="paragraph" :num-lines="8" />
      <USkeleton type="paragraph" :num-lines="6" />
    </div>
    <!-- END "Content" -->

    <!-- "Toc" -->
    <div v-if="tocContent" class="fixed z-20 p-2 rounded-full bottom-2 right-2 backdrop-blur-xl">
      <UButton
        color="secondary"
        variant="full"
        size="sm"
        unified
        rounded
        class="duration-300"
        :class="{
          'rotate-[360deg]': tocOpenning,
        }"
        @click="tocOpenning = !tocOpenning"
      >
        <UIcon :icon="!tocOpenning ? 'fluent:text-bullet-list-ltr-24-regular' : 'fluent:dismiss-24-regular'" />
      </UButton>

      <!-- "Toc Content" -->
      <Transition
        enter-active-class="duration-300 ease-out"
        enter-from-class="scale-50 opacity-0"
        leave-active-class="duration-200 ease-in"
        leave-to-class="scale-50 opacity-0"
      >
        <div
          v-show="tocOpenning"
          v-dompurify-html="tocContent"
          class="absolute p-2 transition origin-bottom-right border rounded-lg right-[calc(100%-0.5rem)] bottom-[calc(100%-0.5rem)] bg-bg-lighter border-fg-darkest prose overflow-auto sm:w-[calc(100vw-5rem)] xl:w-[18rem] 2xl:w-[20rem] w-[24rem] max-h-[60vh] min-h-[3rem]"
        ></div>
      </Transition>
      <!-- END "Toc Content" -->
    </div>
    <!-- END "Toc" -->
  </div>
</template>

<script setup lang="ts">
  import { reactive, ref, watch } from 'vue';
  import { useRoute, useRouter } from 'vue-router';
  import { useI18n } from 'vue-i18n';
  import { buildVueDompurifyHTMLDirective } from 'vue-dompurify-html';
  import { useQuery, useResult } from '@vue/apollo-composable';
  import { gql } from 'graphql-tag';
  import { parseISO } from 'date-fns';
  import { UButton, UIcon, UPill, USkeleton } from '@cuongvuong-phoenix-com-client/ui';
  import { useHeadStore } from '~/store/head';
  import { formatDatetime } from '~/utils/helpers';
  import { RouteName } from '~/utils/constants';
  import type { PostQuery, PostQueryVariables } from '~/types/graphql';

  const router = useRouter();
  const route = useRoute();
  const { t, locale } = useI18n();
  const headStore = useHeadStore();

  const vDompurifyHtml = buildVueDompurifyHTMLDirective();

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

  const tocContent = ref<string>();

  const gqlPost = useResult(postResult, undefined, (data) => {
    const content = data.post.content.replace(/(<nav.*<\/nav>)/, (_, toc) => {
      tocContent.value = toc;

      return '';
    });

    return {
      ...data.post,
      content,
      createdAt: parseISO(data.post.createdAt),
      updatedAt: data.post.updatedAt ? parseISO(data.post.updatedAt) : undefined,
    };
  });

  /* ----------------------------------------------------------------
  Toc
  ---------------------------------------------------------------- */
  const tocOpenning = ref(false);

  /* ----------------------------------------------------------------
  Head
  ---------------------------------------------------------------- */
  watch(
    [gqlPost, locale],
    ([gqlPostValue]) => {
      if (gqlPostValue) {
        headStore.title = t(`head.${RouteName.BLOG_POST}.title`, { post: gqlPostValue.title });
      }
    },
    { immediate: true }
  );
</script>
