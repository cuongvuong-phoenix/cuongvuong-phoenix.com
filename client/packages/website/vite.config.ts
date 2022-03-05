import * as path from 'path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import purgeIcons from 'vite-plugin-purge-icons';
import vueI18n from '@intlify/vite-plugin-vue-i18n';
import markdown from 'vite-plugin-md';
import MarkdownItAttrs from 'markdown-it-attrs';
import MarkdownItAnchor from 'markdown-it-anchor';
import MarkdownItEmoji from 'markdown-it-emoji';
import MarkdownItSub from 'markdown-it-sub';
import MarkdownItSup from 'markdown-it-sup';
import MarkdownItIns from 'markdown-it-ins';
import MarkdownItMark from 'markdown-it-mark';
import MarkdownItFootnote from 'markdown-it-footnote';
import MarkdownItAbbr from 'markdown-it-abbr';
import MarkdownItToc from 'markdown-it-toc-done-right';
import MarkdownItPrism from 'markdown-it-prism';
import MarkdownItPrismBackticks from 'markdown-it-prism-backticks';
import slugify from 'slugify';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '~': path.resolve(__dirname, 'src'),
      'vue-i18n': 'vue-i18n/dist/vue-i18n.runtime.esm-bundler.js',
      '@cvp-web-client/ui/css': path.resolve(__dirname, '../ui/dist/assets/index.css'),
      '@cvp-web-client/ui': path.resolve(__dirname, '../ui/src/index.ts'),
    },
  },
  plugins: [
    vue({
      include: [/\.vue$/, /\.md$/],
      reactivityTransform: true,
    }),
    purgeIcons(),
    vueI18n({
      include: path.resolve(__dirname, 'src/locales/translations/**'),
    }),
    markdown({
      wrapperClasses: 'prose dark:prose-invert mx-auto',
      markdownItSetup(md) {
        md.use(MarkdownItAttrs, {
          // Security (https://github.com/arve0/markdown-it-attrs#security).
          allowedAttributes: ['id', 'class'],
        });
        md.use(MarkdownItAnchor, {
          slugify: (s) => slugify(s, { lower: true }),
          permalink: MarkdownItAnchor.permalink.ariaHidden({
            placement: 'before',
            symbol:
              '<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M10.59 13.41c.41.39.41 1.03 0 1.42c-.39.39-1.03.39-1.42 0a5.003 5.003 0 0 1 0-7.07l3.54-3.54a5.003 5.003 0 0 1 7.07 0a5.003 5.003 0 0 1 0 7.07l-1.49 1.49c.01-.82-.12-1.64-.4-2.42l.47-.48a2.982 2.982 0 0 0 0-4.24a2.982 2.982 0 0 0-4.24 0l-3.53 3.53a2.982 2.982 0 0 0 0 4.24m2.82-4.24c.39-.39 1.03-.39 1.42 0a5.003 5.003 0 0 1 0 7.07l-3.54 3.54a5.003 5.003 0 0 1-7.07 0a5.003 5.003 0 0 1 0-7.07l1.49-1.49c-.01.82.12 1.64.4 2.43l-.47.47a2.982 2.982 0 0 0 0 4.24a2.982 2.982 0 0 0 4.24 0l3.53-3.53a2.982 2.982 0 0 0 0-4.24a.973.973 0 0 1 0-1.42Z"/></svg>',
          }),
        });
        md.use(MarkdownItEmoji);
        md.use(MarkdownItSub);
        md.use(MarkdownItSup);
        md.use(MarkdownItIns);
        md.use(MarkdownItMark);
        md.use(MarkdownItFootnote);
        md.use(MarkdownItAbbr);
        md.use(MarkdownItToc, {
          slugify: (s) => slugify(s, { lower: true }),
        });
        md.use(MarkdownItPrism, { defaultLanguage: 'markup' });
        md.use(MarkdownItPrismBackticks);
      },
    }),
  ],
});
