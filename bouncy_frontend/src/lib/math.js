/**
 * @param {{ x: number; y: number; }} a
 * @param {{ x: number; y: number; }} b
 */
export function distance2d(a, b) {
    return Math.hypot(a.x - b.x, a.y - b.y);
}

/**
 * Vector in clockwise angle format, x is horizontal and y vertical
 * @param {{ x: number; y: number; }} start
 * @param {number} alpha
 * @param {number} length
 */
export function add2dVector(start, alpha, length) {
    return {
        x: start.x + Math.cos(alpha) * length,
        y: start.y + Math.sin(alpha) * length,
    };
}
