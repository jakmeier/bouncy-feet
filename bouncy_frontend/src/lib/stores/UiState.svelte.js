import { goto } from '$app/navigation';
import { tick } from 'svelte';
import { writable } from 'svelte/store';

export const hideNavigation = writable(true);
export const wideView = writable(false);
export const backgroundColor = writable('var(--theme-neutral-black)');
export const fontColor = writable('var(--theme-neutral-white)');

/**
 * State for persistent, root-level shared-element route transitions.
 * @typedef {{ top: number, left: number, width: number, height: number }} TransitionRect
 * @typedef {TransitionRect & { borderRadius: number }} TransitionSurface
 * @typedef {TransitionRect & { fontSize: number, color: string, fontWeight: string, text: string }} TransitionTitle
 * @typedef {TransitionRect & { fontSize: number, color: string, fontWeight: string }} TransitionTitleTarget
 * @typedef {TransitionRect & { html: string }} TransitionProgress
 */
export const pageTransition = $state({
    active: false,
    kind: /** @type {'challenge' | 'coach' | null} */ (null),
    finishing: false,
    setId: /** @type {string | undefined} */ (undefined),
    phase:
    /** @type {'fade-source' | 'enlarge-title' | 'expand' | 'settle' | 'finishing' | 'idle'} */ (
            'idle'
        ),
    surface: /** @type {TransitionSurface | null} */ (null),
    title: /** @type {TransitionTitle | null} */ (null),
    progress: /** @type {TransitionProgress | null} */ (null),
    destTitle: /** @type {TransitionTitleTarget | null} */ (null),
    destProgress: /** @type {TransitionRect | null} */ (null),
    destReady: false,
});

// Existing challenge pages retain this name; coach and challenge transitions
// now share the same state machine and root overlay.
export const challengeTransition = pageTransition;

const CHALLENGE_EXPAND_WAIT = 460;
const CHALLENGE_SETTLE_WAIT = 490;
const CHALLENGE_FINISH_WAIT = 400;

/** @param {Element} node @returns {TransitionRect} */
function rectOf(node) {
    const r = node.getBoundingClientRect();
    return { top: r.top, left: r.left, width: r.width, height: r.height };
}

/** @param {number} ms */
function wait(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

/** @param {string} setId @param {number} timeout @returns {Promise<void>} */
function waitForDest(setId, timeout) {
    const start = Date.now();
    return new Promise((resolve) => {
        function check() {
            if (
                pageTransition.setId !== setId ||
                pageTransition.destReady ||
                Date.now() - start > timeout
            ) {
                resolve();
                return;
            }
            setTimeout(check, 30);
        }
        check();
    });
}

/**
 * @param {string} href
 * @param {string} setId
 * @param {{ surfaceNode: HTMLElement, titleNode: HTMLElement, progressNode: HTMLElement }} nodes
 */
export async function startChallengeTransition(href, setId, nodes) {
    if (pageTransition.active) return;
    if (
        typeof window !== 'undefined' &&
        window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ) {
        goto(href);
        return;
    }

    const surfaceStyle = getComputedStyle(nodes.surfaceNode);
    const titleStyle = getComputedStyle(nodes.titleNode);
    pageTransition.kind = 'challenge';
    pageTransition.setId = setId;
    pageTransition.surface = {
        ...rectOf(nodes.surfaceNode),
        borderRadius: parseFloat(surfaceStyle.borderRadius) || 24,
    };
    pageTransition.title = {
        ...rectOf(nodes.titleNode),
        fontSize: parseFloat(titleStyle.fontSize),
        color: titleStyle.color,
        fontWeight: titleStyle.fontWeight,
        text: (nodes.titleNode.textContent || '').trim(),
    };
    pageTransition.progress = {
        ...rectOf(nodes.progressNode),
        html: nodes.progressNode.innerHTML,
    };
    pageTransition.destReady = false;
    pageTransition.destTitle = null;
    pageTransition.destProgress = null;
    pageTransition.finishing = false;
    pageTransition.phase = 'expand';
    pageTransition.active = true;

    await tick();
    await wait(CHALLENGE_EXPAND_WAIT);
    goto(href);
    await waitForDest(setId, 1200);
    pageTransition.phase = 'settle';
    await tick();
    await wait(CHALLENGE_SETTLE_WAIT);
    pageTransition.phase = 'finishing';
    pageTransition.finishing = true;
    await tick();
    await wait(CHALLENGE_FINISH_WAIT);
    resetTransition();
}

/** @param {string} setId @param {Element | null | undefined} titleNode @param {Element | null | undefined} progressNode */
export function publishChallengeDest(setId, titleNode, progressNode) {
    if (
        pageTransition.kind !== 'challenge' ||
        !pageTransition.active ||
        pageTransition.setId !== setId
    )
        return;
    if (titleNode) {
        const style = getComputedStyle(titleNode);
        pageTransition.destTitle = {
            ...rectOf(titleNode),
            fontSize: parseFloat(style.fontSize),
            color: style.color,
            fontWeight: style.fontWeight,
        };
    }
    if (progressNode) pageTransition.destProgress = rectOf(progressNode);
    pageTransition.destReady = true;
}

/**
 * @param {string} href
 * @param {HTMLElement} titleNode
 * @param {string} text
 */
export async function startCoachTransition(href, titleNode, text) {
    if (pageTransition.active) return;
    if (
        typeof window !== 'undefined' &&
        window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ) {
        goto(href);
        return;
    }

    const style = getComputedStyle(titleNode);
    pageTransition.kind = 'coach';
    pageTransition.setId = undefined;
    pageTransition.surface = null;
    pageTransition.progress = null;
    pageTransition.destTitle = null;
    pageTransition.destProgress = null;
    pageTransition.destReady = false;
    pageTransition.title = {
        ...rectOf(titleNode),
        fontSize: parseFloat(style.fontSize),
        color: style.color,
        fontWeight: style.fontWeight,
        text,
    };
    pageTransition.finishing = false;
    pageTransition.phase = 'fade-source';
    pageTransition.active = true;

    await tick();
    await wait(600);
    // The old floating element was an h1, which became bold before enlarging.
    pageTransition.title.fontWeight = '700';
    pageTransition.phase = 'enlarge-title';
    await tick();
    await wait(600);

    goto(href);
    pageTransition.phase = 'settle';
    await tick();
    await waitForCoachDest(1200);
    // Settle the clone onto the real header, then crossfade to that header while
    // the destination page is already fading in underneath.
    await wait(600);
    pageTransition.phase = 'finishing';
    pageTransition.finishing = true;
    await tick();
    await wait(600);
    resetTransition();
}

/** @param {number} timeout @returns {Promise<void>} */
function waitForCoachDest(timeout) {
    const start = Date.now();
    return new Promise((resolve) => {
        function check() {
            if (
                pageTransition.kind !== 'coach' ||
                pageTransition.destReady ||
                Date.now() - start > timeout
            ) {
                resolve();
                return;
            }
            setTimeout(check, 30);
        }
        check();
    });
}

/** @param {Element | null | undefined} titleNode */
export function publishCoachDest(titleNode) {
    if (
        pageTransition.kind !== 'coach' ||
        pageTransition.phase !== 'settle' ||
        !titleNode
    )
        return;
    const style = getComputedStyle(titleNode);
    pageTransition.destTitle = {
        ...rectOf(titleNode),
        fontSize: parseFloat(style.fontSize),
        color: style.color,
        fontWeight: style.fontWeight,
    };
    pageTransition.destReady = true;
}

function resetTransition() {
    pageTransition.active = false;
    pageTransition.kind = null;
    pageTransition.finishing = false;
    pageTransition.phase = 'idle';
    pageTransition.destReady = false;
}
