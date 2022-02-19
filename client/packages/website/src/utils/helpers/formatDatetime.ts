import { type Locale, format } from 'date-fns';
import { enUS } from 'date-fns/locale';
import { type AppLocale } from '~/locales/utils';

const localeObjectMap: Record<AppLocale, Locale> = {
  'en-US': enUS,
};

export function formatDatetime(date: Date, locale: string, formatString = 'PP') {
  return format(date, formatString, {
    locale: localeObjectMap[locale as AppLocale],
  });
}
