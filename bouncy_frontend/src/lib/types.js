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
 * @typedef {Object} PageColoring
 * @property {string} pageColor
 * @property {string} secondaryColor
 * @property {string} fontColor
 * @property {string} danceFloorColor
 * @property {string} fontOnDanceFloorColor
 * 
 * @typedef {Object} AvatarStyleContext
 * @property {AvatarColoring} coloring
 * @property {AvatarHeadStyle} headStyle
 * @property {AvatarBodyShape} bodyShape
 * @property {PageColoring} pageColoring
 *
 * @typedef {Object} DanceSessionResult
 * @property {number} numSteps
 * @property {number} hits
 * @property {number} misses
 * @property {number} duration
 * @property {Date} timestamp
 *
 * @typedef {Object} StepFilter
 * @property {boolean} uniqueNames
 * @property {string[]} [sources]
 * @property {string} [stepName]
 *
 * @typedef {Object} UserData
 * @property {string} openid 
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
 * @typedef {Object} PublicUserResponse
 * @property {number} id
 * @property {string} display_name
 *
 * @typedef {Object} AccessToken
 * @property {string} token_type
 * @property {string} access_token
 * @property {string} refresh_token
 *
 * @typedef {Object} PwaAuth
 * @property {null|AccessToken} peerTubeToken
 *
 * @typedef {import('$lib/stores/ApiUser.svelte.js').ApiUser} ApiUser
 * 
 * @typedef {Object} UserContextData
 * @property {UserData} user
 * @property {ApiUser} [apiUser] -- this may be a guest session
 * @property {FullUser} [fullUser] -- this is a fully onboarded user with a Keycloak entry and a PeerTube account
 * @property {UserAuthState} authState
 *
 * @typedef {Object} FullUser
 * @property {PwaAuth} pwaAuth,
 * @property {()=>boolean} isLoggedInToApi -- Has an active, non-expired PeerTube API session. (Reactive $derived state)
 * @property {()=>{}} refreshPeerTubeUser
 * @property {()=>void} logout
 * @property {Promise<import("$lib/peertube-openapi").User | undefined>} peerTubeUser
 * @property {() => Promise<BfError | { accessToken: string }>} peerTubeToken
 *
 * @typedef {import('$lib/enum_types').UserAuthState} UserAuthState
 *
 * @typedef {Object} UserMetaResponse
 * @property {String} key_name
 * @property {String} key_value
 * @property {String} [last_modified] -- Option<chrono::NaiveDateTime>
 * @property {number} version_nr -- i16
 *
 * @typedef {Object} BfError -- for translated display to the user
 * @property {string} title -- id to put inside $t() for translation
 * @property {string} description -- id to put inside $t() for translation
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
 * @property {string} selectedCoach
 * @property {LocalFlags} flags
 * 
 * @typedef {Object} LocalFlags
 * @property {boolean} seenNoUploadHint
 * 
 * @typedef {Object} ClientSessionData
 * @property {string} id
 * @property {string} secret
 * @property {DynUserMeta} meta
 * 
 * @typedef {Object} UserMeta -- old in-memory meta values
 * @property {OnboardingState} [onboarding]
 * 
 * @typedef {Object.<string, string>} DynUserMeta -- new in-memory meta values,
 * mapping (untyped) keys to stringified values for quick in-memory access
 * 
 * @typedef {Object} TranslatedText
 * @property {string} de
 * @property {string} en
 * 
 * @typedef {Object} PeerTubePlayerState
 * @property {number} position - Current playback position in seconds.
 * @property {number} volume - Volume level (0.0 to 1.0).
 * @property {string} duration - Total duration of the video (as stringified float).
 * @property {"playing" | "paused" | "ended"} playbackState - Current playback state.
 *
 * @typedef {Object} ClubsContextData
 * @property {ClubsData} clubsData
 * 
 * @typedef {Object} Club
 * @property {number} id
 * @property {string} name
 * @property {string} description
 * @property {string} [avatar] -- full PeerTube path
 * @property {AvatarStyleContext} style
 * @property {object} stats
 *
 * @typedef {Object} ClubsData
 * @property {Club[]} mine
 * @property {Club[]} public
 * @property {ClubDetailsResponse} [currentClubDetails]
 * @property {boolean} loadedForUser
 *
 *
 * @typedef {object} ClubDetailsResponse
 * @property {PublicUserResponse[]} admins
 * @property {number} num_members
 * @property {number} [channel_id]
 * @property {string} [channel_handle]
 * @property {PlaylistInfo} [main_playlist]
 * @property {PlaylistInfo[]} public_playlists
 * @property {string} [web_link]
 * @property {PrivateClubDetails} [private]
 * 
 * @typedef {object} PrivateClubDetails
 * @property {PlaylistInfo[]} private_playlists
 * @property {PublicUserResponse[]} members
 *
 *
 * @typedef {object} EditableClubDetails
 * @property {string} description
 * @property {string} [url]
 *
 *
 * @typedef {Object} PlaylistInfo
 * @property {number} id
 * @property {string} short_uuid
 * 
 */
