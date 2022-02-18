import { createApp } from 'vue';
import { createHead } from '@vueuse/head';
import messages from '@intlify/vite-plugin-vue-i18n/messages';
import '@purge-icons/generated';
import NProgress from 'nprogress';
import App from '~/App.vue';
import { router } from '~/router';
import { store } from '~/store';
import { setupI18n } from '~/locales';
import '~/assets/styles/index.css';
import '@vcp-web-client/ui/css';

const app = createApp(App);

// NProgress
router.beforeEach(() => {
  NProgress.start();
});
router.afterEach(() => {
  NProgress.done();
});

// vue-i18n.
const i18n = setupI18n(router, messages);

// @vueuse/head.
const head = createHead();

app.use(router).use(store).use(i18n).use(head).mount('#app');
