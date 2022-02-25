import { createApp, h, provide } from 'vue';
import messages from '@intlify/vite-plugin-vue-i18n/messages';
import { createHead } from '@vueuse/head';
import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client/core';
import { DefaultApolloClient } from '@vue/apollo-composable';
import '@purge-icons/generated';
import NProgress from 'nprogress';
import App from '~/App.vue';
import { router } from '~/router';
import { store } from '~/store';
import { setupI18n } from '~/locales';
import '~/assets/styles/index.css';
import '@cvp-web-client/ui/css';

// Vue Apollo.
const apolloClient = new ApolloClient({
  link: createHttpLink({
    uri: 'http://localhost:7878/api/graphql',
  }),
  cache: new InMemoryCache(),
});

// Vue.
const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient);
  },
  render: () => h(App),
});

// vue-i18n.
const i18n = setupI18n(router, messages);

// @vueuse/head.
const head = createHead();

// NProgress
router.beforeEach(() => {
  NProgress.start();
});
router.afterEach(() => {
  NProgress.done();
});

app.use(router).use(store).use(i18n).use(head).mount('#app');
