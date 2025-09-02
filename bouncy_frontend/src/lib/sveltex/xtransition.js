import { cubicOut } from "svelte/easing";

/**
 * @typedef {Object} TransitionOptions
 * @property {number} duration
 * @property {number} delay
 * @property {number} easing
 */

/**
 * @param {*} node 
 * @param {import("svelte/transition").ScaleParams} options 
 * @returns {import("svelte/transition").TransitionConfig}
 */
export function scaleY(
    node,
    { delay = 0, duration = 400, easing = cubicOut, start = 0, opacity = 0 } = {}
) {
    const style = getComputedStyle(node);
    const target_opacity = +style.opacity;
    const transform = style.transform === 'none' ? '' : style.transform;
    const sd = 1 - start;
    const od = target_opacity * (1 - opacity);
    return {
        delay, duration, easing,
        css: (_t, u) => `
                transform: ${transform} scaleY(${1 - sd * u});
                opacity: ${target_opacity - od * u}
            `
    }
}