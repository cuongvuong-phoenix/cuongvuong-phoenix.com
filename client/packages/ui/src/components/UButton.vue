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
    :active-class="linkActiveType === 'default' ? activeClasses : undefined"
    :exact-active-class="linkActiveType === 'exact' ? activeClasses : undefined"
    :class="classes"
  >
    <slot />
  </RouterLink>
  <!-- END "Link" -->
</template>

<script lang="ts">
  export const buttonTypes = ['button', 'reset', 'submit'] as const;
  export type ButtonType = typeof buttonTypes[number];

  export const buttonVariants = ['flat', 'outlined', 'full'] as const;
  export type ButtonVariant = typeof buttonVariants[number];

  export const buttonLinkActiveTypes = ['default', 'exact', 'none'] as const;
  export type ButtonLinkActiveType = typeof buttonLinkActiveTypes[number];
</script>

<script setup lang="ts">
  import { computed } from 'vue';
  import { type RouteLocationRaw, RouterLink } from 'vue-router';
  import { type Color, type Size, colors, sizes } from '../utils/constants';

  const {
    link,
    linkReplace,
    linkActiveType = 'default',
    type = buttonTypes[0],
    size = sizes[0],
    unified,
    rounded,
    variant = buttonVariants[0],
    color = colors[0],
    active,
    disabled,
  } = defineProps<{
    // `link` specific props.
    link?: RouteLocationRaw;
    linkReplace?: boolean;
    linkActiveType?: ButtonLinkActiveType;
    // Shared props.
    type?: ButtonType;
    size?: Size;
    unified?: boolean;
    rounded?: boolean;
    variant?: ButtonVariant;
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
          return 'px-2.5 py-1.5 space-x-1.5 text-sm';
        case 'default':
          return 'px-3 py-2 space-x-2';
        case 'lg':
          return 'px-3.5 py-2.5 space-x-2.5 text-lg';
        default:
          return undefined;
      }
    } else {
      switch (size) {
        case 'sm':
          return 'p-1.5 space-x-1.5 text-sm';
        case 'default':
          return 'p-2 space-x-2';
        case 'lg':
          return 'p-2.5 space-x-2.5 text-lg';
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
            return `${base} enabled:hover:text-primary-default`;
          case 'primary':
            return `${base} text-primary-default enabled:hover:text-primary-darker`;
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
            return `${base} ${baseOutlined} enabled:hover:text-primary-default`;
          case 'primary':
            return `${base} ${baseOutlined} text-primary-default enabled:hover:text-primary-darker`;
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
            return `${base} text-bg-default bg-fg-default enabled:hover:bg-primary-default`;
          case 'primary':
            return `${base} text-bg-default bg-primary-default enabled:hover:bg-primary-darker`;
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
            return 'text-bg-default bg-primary-default enabled:hover:text-bg-default enabled:hover:bg-primary-darker';
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
    'inline-flex items-center justify-center font-bold focusible-visible',
    unifiedWithSizeClasses.value,
    variantWithRoundedWithColorClasses.value,
    active ? activeClasses.value : undefined,
  ]);
</script>
