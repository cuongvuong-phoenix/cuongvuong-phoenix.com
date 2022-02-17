<template>
  <!-- "Regular" -->
  <button
    v-if="!link || (link && disabled)"
    v-bind="$attrs"
    :type="type"
    :disabled="disabled"
    class="disabled:opacity-50"
    :class="classes"
  >
    <slot />
  </button>
  <!-- END "Regular" -->

  <!-- "Link" -->
  <RouterLink
    v-else
    v-bind="$attrs"
    :to="link"
    :replace="linkReplace"
    :active-class="!linkActiveExact ? activeClasses : undefined"
    :exact-active-class="linkActiveExact ? activeClasses : undefined"
    :class="classes"
  >
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
    linkReplace,
    linkActiveExact,
    type = 'button',
    size = 'default',
    unified,
    variant = 'flat',
    color = 'default',
    active,
    disabled,
  } = defineProps<{
    // `link` specific props.
    link?: RouteLocationRaw;
    linkReplace?: boolean;
    linkActiveExact?: boolean;
    // Shared props.
    type?: ButtonType;
    size?: Size;
    unified?: boolean;
    variant?: Variant;
    color?: Color;
    active?: boolean;
    disabled?: boolean;
  }>();

  /* ----------------------------------------------------------------
  Icon + Size classes
  ---------------------------------------------------------------- */
  const unifiedWithSizeClasses = computed<string | undefined>(() => {
    if (!unified) {
      switch (size) {
        case 'sm':
          return 'px-[0.625rem] py-[0.375rem] space-x-[0.375rem] text-sm';
        case 'default':
          return 'px-3 py-2 space-x-2';
        case 'lg':
          return 'px-[0.875rem] py-[0.625rem] space-x-[0.625rem] text-lg';
        default:
          return undefined;
      }
    } else {
      switch (size) {
        case 'sm':
          return 'p-[0.375rem] space-x-[0.375rem] text-sm';
        case 'default':
          return 'p-2 space-x-2';
        case 'lg':
          return 'p-[0.625rem] space-x-[0.625rem] text-lg';
        default:
          return undefined;
      }
    }
  });

  /* ----------------------------------------------------------------
  Variant + Color classes
  ---------------------------------------------------------------- */
  const variantWithColorClasses = computed<string | undefined>(() => {
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
            return undefined;
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
            return undefined;
        }
      }

      case 'full':
        switch (color) {
          case 'default':
            return 'dark:text-bg-dark-default dark:bg-fg-dark-default';
          case 'primary':
            return 'dark:text-bg-dark-default dark:bg-primary-dark-default';
          case 'secondary':
            return 'dark:text-bg-dark-default dark:bg-secondary-dark-default';
          case 'alternative':
            return 'dark:text-bg-dark-default dark:bg-alternative-dark-default';
          case 'success':
            return 'dark:text-bg-dark-default dark:bg-success-dark-default';
          case 'error':
            return 'dark:text-fg-dark-default dark:bg-error-dark-default';
          default:
            return undefined;
        }

      default:
        return undefined;
    }
  });

  /* ----------------------------------------------------------------
  Active classes
  ---------------------------------------------------------------- */
  const activeClasses = computed<string | undefined>(() => {
    switch (variant) {
      case 'flat':
        switch (color) {
          case 'default':
            return 'dark:text-primary-dark-default';
          default:
            return undefined;
        }
      default:
        return undefined;
    }
  });

  /* ----------------------------------------------------------------
  Classes
  ---------------------------------------------------------------- */
  const classes = computed(() => [
    'flex items-center justify-center font-bold transition duration-300',
    unifiedWithSizeClasses.value,
    variantWithColorClasses.value,
    active ? activeClasses.value : undefined,
  ]);
</script>
