<template>
  <div>
    <ULabel v-if="label" :label="label" :required="required" />

    <div
      class="flex items-center px-3 py-2 space-x-2 border rounded-lg border-fg-dark-default placeholder-fg-dark-darker focusible-within"
    >
      <slot name="prepended" />

      <input
        :id="id"
        type="text"
        :name="name"
        :placeholder="placeholder"
        :value="modelValue"
        class="block w-full"
        @input="event => emit('update:modelValue', (event.target as HTMLInputElement).value)"
      />

      <slot name="appended" />
    </div>
  </div>
</template>

<script setup lang="ts">
  import ULabel from './ULabel.vue';

  const { label, required } = defineProps<{
    // Label.
    label?: string;
    required?: boolean;
    // Core.
    modelValue?: string;
    // Attrs.
    id?: string;
    name?: string;
    placeholder?: string;
  }>();

  const emit = defineEmits<{
    (e: 'update:modelValue', value?: string): void;
  }>();
</script>

<style lang="postcss">
  input[type='text'] {
    @apply p-0 bg-transparent border-none;

    &:focus {
      @apply outline-none ring-0 border-none;
    }
  }
</style>
