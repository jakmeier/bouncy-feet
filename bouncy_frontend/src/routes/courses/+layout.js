import { Course, parseCourseString } from '$lib/instructor/bouncy_instructor';
import { base } from '$app/paths';


/** @type {import('./$types').LayoutLoad} */
export async function load({ fetch, parent }) {
    const { i18n } = await parent();
    const lang = i18n.locale;
    const coursePromises = [
        '000-rm-basics.ron'
    ].map(filename => fetch(`${base}/courses/${filename}`)
        .then(
            (data) => data.text()
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

