import { createI18n } from 'vue-i18n';
import { DEFAULT_LOCALE, type I18n } from './utils';

export * from '~/locales/utils';

export function setupI18n(messages: any) {
  const i18n = createI18n({
    legacy: false,
    fallbackLocale: DEFAULT_LOCALE,
    messages,
    missingWarn: false,
    fallbackWarn: false,
  }) as I18n;

  return i18n;
}
