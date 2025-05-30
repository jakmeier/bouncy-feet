import { addTranslations, locale, setLocale, setRoute } from '$lib/i18n.js';
import { initInstructorWasmOnce } from '$lib/instructor/loader.js';
import {
    Course,
    dances,
    init,
    loadDanceString,
    loadPoseString,
    loadStepString,
    parseCourseString,
    poses,
    PoseWrapper,
    steps,
    stepsBySource,
    StepWrapper,
} from '$lib/instructor/bouncy_instructor';

// This is the root layout, hence it defines prerendering for the entire app default.
// Translations act weird with prerender on, so I'm disabling it.
export const prerender = false;
export const ssr = true;
export const trailingSlash = 'always';

let loadedOnce = false;

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, data }) => {
    await initInstructorWasmOnce(fetch);
    const { i18n, translations } = data;
    const { locale, route } = i18n;

    addTranslations(translations);

    await setRoute(route);
    await setLocale(locale);

    const { lookupSteps, lookupPoses } = await loadCollectionAssets();
    const officialDances = dances();

    const coursePromises = [
        import('$lib/assets/courses/000-rm-basics.ron?raw'),
        import('$lib/assets/courses/002-v-step-basics.ron?raw'),
        import('$lib/assets/courses/003-intro.ron?raw'),
        import('$lib/assets/courses/004-rm-practice.ron?raw'),
        import('$lib/assets/courses/005-rrm.ron?raw')
    ].map(promise => promise
        .then(
            (data) => data.default
        ).then(
            (text) => parseCourseString(text, locale)
        )
        .catch((e) => console.error(e)));

    /** @type {(Course|void)[]} */
    const coursesResults = await Promise.all(coursePromises);
    /** @type {Course[]} */
    // @ts-ignore
    const courses = coursesResults.filter((c) => c);

    return {
        i18n,
        translations,
        officialDances,
        courses,
        lookupSteps,
        lookupPoses,
    };
}


async function loadCollectionAssets() {
    const poseFile = loadRon("pose");
    const animationPoseFile = loadRon("animation_poses");
    const danceFile = loadRon("dance");

    const stepFiles = [
        import('$lib/assets/steps/basic.ron?raw'),
        import('$lib/assets/steps/footwork.ron?raw'),
        import('$lib/assets/steps/idle_steps.ron?raw'),
        import('$lib/assets/steps/misc.ron?raw'),
        import('$lib/assets/steps/rm_variations.ron?raw'),
        import('$lib/assets/steps/shapes.ron?raw'),
        import('$lib/assets/steps/animation.ron?raw')
    ].map((promise) => promise.catch((e) => console.error(e)));

    const [
        poseFileResponse,
        animationPoseFileResponse,
        danceFileResponse,
        ...stepFileResponses
    ] = await Promise.all([poseFile, animationPoseFile, danceFile, ...stepFiles]);

    const stepFileStrings = await Promise.all(stepFileResponses.map((data) => data.default)).catch((e) => console.error(e));

    const poseFileString = poseFileResponse;
    const animationPoseFileString = animationPoseFileResponse;
    const danceFileString = danceFileResponse;

    let collectionData = {
        poseFileString,
        animationPoseFileString,
        danceFileString,
        stepFileStrings: {
            basic: stepFileStrings[0],
            footwork: stepFileStrings[1],
            idle_steps: stepFileStrings[2],
            misc: stepFileStrings[3],
            rm_variations: stepFileStrings[4],
            shapes: stepFileStrings[5],
            animation: stepFileStrings[6]
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
            animation: stepsBySource('animation'),
        }
    }


    /** 
     * Exported function to lookup a filtered list of steps.
     *
     * (Note: I might want to move this to Rust code.)
     * @param {StepFilter} filter 
     * @returns {StepWrapper[]}
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

    /** 
     * Exported function to lookup a filtered list of poses.
     *
     * (Note: I might want to move this to Rust code.)
     * @param {StepFilter} filter 
     * @returns {PoseWrapper[]}
    */
    function lookupPoses(filter) {
        // TODO: filtering
        return poses();
    }


    return { lookupSteps, lookupPoses };
}

/**
 * @param {string} name
 * @returns {Promise<string>}
 */
function loadRon(name) {
    return import(`$lib/assets/${name}.ron?raw`).then((data) => data.default).catch((e) => console.error(e));
}

/**
 * @param {{ poseFileString: any; animationPoseFileString: string; danceFileString: any; stepFileStrings: any; }} data
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
        if (data.animationPoseFileString) {
            loadPoseString(data.animationPoseFileString);
            loadStepString(data.stepFileStrings.animation, 'animation');
        }
        loadDanceString(data.danceFileString);
    }
}
