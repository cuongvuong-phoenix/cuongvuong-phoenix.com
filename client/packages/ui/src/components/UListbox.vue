<template>
  <Listbox
    as="div"
    :model-value="modelValue"
    class="relative"
    @update:model-value="(value) => emit('update:modelValue', value)"
  >
    <!-- "Button" -->
    <ListboxButton
      class="inline-flex w-full items-center justify-center space-x-2 text-left"
      :class="[defaultButtonClasses, buttonClasses]"
    >
      <UIcon v-if="buttonIconPrepended" :icon="buttonIconPrepended" />

      <div
        class="min-w-0 flex-1 truncate"
        :class="{
          'text-fg-darker': !modelValue,
        }"
      >
        <template v-if="modelValue">{{ modelValue.text }}</template>
        <template v-else>{{ buttonPlaceholder }} </template>
      </div>

      <UIcon :icon="buttonIconAppended" />
    </ListboxButton>
    <!-- END "Button" -->

    <!-- "Options" -->
    <Transition
      enter-active-class="transition-[opacity_margin] duration-300 ease-out"
      :enter-from-class="buttonVariant === 'outlined' ? '-mt-6 opacity-0' : '-mt-3 opacity-0'"
      leave-active-class="transition-[opacity_margin] duration-200 ease-in"
      :leave-to-class="buttonVariant === 'outlined' ? '-mt-6 opacity-0' : '-mt-3 opacity-0'"
    >
      <ListboxOptions
        class="bg-bg-lighter border-fg-darkest absolute top-full z-20 mt-1 max-h-64 overflow-auto rounded-lg border py-1 shadow-2xl focus:outline-none"
        :class="{
          'w-full': optionsFullWidth,
        }"
      >
        <ListboxOption
          v-for="option in options"
          :key="option.value"
          v-slot="{ active, selected }"
          :value="option"
          :disabled="option.disabled"
          as="template"
        >
          <li
            class="enabled:hover:bg-bg-default flex cursor-pointer items-center space-x-2 px-3 py-2"
            :class="{
              'bg-bg-default': active,
              'text-primary-default font-medium': selected,
              'opacity-50': option.disabled,
            }"
          >
            <UIcon v-if="option.icon" :icon="option.icon" />
            <div
              class="flex-1"
              :class="{
                'min-w-0 truncate': !optionTextWrap,
              }"
            >
              {{ option.text }}
            </div>
          </li>
        </ListboxOption>
      </ListboxOptions>
    </Transition>
    <!-- END "Options" -->
  </Listbox>
</template>

<script lang="ts">
  export type Value = string | number;

  export interface Option {
    value: Value;
    text: string;
    icon?: string;
    disabled?: boolean;
  }

  export const listboxVariants = ['outlined', 'flat'] as const;
  export type ListboxVariant = typeof listboxVariants[number];
</script>

<script setup lang="ts">
  import { computed } from 'vue';
  import { Listbox, ListboxButton, ListboxOption, ListboxOptions } from '@headlessui/vue';
  import UIcon from './UIcon.vue';

  const {
    modelValue,
    options = [],
    buttonVariant = listboxVariants[0],
    buttonIconAppended = 'fluent:chevron-down-24-regular',
  } = defineProps<{
    modelValue?: Option;
    options?: Option[];
    // Button.
    buttonVariant?: ListboxVariant;
    buttonPlaceholder?: string;
    buttonIconPrepended?: string;
    buttonIconAppended?: string;
    buttonClasses?: string;
    // Option.
    optionsFullWidth?: boolean;
    optionTextWrap?: boolean;
  }>();

  const emit = defineEmits<{
    (e: 'update:modelValue', value?: Option): void;
  }>();

  /* ----------------------------------------------------------------
  Styles
  ---------------------------------------------------------------- */
  const defaultButtonClasses = computed<string | undefined>(() => {
    switch (buttonVariant) {
      case 'flat':
        return 'enabled:hover:text-primary-default focusible-visible';
      case 'outlined':
        return 'px-3 py-2 rounded-lg border border-fg-default enabled:hover:border-primary-default focusible-default';
      default:
        return undefined;
    }
  });
</script>
