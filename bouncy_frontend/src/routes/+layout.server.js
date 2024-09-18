import { locales, loadTranslations, defaultLocale, translations } from '$lib/i18n.js';

// This is the root layout, hence it defines prerendering for the entire app default.
export const prerender = false;

/** @type {import('@sveltejs/kit').ServerLoad} */
export const load = async ({ url, cookies, request }) => {
    const { pathname } = url;

    // Try to get the locale from cookie
    let locale = (cookies.get('lang') || '');
    // Get defined locales
    const supportedLocales = locales.get();

    // Get user preferred locale
    if (!locale) {
        const header = request.headers.get('accept-language');
        if (header) {
            locale = findBestLocale(header, supportedLocales);
        }
    }

    // Use default locale if current locale is not supported
    if (!supportedLocales.includes(locale)) {
        locale = defaultLocale;
        console.log("Found a matching locale: ", locale);
    }

    await loadTranslations(locale, pathname); // keep this just before the `return`

    return {
        i18n: { locale, route: pathname },
        translations: translations.get(), // `translations` on server contain all translations loaded by different clients
    };
};

/**
 * @param {string} header
 * @param {string[]} supportedLocales
 */
function findBestLocale(header, supportedLocales) {

    // The accept-language header may have multiple locales separated by ",",
    // each item potentially given a quality weight after a ";".
    // Example:
    // Accept-Language: fr-CH, fr;q=0.9, en;q=0.8, de;q=0.7, *;q=0.5
    const preferredLocales = header.split(',').map(locale => {
        const parts = locale.trim().split(';');
        const lang = parts[0];
        const quality = parts[1] ? parseFloat(parts[1].split('=')[1]) : 1.0; // Defaults to 1 if no quality value
        return { lang, quality };
    }).sort((a, b) => b.quality - a.quality); // Sort by quality

    // language + region match > language match > lower priority locale
    for (const { lang } of preferredLocales) {
        if (supportedLocales.includes(lang)) {
            return lang;
        }

        const baseLang = lang.split('-')[0];
        const closestMatch = supportedLocales.find(supported => supported.startsWith(baseLang));
        if (closestMatch) {
            return closestMatch;
        }
    }

    return null;
}