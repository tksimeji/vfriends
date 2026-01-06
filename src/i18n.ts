import {createI18n} from 'vue-i18n';
import {messages} from './i18n/messages';

export type LocaleKey = keyof typeof messages;

const SUPPORTED_LOCALES = Object.keys(messages) as LocaleKey[];
const resolveLocale = (): LocaleKey => {
  if (typeof window === 'undefined') return 'ja';
  const stored = window.localStorage.getItem('locale');
  if (stored && SUPPORTED_LOCALES.includes(stored as LocaleKey)) {
    return stored as LocaleKey;
  }
  const browser = window.navigator.language?.split('-')[0];
  if (browser && SUPPORTED_LOCALES.includes(browser as LocaleKey)) {
    return browser as LocaleKey;
  }
  return 'ja';
};

export const i18n = createI18n({
  legacy: false,
  locale: resolveLocale(),
  fallbackLocale: 'en',
  messages,
});

export const setLocale = (locale: LocaleKey) => {
  i18n.global.locale.value = locale;
  if (typeof window !== 'undefined') {
    window.localStorage.setItem('locale', locale);
  }
};

export const t = (key: string, params?: Record<string, unknown>) =>
  params ? i18n.global.t(key, params) : i18n.global.t(key);
