import { addTranslations, setLocale, setRoute } from '$lib/i18n.js';
import {
    loadDanceString,
    loadPoseString,
    loadStepString,
    dances,
    steps,
    stepsBySource,
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
    const stepBySourceData = {
        basic: stepsBySource('basic'),
        footwork: stepsBySource('footwork'),
        idle_steps: stepsBySource('idle_steps'),
        misc: stepsBySource('misc'),
        rm_variations: stepsBySource('rm_variations'),
        shapes: stepsBySource('shapes'),
    };
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
        steps: stepBySourceData,
        officialDances,
    };
};

function loadOnce(data) {
    loadPoseString(data.poseFileString);
    loadStepString(data.stepFileStrings.basic, 'basic');
    loadStepString(data.stepFileStrings.footwork, 'footwork');
    loadStepString(data.stepFileStrings.idle_steps, 'idle_steps');
    loadStepString(data.stepFileStrings.misc, 'misc');
    loadStepString(data.stepFileStrings.rm_variations, 'rm_variations');
    loadStepString(data.stepFileStrings.shapes, 'shapes');
    loadDanceString(data.danceFileString);
}