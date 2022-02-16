<template>
  <!-- "Regular" -->
  <button
    v-if="!link || (link && disabled)"
    v-bind="$attrs"
    :type="buttonType"
    :disabled="disabled"
    class="flex items-center justify-center font-bold"
    :class="classes"
  >
    <slot />
  </button>
  <!-- END "Regular" -->

  <!-- "Link" -->
  <RouterLink v-else v-bind="$attrs" :to="link" class="flex items-center justify-center font-bold" :class="classes">
    <slot />
  </RouterLink>
  <!-- END "Link" -->
</template>

<script lang="ts">
  export const variants = ['flat', 'outlined', 'full'] as const;
  export type Variant = typeof variants[number];

  export const buttonTypes = ['button', 'reset', 'submit'] as const;
  export type ButtonType = typeof buttonTypes[number];
</script>

<script setup lang="ts">
  import { computed } from 'vue';
  import { type RouteLocationRaw, RouterLink } from 'vue-router';
  import { type Color, type Size } from '../utils/constants';

  const {
    link,
    buttonType = 'button',
    size = 'default',
    unified = false,
    variant = 'flat',
    color = 'default',
    disabled = false,
  } = defineProps<{
    link?: RouteLocationRaw;
    buttonType?: ButtonType;
    size?: Size;
    unified?: boolean;
    variant?: Variant;
    color?: Color;
    disabled?: boolean;
  }>();

  /* ----------------------------------------------------------------
  Icon + Size classes
  ---------------------------------------------------------------- */
  const unifiedWithSizeClasses = computed<string>(() => {
    if (!unified) {
      switch (size) {
        case 'sm':
          return 'px-[10px] py-[6px] space-x-[6px] text-sm';
        case 'default':
          return 'px-3 py-2 space-x-2';
        case 'lg':
          return 'px-[14px] py-[10px] space-x-[10px] text-lg';
        default:
          return '';
      }
    } else {
      switch (size) {
        case 'sm':
          return 'p-[6px] space-x-[6px] text-sm';
        case 'default':
          return 'p-2 space-x-2';
        case 'lg':
          return 'p-[10px] space-x-[10px] text-lg';
        default:
          return '';
      }
    }
  });

  /* ----------------------------------------------------------------
  Variant + Color classes
  ---------------------------------------------------------------- */
  const variantWithColorClasses = computed<string>(() => {
    switch (variant) {
      case 'flat':
        switch (color) {
          case 'default':
            return 'dark:hover:text-primary-dark-darker';
          case 'primary':
            return 'dark:text-primary-dark-default';
          case 'secondary':
            return 'dark:text-secondary-dark-default';
          case 'alternative':
            return 'dark:text-alternative-dark-default';
          case 'success':
            return 'dark:text-success-dark-default';
          case 'error':
            return 'dark:text-error-dark-default';
          default:
            return '';
        }
      case 'outlined': {
        const base = 'border border-current rounded-lg';

        switch (color) {
          case 'default':
            return base;
          case 'primary':
            return `${base} dark:text-primary-dark-default`;
          case 'secondary':
            return `${base} dark:text-secondary-dark-default`;
          case 'alternative':
            return `${base} dark:text-alternative-dark-default`;
          case 'success':
            return `${base} dark:text-success-dark-default`;
          case 'error':
            return `${base} dark:text-error-dark-default`;
          default:
            return '';
        }
      }

      case 'full':
        switch (color) {
          case 'default':
            return 'dark:text-background-dark-default dark:bg-foreground-dark-default';
          case 'primary':
            return 'dark:text-background-dark-default dark:bg-primary-dark-default';
          case 'secondary':
            return 'dark:text-background-dark-default dark:bg-secondary-dark-default';
          case 'alternative':
            return 'dark:text-background-dark-default dark:bg-alternative-dark-default';
          case 'success':
            return 'dark:text-background-dark-default dark:bg-success-dark-default';
          case 'error':
            return 'dark:text-foreground-dark-default dark:bg-error-dark-default';
          default:
            return '';
        }

      default:
        return '';
    }
  });

  /* ----------------------------------------------------------------
  Disabled classes
  ---------------------------------------------------------------- */
  const disabledClasses = computed<string>(() => (disabled ? 'opacity-50' : ''));

  /* ----------------------------------------------------------------
  Classes
  ---------------------------------------------------------------- */
  const classes = computed(
    () => `${unifiedWithSizeClasses.value} ${variantWithColorClasses.value} ${disabledClasses.value}`
  );
</script>

<style scoped></style>
