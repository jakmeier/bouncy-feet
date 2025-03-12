// safari makes me do this sorta thing
// https://github.com/sveltejs/kit/issues/7805
// https://github.com/sveltejs/kit/issues/13015

let initialized = false;

export async function initInstructorWasmOnce() {
    if (initialized) return;
    const mod = await import('$lib/instructor/bouncy_instructor');
    let init = mod.default;
    await init();
    initialized = true;
}