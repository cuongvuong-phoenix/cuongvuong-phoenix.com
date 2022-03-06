import { type I18n as baseI18n } from 'vue-i18n';

export type I18n = baseI18n<{}, {}, {}, false>;

/* ----------------------------------------------------------------
Constants
---------------------------------------------------------------- */
export const LANGUAGES = [
  { locale: 'en-US', name: 'English (US)', default: true },
  // { locale: 'vi-VI', name: 'Tiếng Việt', default: false },
] as const;

export type AppLocale = typeof LANGUAGES[number]['locale'];

export const LOCALES = LANGUAGES.map((lang) => lang.locale);

export const DEFAULT_LOCALE = LANGUAGES.find((lang) => lang.default)!.locale;

/* ----------------------------------------------------------------
Functions
---------------------------------------------------------------- */
export function setI18nLocale(i18n: I18n, locale: string) {
  i18n.global.locale.value = locale;

  document.querySelector('html')?.setAttribute('lang', locale);
}
