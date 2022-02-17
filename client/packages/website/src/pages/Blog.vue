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
        <div>Input</div>
        <!-- END "Search box" -->

        <!-- "Total results count" -->
        <p>
          <span class="font-bold dark:text-fg-dark-darker">{{ t('common.total').toUpperCase() }}:</span>&nbsp;<span
            class="italic"
            >{{ t('common.post', { count: 93 }, 3) }}</span
          >
        </p>
        <!-- END "Total results count" -->

        <!-- "Sort" -->
        <div class="flex items-center space-x-4">
          <p class="flex-1">
            <span class="font-bold dark:text-fg-dark-darker">{{ t('common.sort').toUpperCase() }}:</span>&nbsp;<span
              class="italic"
              >{{ t('common.updated') }}</span
            >
          </p>

          <UButton class="!p-0">
            <WIcon icon="octicon:sort-desc-24" />
          </UButton>
        </div>
        <!-- END "Sort" -->

        <!-- "Filter (by Tags)" -->
        <div class="space-y-2">
          <div class="font-bold dark:text-fg-dark-darker">{{ t('pages.blog.filters.by-tags', 2).toUpperCase() }}</div>

          <div class="flex flex-wrap items-center -mx-1">
            <div v-for="tag in tags" :key="tag.name" class="p-1">
              <div class="border rounded-[2rem] border-current px-[0.625rem] py-[0.375rem] space-x-1 flex items-center">
                <WIcon v-if="tag.icon" :icon="tag.icon" />
                <span>{{ tag.name }}</span>
              </div>
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
  import { computed, ref } from 'vue';
  import { useI18n } from 'vue-i18n';
  import { useIntervalFn } from '@vueuse/core';
  import { UButton } from '@vcp-web-client/ui';
  import WIcon from '~/components/WIcon.vue';

  const { t } = useI18n();

  /* ----------------------------------------------------------------
  READ tags
  ---------------------------------------------------------------- */
  interface Tag {
    name: string;
    icon?: string;
  }

  const tags = computed<Tag[]>(() => [
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
