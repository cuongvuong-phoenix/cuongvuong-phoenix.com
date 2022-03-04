const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  safelist: [
    'prose',
    'dark:prose-light',
    'mx-auto',
    // Custom.
    'underline',
  ],
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/line-clamp'),
    require('@cvp-web-client/tailwind-plugins'),
  ],
  corePlugins: {
    preflight: false,
    container: false,
  },
  theme: {
    /* ----------------------------------------------------------------
    Overriding
    ---------------------------------------------------------------- */
    screens: {
      '3xl': { max: '1920px' },
      '2xl': { max: '1536px' },
      'xl': { max: '1280px' },
      'lg': { max: '1024px' },
      'md': { max: '768px' },
      'sm': { max: '640px' },
    },

    fontFamily: {
      sans: "'IBM Plex Sans', sans-serif",
      serif: "'IBM Plex Serif', serif",
      mono: "'JetBrains Mono', monospace",
    },

    /* ----------------------------------------------------------------
    Extending
    ---------------------------------------------------------------- */
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

      typography: ({ theme }) => ({
        DEFAULT: {
          css: {
            '--tw-prose-body': theme('colors.fg.default'),
            '--tw-prose-headings': theme('colors.primary.default'),
            '--tw-prose-lead': theme('colors.fg.darker'),
            '--tw-prose-links': theme('colors.primary.default'),
            '--tw-prose-bold': theme('colors.fg.default'),
            '--tw-prose-counters': theme('colors.fg.darker'),
            '--tw-prose-bullets': theme('colors.fg.darkest'),
            '--tw-prose-hr': theme('colors.fg.darkest'),
            '--tw-prose-quotes': theme('colors.secondary.default'),
            '--tw-prose-quote-borders': theme('colors.fg.darkest'),
            '--tw-prose-captions': theme('colors.fg.darker'),
            '--tw-prose-code': theme('colors.primary.darker'),
            '--tw-prose-pre-code': theme('colors.zinc[300]'),
            '--tw-prose-pre-bg': theme('colors.bg.lighter'),
            '--tw-prose-th-borders': theme('colors.fg.darkest'),
            '--tw-prose-td-borders': theme('colors.zinc[700]'),
            // TODO: Light theme (invert).
            '--tw-prose-invert-body': theme('colors.zinc[700]'),
            '--tw-prose-invert-headings': theme('colors.zinc[900]'),
            '--tw-prose-invert-lead': theme('colors.zinc[600]'),
            '--tw-prose-invert-links': theme('colors.zinc[900]'),
            '--tw-prose-invert-bold': theme('colors.zinc[900]'),
            '--tw-prose-invert-counters': theme('colors.zinc[500]'),
            '--tw-prose-invert-bullets': theme('colors.zinc[300]'),
            '--tw-prose-invert-hr': theme('colors.zinc[200]'),
            '--tw-prose-invert-quotes': theme('colors.zinc[900]'),
            '--tw-prose-invert-quote-borders': theme('colors.zinc[200]'),
            '--tw-prose-invert-captions': theme('colors.zinc[500]'),
            '--tw-prose-invert-code': theme('colors.zinc[900]'),
            '--tw-prose-invert-pre-code': theme('colors.zinc[200]'),
            '--tw-prose-invert-pre-bg': theme('colors.zinc[800]'),
            '--tw-prose-invert-th-borders': theme('colors.zinc[300]'),
            '--tw-prose-invert-td-borders': theme('colors.zinc[200]'),
            // Elements.
            'a': {
              'font-style': 'italic',
              'text-decoration-line': 'none',
              'text-underline-offset': '1px',

              '&:hover': {
                'text-decoration-line': 'underline',
              },

              'code': {
                color: 'var(--tw-prose-code)',
              },
            },
            'code::before': false,
            'code::after': false,
          },
        },
      }),
    },
  },
};
