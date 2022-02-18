const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: 'class',
  plugins: [require('@vcp-web-client/tailwind-plugins')],
  theme: {
    extend: {
      colors: {
        'bg-dark': {
          default: `#20182B`,
        },
        'fg-dark': {
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
