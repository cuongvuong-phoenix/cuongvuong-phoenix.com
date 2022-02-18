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
};

module.exports = plugin((helpers) => {
  Object.values(plugins).forEach((fn) => {
    fn(helpers);
  });
});
