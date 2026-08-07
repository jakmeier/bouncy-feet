import { addTranslations, locale, setLocale, setRoute } from '$lib/i18n.js';

// These modules pull in the instructor WASM and the full application data. Keep
// them dynamic so the lightweight QR landing route never downloads or starts
// the app shell.
/** @type {typeof import('$lib/instructor/loader.js').initInstructorWasmOnce} */
let initInstructorWasmOnce;
/** @type {typeof import('$lib/instructor/bouncy_instructor').dances} */
let dances;
/** @type {typeof import('$lib/instructor/bouncy_instructor').bf_init} */
let bf_init;
/** @type {typeof import('$lib/instructor/bouncy_instructor').loadDanceString} */
let loadDanceString;
/** @type {typeof import('$lib/instructor/bouncy_instructor').loadPoseString} */
let loadPoseString;
/** @type {typeof import('$lib/instructor/bouncy_instructor').loadStepString} */
let loadStepString;
/** @type {typeof import('$lib/instructor/bouncy_instructor').parseCourseString} */
let parseCourseString;
/** @type {typeof import('$lib/instructor/bouncy_instructor').poses} */
let poses;
/** @type {typeof import('$lib/instructor/bouncy_instructor').steps} */
let steps;
/** @type {typeof import('$lib/instructor/bouncy_instructor').stepsBySource} */
let stepsBySource;
/** @type {typeof import('$lib/stores/Clubs.svelte').loadPublicClubs} */
let loadPublicClubs;

async function loadAppDependencies() {
    if (!initInstructorWasmOnce) {
        const [loader, instructor, clubs] = await Promise.all([
            import('$lib/instructor/loader.js'),
            import('$lib/instructor/bouncy_instructor'),
            import('$lib/stores/Clubs.svelte'),
        ]);
        ({ initInstructorWasmOnce } = loader);
        ({
            dances,
            bf_init,
            loadDanceString,
            loadPoseString,
            loadStepString,
            parseCourseString,
            poses,
            steps,
            stepsBySource,
        } = instructor);
        ({ loadPublicClubs } = clubs);
    }
}

// This is the root layout, hence it defines prerendering for the entire app default.
// Translations act weird with prerender on, so I'm disabling it.
export const prerender = false;
export const ssr = true;
export const trailingSlash = 'always';

let loadedOnce = false;

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, data, url }) => {
    const { i18n, translations } = /** @type {any} */ (data);
    const { locale, route } = i18n;
    const isQrShell = url.pathname === '/qr' || url.pathname.startsWith('/qr/');

    addTranslations(translations);
    await setRoute(route);
    await setLocale(locale);

    if (isQrShell) {
        // Keep the layout data shape stable for non-QR child routes without
        // loading their data or returning non-serializable placeholder
        // functions. The QR shell never consumes these values.
        return {
            i18n,
            translations,
            officialDances: /** @type {any} */ ([]),
            courses: /** @type {any} */ ([]),
            publicClubs: /** @type {any} */ ([]),
            lookupSteps: /** @type {any} */ (null),
            lookupPoses: /** @type {any} */ (null),
            isQrShell,
        };
    }

    await loadAppDependencies();
    await initInstructorWasmOnce(fetch);

    const { lookupSteps, lookupPoses } = await loadCollectionAssets();
    const officialDances = dances();

    const coursePromises = [
        import('$lib/assets/courses/000-rm-basics.ron?raw'),
        import('$lib/assets/courses/002-v-step-basics.ron?raw'),
        import('$lib/assets/courses/003-intro.ron?raw'),
        import('$lib/assets/courses/004-rm-practice.ron?raw'),
        import('$lib/assets/courses/005-rrm.ron?raw'),
        import('$lib/assets/courses/006-dnb.ron?raw')
    ].map(promise => promise
        .then(
            (data) => data.default
        ).then(
            (text) => parseCourseString(text, locale)
        )
        .catch((e) => console.error(e)));

    /** @type {(import('$lib/instructor/bouncy_instructor').Course|void)[]} */
    const coursesResults = await Promise.all(coursePromises);
    /** @type {import('$lib/instructor/bouncy_instructor').Course[]} */
    // @ts-ignore
    const courses = coursesResults.filter((c) => c);
    const publicClubs = await loadPublicClubs(fetch);

    return {
        i18n,
        translations,
        officialDances,
        courses,
        publicClubs,
        lookupSteps,
        lookupPoses,
        isQrShell,
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
        import('$lib/assets/steps/animation.ron?raw'),
        import('$lib/assets/steps/dnb.ron?raw')
    ].map((promise) => promise.catch((e) => console.error(e)));

    const [
        poseFileResponse,
        animationPoseFileResponse,
        danceFileResponse,
        ...stepFileResponses
    ] = await Promise.all([poseFile, animationPoseFile, danceFile, ...stepFiles]);

    const stepFileStrings = (await Promise.all(stepFileResponses.map((data) => /** @type {any} */ (data).default)).catch((e) => console.error(e))) || [];

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
            animation: stepFileStrings[6],
            dnb: stepFileStrings[7],
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
            dnb: stepsBySource('dnb'),
        }
    }


    /** 
     * Exported function to lookup a filtered list of steps.
     *
     * (Note: I might want to move this to Rust code.)
     * @param {StepFilter} filter 
     * @returns {import('$lib/instructor/bouncy_instructor').StepWrapper[]}
    */
    function lookupSteps(filter) {
        // collect all steps to include from sources
        let stepLists = [];
        if (filter.sources) {
            for (let i = 0; i < filter.sources.length; i++) {
                const sourceName = filter.sources[i];
                const list = stepData.bySource[/** @type {keyof typeof stepData.bySource} */ (sourceName)];
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
     * @returns {import('$lib/instructor/bouncy_instructor').PoseWrapper[]}
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
        bf_init(Math.random() * 4_294_967_295, lang);
        loadPoseString(data.poseFileString);
        loadStepString(data.stepFileStrings.basic, 'basic');
        loadStepString(data.stepFileStrings.footwork, 'footwork');
        loadStepString(data.stepFileStrings.idle_steps, 'idle_steps');
        loadStepString(data.stepFileStrings.misc, 'misc');
        loadStepString(data.stepFileStrings.rm_variations, 'rm_variations');
        loadStepString(data.stepFileStrings.shapes, 'shapes');
        loadStepString(data.stepFileStrings.dnb, 'dnb');
        if (data.animationPoseFileString) {
            loadPoseString(data.animationPoseFileString);
            loadStepString(data.stepFileStrings.animation, 'animation');
        }
        loadDanceString(data.danceFileString);
    }
}
