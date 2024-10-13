import { addTranslations, loadTranslations, locale, setLocale, setRoute } from '$lib/i18n.js';
import {
    loadDanceString,
    loadPoseString,
    loadStepString,
    dances,
    steps,
    stepsBySource,
    StepInfo,
    init,
} from '$lib/instructor/bouncy_instructor';

// This is the root layout, hence it defines prerendering for the entire app default.
// Translations act weird with prerender on, so I'm disabling it.
export const prerender = false;
export const ssr = true;
export const trailingSlash = 'always';

let loadedOnce = false;

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ data }) => {
    const { i18n, translations } = data;
    const { locale, route } = i18n;

    addTranslations(translations);

    await setRoute(route);
    await setLocale(locale);

    const lookupSteps = await loadCollectionAssets();
    const officialDances = dances();

    return {
        i18n,
        translations,
        officialDances,
        lookupSteps,
    };
};


async function loadCollectionAssets() {
    const poseFile = loadRon("pose");
    const danceFile = loadRon("dance");

    const stepFiles = [
        import('$lib/assets/steps/basic.ron?raw'),
        import('$lib/assets/steps/footwork.ron?raw'),
        import('$lib/assets/steps/idle_steps.ron?raw'),
        import('$lib/assets/steps/misc.ron?raw'),
        import('$lib/assets/steps/rm_variations.ron?raw'),
        import('$lib/assets/steps/shapes.ron?raw')
    ].map((promise) => promise.catch((e) => console.error(e)));

    const [
        poseFileResponse,
        danceFileResponse,
        ...stepFileResponses
    ] = await Promise.all([poseFile, danceFile, ...stepFiles]);

    const stepFileStrings = await Promise.all(stepFileResponses.map((data) => data.default)).catch((e) => console.error(e));

    const poseFileString = poseFileResponse;
    const danceFileString = danceFileResponse;

    let collectionData = {
        poseFileString,
        danceFileString,
        stepFileStrings: {
            basic: stepFileStrings[0],
            footwork: stepFileStrings[1],
            idle_steps: stepFileStrings[2],
            misc: stepFileStrings[3],
            rm_variations: stepFileStrings[4],
            shapes: stepFileStrings[5]
        }
    };

    if (!loadedOnce) {
        loadOnce(collectionData, locale.get());
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

    return lookupSteps;
}

/**
 * @param {string} name
 * @returns {Promise<string>}
 */
function loadRon(name) {
    return import(`$lib/assets/${name}.ron?raw`).then((data) => data.default).catch((e) => console.error(e));
}

/**
 * @param {{ poseFileString: any; danceFileString: any; stepFileStrings: any; }} data
 * @param {string} lang
 */
function loadOnce(data, lang) {
    if (data.poseFileString && data.poseFileString.length > 0) {
        loadedOnce = true;
        init(Math.random() * 4_294_967_295, lang);
        loadPoseString(data.poseFileString);
        loadStepString(data.stepFileStrings.basic, 'basic');
        loadStepString(data.stepFileStrings.footwork, 'footwork');
        loadStepString(data.stepFileStrings.idle_steps, 'idle_steps');
        loadStepString(data.stepFileStrings.misc, 'misc');
        loadStepString(data.stepFileStrings.rm_variations, 'rm_variations');
        loadStepString(data.stepFileStrings.shapes, 'shapes');
        loadDanceString(data.danceFileString);
    }
}
