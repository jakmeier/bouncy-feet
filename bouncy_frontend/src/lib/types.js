
/**
 * @typedef {Object} Features
 * @property {boolean} enableAvatarRotation
 * @property {boolean} enableFreestyleRecording
 * @property {boolean} enableStepRecording
 * @property {boolean} enableDanceCollection
 * @property {boolean} enableDanceCreator
 * @property {boolean} enableDevView
 * 
 * @typedef {Object} Point
 * @property {number} x 
 * @property {number} y
 *
 * @typedef {Object} Line
 * @property {String} id
 * @property {Point} start
 * @property {Point} end
 * @property {number} z
 * @property {any} style
 * 
 * @typedef {Object} Polygon
 * @property {String} id
 * @property {Point[]} points
 * @property {any} style
 *
 * @typedef {Object} Circle
 * @property {number} cx 
 * @property {number} cy 
 * @property {number} r 
 * @property {string} fill 
 *
 * @typedef {Object} Style
 * @property {String} color
 * @property {"inherit" | "round" | "butt" | "square"} linecap
 * @property {number} lineWidth
 * 
 */
