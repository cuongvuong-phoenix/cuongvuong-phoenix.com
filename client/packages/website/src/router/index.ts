import { promiseTimeout } from '@vueuse/core';
import { createRouter, createWebHistory } from 'vue-router';
import { routes } from './routes';
import { HeaderHeight } from '~/store/ui';

export * from './routes';

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior: async (to, from, savedPosition) => {
    const behavior: ScrollBehavior = 'smooth';

    if (!to.hash) {
      if (to.meta.transition && to.meta.transitionLeaveDuration) {
        await promiseTimeout(to.meta.transitionLeaveDuration);
      } else {
        await promiseTimeout(250);
      }
    } else {
      await promiseTimeout(0);
    }

    if (to.hash) {
      return {
        el: to.hash,
        top: HeaderHeight.NARROW + 8,
        behavior: !from.name ? behavior : undefined,
      };
    } else if (savedPosition) {
      return { ...savedPosition, behavior };
    } else {
      return { top: 0, behavior };
    }
  },
});
