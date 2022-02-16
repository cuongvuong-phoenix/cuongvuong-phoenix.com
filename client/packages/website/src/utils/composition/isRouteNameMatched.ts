import { type RouteLocationNormalizedLoaded } from 'vue-router';
import { type RouteName } from '../constants';

export function isRouteNameMatched(route: RouteLocationNormalizedLoaded, name: RouteName, exact = true) {
  if (exact) {
    return route.name === name;
  } else {
    return route.matched.reduce((accum, item) => accum || item.name === name, false);
  }
}
