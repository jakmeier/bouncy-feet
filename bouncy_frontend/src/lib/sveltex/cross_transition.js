/**
 * The `crossfade` function copied and edited to have no opacity fading or size scaling.
 *
 * @param {CrossfadeParams & {
* 	fallback?: (node: Element, params: CrossfadeParams, intro: boolean) => TransitionConfig;
* }} params
* @returns {[(node: any, params: CrossfadeParams & { key: any; }) => () => TransitionConfig, (node: any, params: CrossfadeParams & { key: any; }) => () => TransitionConfig]}
*/
export function noFadeCrossfade({ fallback, ...defaults }) {
    /** @type {Map<any, Element>} */
    const to_receive = new Map();
    /** @type {Map<any, Element>} */
    const to_send = new Map();

    /**
     * @param {Element} from_node
     * @param {Element} node
     * @param {CrossfadeParams} params
     * @returns {TransitionConfig}
     */
    function crossfade(from_node, node, params) {
        const {
            delay = 0,
            duration = /** @param {number} d */ (d) => Math.sqrt(d) * 30,
            easing = cubic_out,
        } = assign(assign({}, defaults), params);
        const from = from_node.getBoundingClientRect();
        const to = node.getBoundingClientRect();
        const dx = from.left - to.left;
        const dy = from.top - to.top;
        const d = Math.sqrt(dx * dx + dy * dy);
        const style = getComputedStyle(node);
        const transform = style.transform === 'none' ? '' : style.transform;
        return {
            delay,
            duration: typeof duration === 'function' ? duration(d) : duration,
            easing,
            css: (t, u) => `
        transform-origin: top left;
        transform: ${transform} translate(${u * dx}px,${u * dy}px);
      `,
        };
    }

    /**
     * @param {Map<any, Element>} items
     * @param {Map<any, Element>} counterparts
     * @param {boolean} intro
     * @returns {(node: any, params: CrossfadeParams & { key: any; }) => () => TransitionConfig}
     */
    function transition(items, counterparts, intro) {
        return (node, params) => {
            items.set(params.key, node);
            return () => {
                if (counterparts.has(params.key)) {
                    const other_node = counterparts.get(params.key);
                    counterparts.delete(params.key);
                    return crossfade(/** @type {Element} */(other_node), node, params);
                }
                // if the node is disappearing altogether
                // (i.e. wasn't claimed by the other list)
                // then we need to supply an outro
                items.delete(params.key);
                return fallback && fallback(node, params, intro);
            };
        };
    }
    return [
        transition(to_send, to_receive, false),
        transition(to_receive, to_send, true),
    ];
}

/**
* @template T
* @template S
* @param {T} tar
* @param {S} src
* @returns {T & S}
*/
function assign(tar, src) {
    // @ts-ignore
    for (const k in src) tar[k] = src[k];
    return /** @type {T & S} */ (tar);
}

/** @param {number} t */
function cubic_out(t) {
    const f = t - 1.0;
    return f * f * f + 1.0;
}
