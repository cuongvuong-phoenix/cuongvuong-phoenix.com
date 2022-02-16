const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        'background-dark': {
          default: `#20182B`,
        },
        'foreground-dark': {
          default: colors.zinc[100],
          darker: colors.zinc[400],
          darkest: colors.zinc[600],
        },
        'primary-dark': {
          default: colors.amber[400],
          darker: colors.amber[500],
        },
        'secondary-dark': {
          default: colors.pink[400],
        },
        'alternative-dark': {
          default: colors.sky[600],
        },
        'success-dark': {
          default: colors.emerald[400],
        },
        'error-dark': {
          default: colors.rose[500],
        },
      },
    },
  },
};
