import { locales, loadTranslations, defaultLocale, translations } from '$lib/i18n.js';

export const prerender = true;

/** @type {import('@sveltejs/kit').ServerLoad} */
export const load = async ({ fetch, url, cookies, request }) => {
    const { pathname } = url;

    // Try to get the locale from cookie
    let locale = (cookies.get('lang') || '');

    // Get user preferred locale
    if (!locale) {
        // TODO: better locale detection: find closest match instead of exact match only
        const header = request.headers.get('accept-language');
        if (header) {
            locale = header.split(/,/)[0];
        }
    }

    // Get defined locales
    const supportedLocales = locales.get();

    // Use default locale if current locale is not supported
    if (!supportedLocales.includes(locale)) {
        locale = defaultLocale;
    }

    await loadTranslations(locale, pathname); // keep this just before the `return`

    const poseFile = fetch('/pose.ron').catch((e) => console.error(e));
    const danceFile = fetch('/dance.ron').catch((e) => console.error(e));

    const stepFiles = [
        'basic.ron',
        'footwork.ron',
        'idle_steps.ron',
        'misc.ron',
        'rm_variations.ron',
        'shapes.ron'
    ].map(filename => fetch(`/steps/${filename}`).catch((e) => console.error(e)));

    const [
        poseFileResponse,
        danceFileResponse,
        ...stepFileResponses
    ] = await Promise.all([poseFile, danceFile, ...stepFiles]);

    const stepFileStrings = await Promise.all(stepFileResponses.map(response => response.text()));

    return {
        i18n: { locale, route: pathname },
        translations: translations.get(), // `translations` on server contain all translations loaded by different clients
        poseFileString: await poseFileResponse.text(),
        danceFileString: await danceFileResponse.text(),
        stepFileStrings: {
            basic: stepFileStrings[0],
            footwork: stepFileStrings[1],
            idle_steps: stepFileStrings[2],
            misc: stepFileStrings[3],
            rm_variations: stepFileStrings[4],
            shapes: stepFileStrings[5]
        }
    };
};