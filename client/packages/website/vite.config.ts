import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import purgeIcons from 'vite-plugin-purge-icons';
import vueI18n from '@intlify/vite-plugin-vue-i18n';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '~': resolve(__dirname, 'src'),
      '@cuongvuong-phoenix-com-client/ui/css': '@cuongvuong-phoenix-com-client/ui/css',
      '@cuongvuong-phoenix-com-client/ui': resolve(__dirname, '../ui/src/index.ts'),
    },
  },
  plugins: [
    vue({
      reactivityTransform: true,
    }),
    purgeIcons(),
    vueI18n({
      include: resolve(dirname(fileURLToPath(import.meta.url)), 'src/locales/translations/**'),
    }),
  ],
});
