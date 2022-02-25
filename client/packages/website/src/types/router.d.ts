import 'vue-router';

declare module 'vue-router' {
  interface RouteMeta {
    transition?: string;
    transitionLeaveDuration?: number;
    staticHeader?: boolean;
  }
}
