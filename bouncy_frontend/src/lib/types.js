
/**
 * @typedef {Object} Features
 * @property {boolean} enableAvatarRotation
 * @property {boolean} enableFreestyleRecording
 * @property {(step: string)=>boolean} enableStepRecording
 * @property {boolean} enableDanceCollection
 * @property {boolean} enableDanceCreator
 * @property {boolean} enableEditorPage
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
 * @property {Style} style
 *
 * @typedef {Object} Path
 * @property {String} id
 * @property {Point[]} points
 * @property {number} z
 * @property {Style} style
 * 
 * @typedef {Object} Polygon
 * @property {String} id
 * @property {Point[]} points
 * @property {Style} style
 *
 * @typedef {Object} Circle
 * @property {number} cx 
 * @property {number} cy 
 * @property {number} r 
 * @property {string} fill 
 * @property {string} stroke 
 * @property {number} strokeWidth 
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
 *
 * @typedef {Object} AvatarHeadStyle
 * @property {string} shape
 * @property {number} size
 * @property {number} strokeWidth
 * 
 * @typedef {Object} AvatarBodyShape
 * @property {number} strokeWidth
 * TODO: add these later
 * property {number} height
 * property {number} width
 * 
 * @typedef {Object} AvatarStyleContext
 * @property {AvatarColoring} coloring
 * @property {AvatarHeadStyle} headStyle
 * @property {AvatarBodyShape} bodyShape
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
 * @typedef {Object} UserContextData
 * @property {import('svelte/store').Writable<UserData>} store,
 * @property {ClientSession} clientSession,
 * @property {any} setUserMeta,
 * @property {any} computeDanceStats,
 * @property {any} addDanceToStats,
 * @property {any} recordFinishedLesson,
 *
 * @typedef {Object} UserLessonProgress
 * @property {LessonProgress[]} lessons
 * 
 * @typedef {number} LessonProgress
 * 
 * @typedef {Object} WeightedPoseLimb
 * @property {string} name
 * @property {number} index
 * @property {number} weight
 * 
 * @typedef {Object} Song
 * @property {string} id
 * @property {number} bpm
 * @property {number} author
 * @property {number} title
 * 
 * @typedef {string} OnboardingState
 * 
 * @typedef {Object} LocalState
 * @property {AvatarStyleContext} avatarStyle
 * 
 * @typedef {Object} ClientSession
 * @property {string} id
 * @property {string} secret
 * @property {UserMeta} meta
 * 
 * @typedef {Object} UserMeta
 * @property {OnboardingState} [onboarding]
 */
