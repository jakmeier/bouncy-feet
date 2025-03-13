// safari makes me do this sorta thing
// https://github.com/sveltejs/kit/issues/7805
// https://github.com/sveltejs/kit/issues/13015

let initialized = false;

export async function initInstructorWasmOnce(fetch) {
    if (initialized) return;
    const mod = await import('$lib/instructor/bouncy_instructor.js');

    if (import.meta.env.SSR) {
        const res = await fetch('/wasm/bouncy_instructor_bg.wasm');
        if (!res.ok) throw new Error(`Failed to fetch WASM in SSR: ${res.status}`);
        const bytes = await res.arrayBuffer();

        const initSync = mod.default;
        initSync(bytes);
    } else {
        const init = mod.default;
        // this uses fetch, which only works in the client
        await init();
    }

    initialized = true;
}