import { createI18n } from 'vue-i18n';
import { type Router } from 'vue-router';
import { type AppLocale, DEFAULT_LOCALE, type I18n, LOCALES, setI18nLocale } from './utils';

export * from '~/locales/utils';

function setupRouterForI18n(i18n: I18n, router: Router) {
  // Auto loading messages & setting locale based on `locale` param.
  router.beforeEach(async (to) => {
    const { name, params, query, hash, fullPath } = to;

    if (!name) {
      throw new Error(`Must provide "name" for "${fullPath}" route.`);
    }

    const locale = params.locale as AppLocale;

    // Check if got the right locales.
    if (!LOCALES.includes(locale)) {
      return { name, params: { ...params, locale: DEFAULT_LOCALE }, query, hash };
    }

    setI18nLocale(i18n, locale);
  });
}

export function setupI18n(router: Router, messages: any) {
  const i18n = createI18n({
    legacy: false,
    fallbackLocale: DEFAULT_LOCALE,
    messages,
    missingWarn: false,
    fallbackWarn: false,
  }) as I18n;

  setupRouterForI18n(i18n, router);

  return i18n;
}
