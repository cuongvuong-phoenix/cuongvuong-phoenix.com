<template>
  <CButton :msg="`${interval}`" />
</template>

<script setup lang="ts">
  import { useInterval } from '@vueuse/core';
  import { CButton } from '@vcp-web-client/ui';

  const interval = useInterval(1000);
</script>
