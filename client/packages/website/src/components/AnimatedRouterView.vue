<template>
  <RouterView v-slot="{ Component }">
    <template v-if="Component">
      <transition
        mode="out-in"
        enter-active-class="transition-opacity duration-300 ease-out"
        enter-from-class="opacity-0"
        leave-active-class="transition-opacity duration-200 ease-in"
        leave-to-class="opacity-0"
      >
        <KeepAlive>
          <Suspense>
            <component :is="Component" />
          </Suspense>
        </KeepAlive>
      </transition>
    </template>
  </RouterView>
</template>

<script setup lang="ts">
  import { RouterView } from 'vue-router';
</script>
