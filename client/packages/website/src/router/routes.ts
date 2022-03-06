import { type RouteRecordRaw } from 'vue-router';
import Home from '~/pages/Home.vue';
import { RouteName } from '~/utils/constants';

export const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: RouteName.HOME,
    component: Home,
  },
  {
    path: '/about',
    name: RouteName.ABOUT,
    component: () => import('~/pages/About.vue'),
  },
  {
    path: '/blog',
    name: RouteName.BLOG,
    component: () => import('~/pages/blog/Index.vue'),
  },
  {
    path: '/blog/:post',
    name: RouteName.BLOG_POST,
    component: () => import('~/pages/BlogPost.vue'),
    meta: {
      dynamicHeadTitle: true,
    },
  },
  {
    path: '/:pathMatch(.*)*',
    name: RouteName.NOT_FOUND,
    redirect: () => ({ name: RouteName.HOME }),
  },
];
