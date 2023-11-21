import { locales, loadTranslations, defaultLocale, translations } from '$lib/i18n.js';

/** @type {import('@sveltejs/kit').ServerLoad} */
export const load = async ({ url, cookies, request }) => {
    const { pathname } = url;

    // Try to get the locale from cookie
    let locale = (cookies.get('lang') || '');

    // Get user preferred locale
    if (!locale) {
        // TODO: better locale detection: find closest match instead of exact match only
        locale = request.headers.get('accept-language').split(/,/)[0];
    }

    // Get defined locales
    const supportedLocales = locales.get();

    // Use default locale if current locale is not supported
    if (!supportedLocales.includes(locale)) {
        locale = defaultLocale;
    }

    await loadTranslations(locale, pathname); // keep this just before the `return`

    return {
        i18n: { locale, route: pathname },
        translations: translations.get(), // `translations` on server contain all translations loaded by different clients
    };
};