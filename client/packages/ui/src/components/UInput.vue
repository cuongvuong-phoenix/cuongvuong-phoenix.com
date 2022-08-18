<template>
  <div>
    <ULabel v-if="label" :label="label" :required="required" />

    <div
      class="border-fg-dark-default placeholder-fg-dark-darker focusible-within flex items-center space-x-2 rounded-lg border px-3 py-2"
    >
      <slot name="prepended" />

      <input
        :id="id"
        type="text"
        :name="name"
        :placeholder="placeholder"
        :value="modelValue"
        class="block w-full"
        @input="onInputDebounced"
      />

      <slot name="appended" />
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useDebounceFn } from '@vueuse/core';
  import ULabel from './ULabel.vue';

  const {
    label,
    required,
    debounceTime = 300,
  } = defineProps<{
    // Label.
    label?: string;
    required?: boolean;
    // Core.
    modelValue?: string;
    debounceTime?: number;
    // Attrs.
    id?: string;
    name?: string;
    placeholder?: string;
  }>();

  const emit = defineEmits<{
    (e: 'update:modelValue', value?: string): void;
  }>();

  const onInputDebounced = useDebounceFn((event: Event) => {
    emit('update:modelValue', (event.target as HTMLInputElement).value);
  }, debounceTime);
</script>

<style lang="postcss">
  input[type='text'] {
    @apply border-none bg-transparent p-0;

    &:focus {
      @apply border-none outline-none ring-0;
    }
  }
</style>
