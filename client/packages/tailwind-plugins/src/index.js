const plugin = require('tailwindcss/plugin');

const plugins = {
  /* ----------------------------------------------------------------
  Variants
  ---------------------------------------------------------------- */
  lightVariants: ({ addVariant }) => {
    addVariant('light', '.light &');
  },

  /* ----------------------------------------------------------------
  Utilities
  ---------------------------------------------------------------- */
  whUtilities: ({ addUtilities, matchUtilities, theme }) => {
    const spacings = Object.entries(theme('spacing'));

    const wh = spacings.reduce(
      (accum, [key, value]) => ({
        ...accum,

        [`.wh-${key}`]: {
          width: value,
          height: value,
        },
        [`.min-wh-${key}`]: {
          'min-width': value,
          'min-height': value,
        },

        [`.max-wh-${key}`]: {
          'max-width': value,
          'max-height': value,
        },
      }),
      {}
    );

    addUtilities(wh);

    matchUtilities({
      '.wh': (value) => ({ width: value, height: value }),
      '.min-wh': (value) => ({ 'min-width': value, 'max-height': value }),
      '.max-wh': (value) => ({ 'min-width': value, 'max-height': value }),
    });
  },

  /* ----------------------------------------------------------------
  Components
  ---------------------------------------------------------------- */
  focusibleComponents: ({ addComponents }) => {
    const states = [
      { name: 'default', state: 'focus' },
      { name: 'within', state: 'focus-within' },
      { name: 'visible', state: 'focus-visible' },
    ];

    const focusible = states.reduce(
      (accum, { name, state }) => ({
        ...accum,
        [`.focusible-${name}`]: {
          '@apply outline-none transition duration-300': {},

          [`&:${state}`]: {
            '@apply border-primary-darker ring ring-primary-darker/40': {},
          },
        },
      }),
      {}
    );

    addComponents(focusible);
  },

  dotComponents: ({ addComponents, theme }) => {
    const spacings = Object.entries(theme('spacing'));
    const dots = spacings.reduce(
      (accum, [key, value]) => ({
        ...accum,
        [`.dot-${key}`]: {
          'width': value,
          'height': value,
          '@apply bg-current rounded-full': {},
        },
      }),
      {}
    );

    addComponents(dots);
  },
};

module.exports = plugin((helpers) => {
  Object.values(plugins).forEach((fn) => {
    fn(helpers);
  });
});
