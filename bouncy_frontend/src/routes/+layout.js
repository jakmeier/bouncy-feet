import { addTranslations, setLocale, setRoute } from '$lib/i18n.js';
import {
    loadDanceString,
    loadPoseString,
    loadStepString,
    dances,
    steps,
} from '$lib/instructor/bouncy_instructor';

export const prerender = true;
export const trailingSlash = 'always';

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

    const allSteps = steps();
    const officialDances = dances();
    const uniqueNameSteps = [];

    const seenNames = new Set();
    for (const step of allSteps) {
        if (!seenNames.has(step.name)) {
            seenNames.add(step.name);
            uniqueNameSteps.push(step);
        }
    }

    return {
        i18n,
        translations,
        uniqueNameSteps,
        allSteps,
        officialDances,
    };
};

function loadOnce(data) {
    loadPoseString(data.poseFileString);
    loadStepString(data.stepFileString);
    loadDanceString(data.danceFileString);
}