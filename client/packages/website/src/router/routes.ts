import { type RouteRecordRaw } from 'vue-router';
import AnimatedRouterView from '~/components/AnimatedRouterView.vue';
import Home from '~/pages/Home.vue';
import { RouteName } from '~/utils/constants';

export const routes: RouteRecordRaw[] = [
  {
    path: '/:locale?/',
    component: AnimatedRouterView,
    children: [
      {
        path: '',
        name: RouteName.HOME,
        component: Home,
        meta: {
          staticHeader: true,
        },
      },
      {
        path: 'about',
        name: RouteName.ABOUT,
        component: () => import('~/pages/About.vue'),
      },
      {
        path: 'contact',
        name: RouteName.CONTACT,
        component: () => import('~/pages/Contact.vue'),
      },
      {
        path: 'blog',
        name: RouteName.BLOG,
        component: () => import('~/pages/Blog.vue'),
      },
      {
        path: 'notes',
        name: RouteName.NOTES,
        component: () => import('~/pages/Notes.vue'),
      },
    ],
  },
];
