const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./src/**/*.{vue,js,ts,jsx,tsx}'],
  plugins: [require('@vcp-web-client/tailwind-plugins')],
  corePlugins: {
    preflight: false,
  },
  theme: {
    extend: {
      colors: {
        bg: {
          default: `#20182B`,
        },
        fg: {
          default: colors.zinc[100],
          darker: colors.zinc[400],
          darkest: colors.zinc[600],
        },
        primary: {
          default: colors.amber[400],
          darker: colors.amber[500],
        },
        secondary: {
          default: colors.pink[400],
        },
        alternative: {
          default: colors.sky[600],
        },
        success: {
          default: colors.emerald[400],
        },
        error: {
          default: colors.rose[500],
        },
      },

      borderRadius: {
        '4xl': '2rem',
      },
    },
  },
};
