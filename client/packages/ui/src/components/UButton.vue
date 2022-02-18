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

  <!-- "External Link" -->
  <a v-else-if="isExternalLink" v-bind="$attrs" :href="(link as string)" target="_blank" :class="classes">
    <slot />
  </a>
  <!-- END "External Link" -->

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
    rounded,
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
    rounded?: boolean;
    variant?: Variant;
    color?: Color;
    active?: boolean;
    disabled?: boolean;
  }>();

  /* ----------------------------------------------------------------
  Link
  ---------------------------------------------------------------- */
  const isExternalLink = computed(() => typeof link === 'string' && link.startsWith('http'));

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
  const variantWithRoundedWithColorClasses = computed<string | undefined>(() => {
    let base;

    if (!rounded) {
      base = 'rounded-lg';
    } else {
      base = 'rounded-full';
    }

    switch (variant) {
      case 'flat':
        switch (color) {
          case 'default':
            return `${base} hover:text-primary-darker`;
          case 'primary':
            return `${base} text-primary-default`;
          case 'secondary':
            return `${base} text-secondary-default`;
          case 'alternative':
            return `${base} text-alternative-default`;
          case 'success':
            return `${base} text-success-default`;
          case 'error':
            return `${base} text-error-default`;
          default:
            return undefined;
        }
      case 'outlined': {
        const baseOutlined = 'border border-current';

        switch (color) {
          case 'default':
            return `${base} ${baseOutlined} hover:text-primary-darker`;
          case 'primary':
            return `${base} ${baseOutlined} text-primary-default`;
          case 'secondary':
            return `${base} ${baseOutlined} text-secondary-default`;
          case 'alternative':
            return `${base} ${baseOutlined} text-alternative-default`;
          case 'success':
            return `${base} ${baseOutlined} text-success-default`;
          case 'error':
            return `${base} ${baseOutlined} text-error-default`;
          default:
            return undefined;
        }
      }

      case 'full': {
        switch (color) {
          case 'default':
            return `${base} text-bg-default bg-fg-default hover:bg-primary-darker`;
          case 'primary':
            return `${base} text-bg-default bg-primary-default hover:bg-primary-darker`;
          case 'secondary':
            return `${base} text-bg-default bg-secondary-default`;
          case 'alternative':
            return `${base} text-bg-default bg-alternative-default`;
          case 'success':
            return `${base} text-bg-default bg-success-default`;
          case 'error':
            return `${base} text-fg-default bg-error-default`;
          default:
            return undefined;
        }
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
            return 'text-primary-default';
          default:
            return undefined;
        }
      case 'outlined':
        switch (color) {
          case 'default':
            return 'text-bg-default bg-primary-default hover:text-bg-default hover:bg-primary-darker';
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
    variantWithRoundedWithColorClasses.value,
    active ? activeClasses.value : undefined,
  ]);
</script>
