
/**
 * @typedef {Object} Features
 * @property {boolean} enableAvatarRotation
 * @property {boolean} enableFreestyleRecording
 * @property {(step: string)=>boolean} enableStepRecording
 * @property {boolean} enableDanceCollection
 * @property {boolean} enableDanceCreator
 * @property {boolean} enableCourses
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
 * @typedef {Object} AvatarColoring
 * @property {string} leftColor
 * @property {string} rightColor
 * @property {string} headColor
 * @property {string} bodyColor
 *
 * @typedef {Object} DanceSessionResult
 * @property {number} numSteps
 * @property {number} experience
 * @property {number} duration
 * @property {any} stats
 * @property {number[]} bpms
 *
 * @typedef {Object} StepFilter
 * @property {boolean} uniqueNames
 * @property {string[]} [sources]
 * @property {string} [stepName]
 *
 * @typedef {Object} UserData
 * @property {string} id 
 * @property {string} publicName 
 * @property {number} recordedSteps 
 * @property {number} recordedSeconds 
 * @property {number} recordedDances 
 * @property {number} score 
 * @property {Object} userSteps 
 * @property {UserLessonProgress} userLessonProgress 
 * @property {boolean} consentSendingStats
 * @property {boolean} experimentalFeatures
 *
 * @typedef {Object} UserLessonProgress
 * @property {LessonProgress[]} lessons
 * 
 * @typedef {number} LessonProgress
 */
