const plugin = require('tailwindcss/plugin');

const plugins = {
  lightVariants: ({ addVariant }) => {
    addVariant('light', '.light &');
  },

  whUtilities: ({ addUtilities, theme }) => {
    const spacings = Object.entries(theme('spacing'));

    const wh = spacings.reduce((accum, [key, value]) => {
      return {
        ...accum,
        [`.wh-${key}`]: {
          width: value,
          height: value,
        },
      };
    }, {});

    addUtilities(wh);
  },

  focusibleComponents: ({ addComponents }) => {
    const states = [
      {
        name: 'default',
        state: 'focus',
      },
      {
        name: 'within',
        state: 'focus-within',
      },
    ];

    const focusible = states.reduce(
      (accum, { name, state }) => ({
        ...accum,
        [`.focusible-${name}`]: {
          '@apply transition duration-300': {},

          [`&:${state}`]: {
            '@apply dark:border-primary-dark-darker ring ring-primary-dark-darker/40': {},
          },
        },
      }),
      {}
    );

    addComponents(focusible);
  },
};

module.exports = plugin((helpers) => {
  Object.values(plugins).forEach((fn) => {
    fn(helpers);
  });
});
