const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./src/**/*.{vue,js,ts,jsx,tsx}'],
  plugins: [require('@cuongvuong-phoenix-com-client/tailwind-plugins')],
  theme: {
    extend: {
      colors: {
        bg: {
          default: '#20182B',
          lighter: '#2b2436',
          lightest: '#362f40',
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
