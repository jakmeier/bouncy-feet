
import { goto } from '$app/navigation';
import { tick } from 'svelte';
import { Tween } from 'svelte/motion';
import { writable } from 'svelte/store';

export const hideNavigation = writable(true);
export const wideView = writable(false);
export const backgroundColor = writable("var(--theme-neutral-black)");
export const fontColor = writable("var(--theme-neutral-white)");


/**
 * @typedef {{ state: boolean, text: string | undefined, color: string | undefined, fontSize: Tween<number> }} FadingOutState
 */
/** @type {FadingOutState} */
export const fadingOut = $state({
    state: false,
    text: undefined,
    color: undefined,
    fontSize: new Tween(16, { duration: 300, delay: 0 }),
});
export const floatingTitleRect = $state({
    top: 0,
    left: 0,
    width: 0,
    height: 0,
});

/**
 * Shared-element transition for opening a challenge set.
 *
 * @typedef {{ top: number, left: number, width: number, height: number }} ChallengeRect
 * @typedef {ChallengeRect & { borderRadius: number }} ChallengeSurface
 * @typedef {ChallengeRect & { fontSize: number, color: string, fontWeight: string, text: string }} ChallengeTitle
 * @typedef {ChallengeRect & { fontSize: number, color: string, fontWeight: string }} ChallengeTitleTarget
 * @typedef {ChallengeRect & { html: string }} ChallengeProgress
 * @typedef {{
 *   active: boolean,
 *   finishing: boolean,
 *   setId: string | undefined,
 *   phase: 'expand' | 'settle' | 'finishing' | 'idle',
 *   surface: ChallengeSurface | null,
 *   title: ChallengeTitle | null,
 *   progress: ChallengeProgress | null,
 *   destTitle: ChallengeTitleTarget | null,
 *   destProgress: ChallengeRect | null,
 *   destReady: boolean,
 * }} ChallengeTransitionState
 */
/** @type {ChallengeTransitionState} */
export const challengeTransition = $state({
    active: false,
    finishing: false,
    setId: undefined,
    phase: 'idle',
    surface: null,
    title: null,
    progress: null,
    destTitle: null,
    destProgress: null,
    destReady: false,
});

const CHALLENGE_EXPAND_WAIT = 460;
const CHALLENGE_SETTLE_WAIT = 490;
const CHALLENGE_FINISH_WAIT = 400;

/** @param {Element} node @returns {ChallengeRect} */
function rectOf(node) {
    const r = node.getBoundingClientRect();
    return { top: r.top, left: r.left, width: r.width, height: r.height };
}

/** @param {number} ms */
function wait(ms) {
    return new Promise((r) => setTimeout(r, ms));
}

/**
 * Wait until the destination page has published its target rects.
 *
 * @param {string} setId
 * @param {number} timeout
 * @returns {Promise<void>}
 */
function waitForDest(setId, timeout) {
    const start = Date.now();
    return new Promise((resolve) => {
        function check() {
            if (challengeTransition.setId !== setId) return resolve();
            if (challengeTransition.destReady) return resolve();
            if (Date.now() - start > timeout) return resolve();
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
    if (challengeTransition.active) return;
    if (
        typeof window !== 'undefined' &&
        window.matchMedia('(prefers-reduced-motion: reduce)').matches
    ) {
        goto(href);
        return;
    }

    const { surfaceNode, titleNode, progressNode } = nodes;
    const surfaceStyle = getComputedStyle(surfaceNode);
    const titleStyle = getComputedStyle(titleNode);

    challengeTransition.setId = setId;
    challengeTransition.surface = {
        ...rectOf(surfaceNode),
        borderRadius: parseFloat(surfaceStyle.borderRadius) || 24,
    };
    challengeTransition.title = {
        ...rectOf(titleNode),
        fontSize: parseFloat(titleStyle.fontSize),
        color: titleStyle.color,
        fontWeight: titleStyle.fontWeight,
        text: (titleNode.textContent || '').trim(),
    };
    challengeTransition.progress = {
        ...rectOf(progressNode),
        html: progressNode.innerHTML,
    };
    challengeTransition.destReady = false;
    challengeTransition.destTitle = null;
    challengeTransition.destProgress = null;
    challengeTransition.finishing = false;
    challengeTransition.phase = 'expand';
    challengeTransition.active = true;

    await tick();
    await wait(CHALLENGE_EXPAND_WAIT);

    goto(href);

    await waitForDest(setId, 1200);

    challengeTransition.phase = 'settle';
    await tick();
    await wait(CHALLENGE_SETTLE_WAIT);

    // Crossfade: fade the overlay clones out while the destination page fades
    // in, then tear the overlay down.
    challengeTransition.phase = 'finishing';
    challengeTransition.finishing = true;
    await tick();
    await wait(CHALLENGE_FINISH_WAIT);

    challengeTransition.active = false;
    challengeTransition.finishing = false;
    challengeTransition.phase = 'idle';
    challengeTransition.destReady = false;
}

/**
 * Called by the destination challenge page on mount so the overlay can glide
 * its cloned title / progress into the real target positions.
 *
 * @param {string} setId
 * @param {Element | null | undefined} titleNode
 * @param {Element | null | undefined} progressNode
 */
export function publishChallengeDest(setId, titleNode, progressNode) {
    if (!challengeTransition.active || challengeTransition.setId !== setId) {
        return;
    }
    if (titleNode) {
        const s = getComputedStyle(titleNode);
        challengeTransition.destTitle = {
            ...rectOf(titleNode),
            fontSize: parseFloat(s.fontSize),
            color: s.color,
            fontWeight: s.fontWeight,
        };
    }
    if (progressNode) {
        challengeTransition.destProgress = rectOf(progressNode);
    }
    challengeTransition.destReady = true;
}

/**
 * Delayed navigation to allow fading out.
 *
 * @param {string} href
 * @param {HTMLElement} preservedNode
 * @param {string} text
 */
export async function fadeOutAndNavigate(href, preservedNode, text) {

    const rect = preservedNode?.getBoundingClientRect();
    floatingTitleRect.top = rect?.top;
    floatingTitleRect.left = rect?.left;
    floatingTitleRect.width = rect?.width;
    floatingTitleRect.height = rect?.height;

    const style = getComputedStyle(preservedNode);


    fadingOut.state = true;
    fadingOut.text = text;
    fadingOut.fontSize.set(parseFloat(style.fontSize), { duration: 0, delay: 0 });
    fadingOut.color = style.color;
    await tick();

    await new Promise((r) => setTimeout(r, 600));
    fadingOut.fontSize.set(36);
    await tick();
    await new Promise((r) => setTimeout(r, 600));

    goto(href);
    fadingOut.state = false;
    await tick();
    fadingOut.text = undefined;
    fadingOut.color = undefined;
}