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

    FINISHED_INTRO_PART1: "FINISHED_INTRO_PART1",

    // TODO: show activated moves, teacher profiles, ...
    // : "",

}