import { addTranslations, setLocale, setRoute } from '$lib/i18n.js';
import {
    loadDanceString,
    loadPoseString,
    loadStepString,
    dances,
    steps,
    stepsBySource,
    StepInfo,
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

    const stepData = {
        allSteps: steps(),
        bySource: {
            basic: stepsBySource('basic'),
            footwork: stepsBySource('footwork'),
            idle_steps: stepsBySource('idle_steps'),
            misc: stepsBySource('misc'),
            rm_variations: stepsBySource('rm_variations'),
            shapes: stepsBySource('shapes'),
        }
    }

    const officialDances = dances();

    /** 
     * Exported function to lookup a filtered list of steps.
     *
     * (Note: I might want to move this to Rust code.)
     * @param {StepFilter} filter 
     * @returns {StepInfo[]}
    */
    function lookupSteps(filter) {
        // collect all steps to include from sources
        let stepLists = [];
        if (filter.sources) {
            for (let i = 0; i < filter.sources.length; i++) {
                const sourceName = filter.sources[i];
                const list = stepData.bySource[sourceName];
                if (list) {
                    stepLists.push(list);
                } else {
                    console.warn("could not load step source", sourceName)
                }
            }
        } else {
            stepLists.push(stepData.allSteps);
        }

        // if there are no additional filters, flatten the lists and return
        const needsNoFiltering = !filter.uniqueNames && !filter.stepName;
        if (needsNoFiltering) {
            return stepLists.flat();
        }

        // filter by given criteria
        const out = [];
        const seenNames = new Set();
        for (const stepList of stepLists) {
            for (const step of stepList) {
                if (filter.uniqueNames && seenNames.has(step.name)) {
                    continue;
                }
                if (filter.stepName && step.name !== filter.stepName) {
                    continue;
                }
                seenNames.add(step.name);
                out.push(step);
            }
        }

        return out;
    };

    return {
        i18n,
        translations,
        officialDances,
        lookupSteps,
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