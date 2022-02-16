<template>
  <div>Blog page</div>
</template>

<script setup lang="ts">
  import { useUiStore } from '~/store/ui';

  const uiStore = useUiStore();

  uiStore.setHeaderFloat(true);
</script>
