<template>
  <RouterView v-slot="{ Component, route }">
    <template v-if="Component">
      <transition mode="out-in" :name="route.meta.transition || 'fade'">
        <KeepAlive v-if="keepAlive">
          <Suspense>
            <component :is="Component" />
          </Suspense>
        </KeepAlive>

        <component :is="Component" v-else />
      </transition>
    </template>
  </RouterView>
</template>

<script setup lang="ts">
  import { RouterView } from 'vue-router';

  const { keepAlive = true } = defineProps<{
    keepAlive?: boolean;
  }>();
</script>

<style lang="postcss">
  .fade-enter-active {
    @apply transition-opacity duration-300 ease-out;
  }

  .fade-leave-active {
    @apply transition-opacity duration-200 ease-in;
  }

  .fade-enter-from,
  .fade-leave-to {
    @apply opacity-0;
  }
</style>
