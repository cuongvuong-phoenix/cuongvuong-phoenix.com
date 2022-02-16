<template>
    <button>{{ msg }}</button>
</template>

<script setup lang="ts">
    const { msg = 'nothing' } = defineProps<{
        msg?: string
    }>()
</script>