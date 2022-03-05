import 'vue-router';

declare module 'vue-router' {
  interface RouteMeta {
    customHead?: boolean;
    dynamicHeadTitle?: boolean;
    transition?: string;
    transitionLeaveDuration?: number;
  }
}
