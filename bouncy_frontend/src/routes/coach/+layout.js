import { Course, parseCourseString } from '$lib/instructor/bouncy_instructor';
import { base } from '$app/paths';


/** @type {import('./$types').LayoutLoad} */
export async function load({ fetch, parent }) {
    const { i18n } = await parent();
    const lang = i18n.locale;
    const coursePromises = [
        import('$lib/assets/courses/000-rm-basics.ron?raw'),
        import('$lib/assets/courses/002-v-step-basics.ron?raw'),
        import('$lib/assets/courses/004-rm-practice.ron?raw'),
        import('$lib/assets/courses/005-rrm.ron?raw')
    ].map(promise => promise
        .then(
            (data) => data.default
        ).then(
            (text) => parseCourseString(text, lang)
        )
        .catch((e) => console.error(e)));

    /** @type {(Course|void)[]} */
    const coursesResults = await Promise.all(coursePromises);
    /** @type {Course[]} */
    // @ts-ignore
    const courses = coursesResults.filter((c) => c);
    return {
        courses
    };
}

