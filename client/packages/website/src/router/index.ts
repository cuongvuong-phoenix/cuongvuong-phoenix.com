import { promiseTimeout } from '@vueuse/core';
import { createRouter, createWebHistory } from 'vue-router';
import { routes } from './routes';

export * from './routes';

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior: async (to, _, savedPosition) => {
    const behavior: ScrollBehavior = 'smooth';

    if (to.meta.transition && to.meta.transitionLeaveDuration) {
      await promiseTimeout(to.meta.transitionLeaveDuration);
    } else {
      await promiseTimeout(250);
    }

    if (savedPosition) {
      return { ...savedPosition, behavior };
    } else if (to.hash) {
      return { el: to.hash, behavior };
    } else {
      return { top: 0, behavior };
    }
  },
});
