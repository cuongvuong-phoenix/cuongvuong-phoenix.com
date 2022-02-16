import { type I18n as baseI18n } from 'vue-i18n';

export type I18n = baseI18n<{}, {}, {}, false>;

/* ----------------------------------------------------------------
Constants
---------------------------------------------------------------- */
export const LANGUAGES: { locale: string; name: string; default?: boolean }[] = [
  { locale: 'en', name: 'English', default: true },
  { locale: 'vi', name: 'Tiếng Việt' },
];

export const LOCALES = LANGUAGES.map((lang) => lang.locale);

export const DEFAULT_LOCALE = LANGUAGES.find((lang) => lang.default)!.locale;

/* ----------------------------------------------------------------
Functions
---------------------------------------------------------------- */
export function setI18nLocale(i18n: I18n, locale: string) {
  i18n.global.locale.value = locale;

  document.querySelector('html')?.setAttribute('lang', locale);
}
