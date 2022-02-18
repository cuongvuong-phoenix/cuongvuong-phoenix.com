<template>
  <label v-bind="$attrs" class="text-lg font-bold">
    <span>{{ label }}</span
    >&nbsp;<span v-if="required" class="text-secondary-dark-default">*</span>
  </label>

  <p v-if="guide" class="text-sm font-light">{{ guide }}</p>
</template>

<script setup lang="ts">
  const { label, guide, required } = defineProps<{
    label: string;
    required?: boolean;
    guide?: string;
  }>();
</script>
