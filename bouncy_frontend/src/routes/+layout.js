import { addTranslations, setLocale, setRoute } from '$lib/i18n.js';
import {
    loadPoseString,
    loadStepString,
    steps,
} from '$lib/instructor/bouncy_instructor';

let loadedOnce = false;

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ data }) => {
    const { i18n, translations } = data;
    const { locale, route } = i18n;

    addTranslations(translations);

    await setRoute(route);
    await setLocale(locale);

    if (!loadedOnce) {
        loadedOnce = true;
        loadOnce(data);
    }

    return {
        i18n,
        translations,
        allSteps: steps(),
    };
};

function loadOnce(data) {
    loadPoseString(data.poseFileString);
    loadStepString(data.stepFileString);
}