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
        component: () => import('~/pages/blog/Index.vue'),
      },
      {
        path: 'blog/:post',
        name: RouteName.BLOG_POST,
        component: () => import('~/pages/BlogPost.vue'),
      },
      {
        path: 'notes',
        name: RouteName.NOTES,
        component: () => import('~/pages/Notes.vue'),
      },
      {
        path: ':pathMatch(.*)*',
        name: RouteName.NOT_FOUND,
        redirect: (to) => ({ name: RouteName.HOME, params: { locale: to.params.locale } }),
      },
    ],
  },
];
