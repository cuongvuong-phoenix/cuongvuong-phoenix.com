import * as path from 'path';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import purgeIcons from 'vite-plugin-purge-icons';
import vueI18n from '@intlify/vite-plugin-vue-i18n';

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
  ],
});
