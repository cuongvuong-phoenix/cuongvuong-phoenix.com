<template>
  <div>
    <h1 class="font-serif text-5xl font-bold text-center">
      {{ t('common.blog') }}
    </h1>

    <!-- "Body" -->
    <div class="grid grid-cols-3 gap-8 mt-16">
      <!-- "Left" -->
      <div class="sticky top-0 space-y-4">
        <!-- "Search box" -->
        <UInput id="search-box" v-model="search" :placeholder="`${t('common.search')}...`">
          <template #prepended>
            <UIcon icon="fluent:search-24-regular" />
          </template>
        </UInput>
        <!-- END "Search box" -->

        <!-- "Total results count" -->
        <p class="truncate">
          <span class="font-bold text-fg-darker">{{ t('common.total').toUpperCase() }}:&nbsp;</span
          ><span class="italic">{{ t('common.post', { count: 93 }, 3) }}</span>
        </p>
        <!-- END "Total results count" -->

        <!-- "Sort" -->
        <div class="flex items-center space-x-4 overflow-ellipsis">
          <div class="flex items-center flex-1 min-w-0">
            <span class="font-bold text-fg-darker">{{ t('common.sort').toUpperCase() }}:&nbsp;</span>

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
      <!-- END "Left" -->

      <!-- "Right" -->
      <div class="col-span-2">Right</div>
      <!-- END "Right" -->
    </div>
    <!-- END "Body" -->
  </div>
</template>

<script setup lang="ts">
  import { type Ref, computed, ref } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { type Option, UButton, UIcon, UInput, UListbox, UPill } from '@vcp-web-client/ui';

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
  interface Tag {
    name: string;
    icon?: string;
    active?: boolean;
  }

  const tags = ref<Tag[]>([
    { name: 'Vue.JS', icon: 'mdi:vuejs' },
    { name: 'Rust', icon: 'mdi:language-rust' },
    { name: 'Figma', icon: 'feather:figma' },
    { name: 'Linux', icon: 'cib:linux' },
    { name: 'Angular', icon: 'mdi:angularjs' },
    { name: 'React.JS', icon: 'mdi:react' },
    { name: 'SQL', icon: 'fluent:database-24-regular' },
    { name: 'Go', icon: 'mdi:language-go' },
    { name: 'UI/UX Design', icon: 'fluent:design-ideas-24-regular' },
    { name: 'Others' },
  ]);
</script>
