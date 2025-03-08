import i18n from 'sveltekit-i18n';
import { enUS, enGB, de } from 'date-fns/locale';

export const defaultLocale = 'en-GB';

// Note: Currently there is one global translation file per language. A more
// scalable approach would be to use per-module files. The data inside the
// mono-file are already hierarchically sorted by page, so a transition wouldn't
// be too horrible to do with a script. But for now it's easier and good-enough
// to stay with a single file.

/** @type {import('sveltekit-i18n').Config} */
const config = ({
    // statically available translations
    translations: {},
    // loaded translations
    loaders: [
        {
            locale: 'en-GB',
            key: '',
            loader: async () => (
                await import('./i18n/en-GB.json')
            ).default,
        },
        {
            locale: 'de-CH',
            key: '',
            loader: async () => (
                await import('./i18n/de-CH.json')
            ).default,
        },
    ],
});

export const { t, locale, locales, loading, loadTranslations, translations, addTranslations, setLocale, setRoute } = new i18n(config);

/** @param {string} locale */
export function dateLocale(locale) {
    return { locale: selectDateLocale(locale) };
}

/** @param {string} locale */
function selectDateLocale(locale) {
    switch (locale) {
        case 'en-GB':
            return enGB;
        case 'en-US':
            return enUS;
        case 'de-CH':
            return de;
        default:
            return enGB;
    }
}