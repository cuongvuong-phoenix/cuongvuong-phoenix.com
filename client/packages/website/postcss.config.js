module.exports = {
  plugins: {
    'postcss-at-rules-variables': {},
    'postcss-import': {},
    'postcss-calc': {},
    'postcss-each': {},
    'postcss-for': {},
    'tailwindcss/nesting': {},
    'tailwindcss': {},
    'autoprefixer': {},
    'postcss-combine-duplicated-selectors': {
      removeDuplicatedProperties: true,
    },
  },
};
