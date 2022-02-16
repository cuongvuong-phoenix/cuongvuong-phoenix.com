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
      },
      {
        path: 'about',
        name: RouteName.ABOUT,
        component: Home,
      },
      {
        path: 'contact',
        name: RouteName.CONTACT,
        component: Home,
      },
      {
        path: 'blog',
        name: RouteName.BLOG,
        component: Home,
      },
      {
        path: 'notes',
        name: RouteName.NOTES,
        component: Home,
      },
    ],
  },
];
