import { PUBLIC_API_BASE } from "$env/static/public";

/**
 * @readonly
 * @enum {OnboardingState} 
 */
export const ONBOARDING_STATE = {
    // A user just opened the app.
    FIRST_VISIT: "FIRST_VISIT",

    // Where on the journey through the first (static) selection of lessons the user.
    STARTED_FIRST_WARMUP: "STARTED_FIRST_WARMUP",
    FINISHED_FIRST_WARMUP: "FINISHED_FIRST_WARMUP",
    STARTED_FIRST_LESSON: "STARTED_FIRST_LESSON",
    FINISHED_FIRST_LESSON: "FINISHED_FIRST_LESSON",
    STARTED_SECOND_LESSON: "STARTED_SECOND_LESSON",
    FINISHED_SECOND_LESSON: "FINISHED_SECOND_LESSON",
    STARTED_THIRD_LESSON: "STARTED_THIRD_LESSON",

    FINISHED_INTRO_PART1: "FINISHED_INTRO_PART1",

    // TODO: show activated moves, teacher profiles, ...
    // : "",

}

export function register() {
    // redirect to backend register -> login
    const currentUrl = window.location.href;
    window.location.assign(
        PUBLIC_API_BASE +
        '/register?redirect_back_to=' +
        encodeURIComponent(currentUrl)
    );
}