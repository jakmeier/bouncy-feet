/*
 * Pose detection from image and video.
 *
 * This file includes the glue code to integrate a third-party solution. The
 * hard work is done by Googl's MediaPipe and the pre-trained model originally
 * published under the name of BlazePose.
 * 
 * Useful links:
 * [Config options](https://developers.google.com/mediapipe/solutions/vision/pose_landmarker/index#configurations_options)
 * [Pose landmark detection guide for Web](https://developers.google.com/mediapipe/solutions/vision/pose_landmarker/web_js)
 * [Mediapipe source](https://github.com/google/mediapipe/tree/e7edd97effcc0dcdb714a45eef3590923397e6d0/mediapipe/tasks/web/vision/pose_landmarker)
 * [Model source](https://developers.google.com/mediapipe/solutions/vision/pose_landmarker/index#models)
 */

import { PoseLandmarker, FilesetResolver } from '@mediapipe/tasks-vision';
import { Cartesian3d, Keypoints, KeypointsSide } from '$lib/instructor/bouncy_instructor';
import { base } from '$app/paths';

/**
 * @param {import("@mediapipe/tasks-vision").Landmark[]} landmarks
 */
export function landmarksToKeypoints(landmarks) {
    const left = new KeypointsSide(
        coordinate(I.LEFT_SHOULDER, landmarks),
        coordinate(I.LEFT_HIP, landmarks),
        coordinate(I.LEFT_KNEE, landmarks),
        coordinate(I.LEFT_ANKLE, landmarks),
        coordinate(I.LEFT_HEEL, landmarks),
        coordinate(I.LEFT_FOOT_INDEX, landmarks),
        coordinate(I.LEFT_ELBOW, landmarks),
        coordinate(I.LEFT_WRIST, landmarks),
    );

    const right = new KeypointsSide(
        coordinate(I.RIGHT_SHOULDER, landmarks),
        coordinate(I.RIGHT_HIP, landmarks),
        coordinate(I.RIGHT_KNEE, landmarks),
        coordinate(I.RIGHT_ANKLE, landmarks),
        coordinate(I.RIGHT_HEEL, landmarks),
        coordinate(I.RIGHT_FOOT_INDEX, landmarks),
        coordinate(I.RIGHT_ELBOW, landmarks),
        coordinate(I.RIGHT_WRIST, landmarks),
    );
    // This uses visibility from mediapipe. It doesn't work well, since the body
    // itself may block visibility to other parts of the body, for example when
    // standing sideways, only half the body is visible.
    // const fullyVisible = landmarks.find((landmark) => landmark.visibility < 0.5) === undefined;

    // Instead, check if all relevant body parts are in the frame.
    const fullyVisible = MAIN_LANDMARKS.find((index) => {
        const c =
            coordinate(index, landmarks);
        return Math.abs(c.x) > 1.0 ||
            Math.abs(c.y) > 1.0;
    }) === undefined;
    return new Keypoints(left, right, fullyVisible);
}

export class PoseDetection {
    // don't use this directly, always use PoseDetection.new()
    /**
     * @param {(result: import("@mediapipe/tasks-vision").PoseLandmarkerResult, timestamp: number) => void} consumer
     * @param {PoseLandmarker} mp
     * @param {number} tZero
     */
    constructor(consumer, mp, tZero) {
        this.consumer = consumer;
        // Used to be an offset subtracted from all timestamps. But this was
        // removed to simplify timestamp handling. Now it's only used internally
        // because mediapipe seems to have problems with absolute timestamps.
        this.tZero = tZero;
        this.tPrev = -1;
        // media pipe `PoseLandmarker`
        this.mp = mp;
    }

    /**
     * Start a detector with a callback that is called on every frame result.
     * 
     * @param {(result: import( '@mediapipe/tasks-vision').PoseLandmarkerResult, timestamp: number) => void} consumer 
     * @param {number|undefined} tZero
     * @returns PoseDetection
     */
    static async new(consumer, tZero) {
        const mp = await initMediaPipeBackend();
        tZero = tZero === undefined ? performance.now() : tZero;
        return new PoseDetection(consumer, mp, tZero);
    }


    /**
     * 
     * @param {TexImageSource} videoElement
     * @param {number | undefined} timestamp
    */
    trackFrame(videoElement, timestamp) {
        if (timestamp === undefined || timestamp === null) {
            timestamp = performance.now();
        }
        if (timestamp <= this.tPrev) {
            if (timestamp < this.tPrev) {
                console.warn("Timestamp is in the past", timestamp, this.tPrev);
            }
            return;
        }
        if (timestamp === 0) {
            timestamp = 1;
        }
        this.tPrev = timestamp;
        this.mp.detectForVideo(videoElement, timestamp - this.tZero, ((result) => this.resultCallback(result, timestamp)));
    }

    /**
     * @param {import('@mediapipe/tasks-vision').PoseLandmarkerResult} result
     * @param {number} timestamp in ms
     */
    resultCallback(result, timestamp) {
        this.consumer(result, timestamp);
    }

    /**
     * Call this to clean up resources.
     */
    close() {
        this.mp.close();
    }
}

async function initMediaPipeBackend() {
    // Note: This loads JS + WASM from the CDN, in the client, while also figuring
    // out if the client supports WASM SIMD or not. This isn't very svelte, I think.
    // But on the other hand, there is not much useful compiler stuff svelte could do.
    // However, it might be worth to load from owned sources even just for removing
    // external dependencies. But then again, depending on jsdelivr.net isn't
    // offending at all.
    // In conclusion, there is no good reason at the moment to do something else.
    const wasmSet = await FilesetResolver.forVisionTasks(
        "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.18/wasm"
    );

    return await PoseLandmarker.createFromOptions(
        wasmSet,
        {
            baseOptions: {
                // loading from a path (could also load from buffer)
                // around 20ms on desktop, around 70ms on phone
                modelAssetPath: `${base}/models/pose_landmarker_lite.task`,
                // around 30ms on desktop, around 100ms on phone
                // modelAssetPath: `${base}/models/pose_landmarker_full.task`,
                // around 100ms detection delay on desktop, unusable for live tracking
                // modelAssetPath: `${base}/models/pose_landmarker_heavy.task`,
            },
            runningMode: "VIDEO",
            numPoses: 1,
            minPoseDetectionConfidence: 0.4,
            minPosePresenceConfidence: 0.4,
            minTrackingConfidence: 0.4,
            outputSegmentationMasks: false,
        });
}

/**
 *
 * @param {number} i
 * @param {import('@mediapipe/tasks-vision').Landmark[]} landmarks
 */
function coordinate(i, landmarks) {
    return new Cartesian3d(landmarks[i].x, landmarks[i].y, landmarks[i].z);
}

export const I = {
    NOSE: 0,
    LEFT_EYE_INNER: 1,
    LEFT_EYE: 2,
    LEFT_EYE_OUTER: 3,
    RIGHT_EYE_INNER: 4,
    RIGHT_EYE: 5,
    RIGHT_EYE_OUTER: 6,
    LEFT_EAR: 7,
    RIGHT_EAR: 8,
    MOUTH_LEFT: 9,
    MOUTH_RIGHT: 10,
    LEFT_SHOULDER: 11,
    RIGHT_SHOULDER: 12,
    LEFT_ELBOW: 13,
    RIGHT_ELBOW: 14,
    LEFT_WRIST: 15,
    RIGHT_WRIST: 16,
    LEFT_PINKY: 17,
    RIGHT_PINKY: 18,
    LEFT_INDEX: 19,
    RIGHT_INDEX: 20,
    LEFT_THUMB: 21,
    RIGHT_THUMB: 22,
    LEFT_HIP: 23,
    RIGHT_HIP: 24,
    LEFT_KNEE: 25,
    RIGHT_KNEE: 26,
    LEFT_ANKLE: 27,
    RIGHT_ANKLE: 28,
    LEFT_HEEL: 29,
    RIGHT_HEEL: 30,
    LEFT_FOOT_INDEX: 31,
    RIGHT_FOOT_INDEX: 32,
};

/** The landmarks that are relevant for pose tracking. */
export const MAIN_LANDMARKS = [
    I.LEFT_SHOULDER,
    I.RIGHT_SHOULDER,
    I.LEFT_ELBOW,
    I.RIGHT_ELBOW,
    I.LEFT_WRIST,
    I.RIGHT_WRIST,
    I.LEFT_HIP,
    I.RIGHT_HIP,
    I.LEFT_KNEE,
    I.RIGHT_KNEE,
    I.LEFT_ANKLE,
    I.RIGHT_ANKLE,
    I.LEFT_HEEL,
    I.RIGHT_HEEL,
    I.LEFT_FOOT_INDEX,
    I.RIGHT_FOOT_INDEX,
];

export const TORSO = [
    I.LEFT_SHOULDER,
    I.RIGHT_SHOULDER,
    I.RIGHT_HIP,
    I.LEFT_HIP,
];

export const bodyOutlinePairs = [
    //torso
    [I.LEFT_SHOULDER, I.RIGHT_SHOULDER],
    [I.LEFT_SHOULDER, I.LEFT_HIP],
    [I.RIGHT_SHOULDER, I.RIGHT_HIP],
    [I.LEFT_HIP, I.RIGHT_HIP],

    // left leg
    [I.LEFT_HIP, I.LEFT_KNEE],
    [I.LEFT_KNEE, I.LEFT_ANKLE],
    [I.LEFT_ANKLE, I.LEFT_FOOT_INDEX],
    [I.LEFT_ANKLE, I.LEFT_HEEL],
    [I.LEFT_FOOT_INDEX, I.LEFT_HEEL],

    // right leg
    [I.RIGHT_HIP, I.RIGHT_KNEE],
    [I.RIGHT_KNEE, I.RIGHT_ANKLE],
    [I.RIGHT_ANKLE, I.RIGHT_FOOT_INDEX],
    [I.RIGHT_ANKLE, I.RIGHT_HEEL],
    [I.RIGHT_FOOT_INDEX, I.RIGHT_HEEL],

    // left arm
    [I.LEFT_SHOULDER, I.LEFT_ELBOW],
    [I.LEFT_ELBOW, I.LEFT_WRIST],
    [I.LEFT_WRIST, I.LEFT_THUMB],
    [I.LEFT_WRIST, I.LEFT_PINKY],
    [I.LEFT_WRIST, I.LEFT_INDEX],

    // right arm
    [I.RIGHT_SHOULDER, I.RIGHT_ELBOW],
    [I.RIGHT_ELBOW, I.RIGHT_WRIST],
    [I.RIGHT_WRIST, I.RIGHT_THUMB],
    [I.RIGHT_WRIST, I.RIGHT_PINKY],
    [I.RIGHT_WRIST, I.RIGHT_INDEX],
];


export const torsoPairs = [
    [I.LEFT_SHOULDER, I.RIGHT_SHOULDER],
    [I.LEFT_SHOULDER, I.LEFT_HIP],
    [I.RIGHT_SHOULDER, I.RIGHT_HIP],
    [I.LEFT_HIP, I.RIGHT_HIP],
];

export const leftSidePairs = [
    // left leg
    [I.LEFT_HIP, I.LEFT_KNEE],
    [I.LEFT_KNEE, I.LEFT_ANKLE],
    [I.LEFT_ANKLE, I.LEFT_FOOT_INDEX],
    [I.LEFT_ANKLE, I.LEFT_HEEL],
    [I.LEFT_FOOT_INDEX, I.LEFT_HEEL],

    // left arm
    [I.LEFT_SHOULDER, I.LEFT_ELBOW],
    [I.LEFT_ELBOW, I.LEFT_WRIST],
    [I.LEFT_WRIST, I.LEFT_THUMB],
    [I.LEFT_WRIST, I.LEFT_PINKY],
    [I.LEFT_WRIST, I.LEFT_INDEX],
];

export const rightSidePairs = [
    // right leg
    [I.RIGHT_HIP, I.RIGHT_KNEE],
    [I.RIGHT_KNEE, I.RIGHT_ANKLE],
    [I.RIGHT_ANKLE, I.RIGHT_FOOT_INDEX],
    [I.RIGHT_ANKLE, I.RIGHT_HEEL],
    [I.RIGHT_FOOT_INDEX, I.RIGHT_HEEL],

    // right arm
    [I.RIGHT_SHOULDER, I.RIGHT_ELBOW],
    [I.RIGHT_ELBOW, I.RIGHT_WRIST],
    [I.RIGHT_WRIST, I.RIGHT_THUMB],
    [I.RIGHT_WRIST, I.RIGHT_PINKY],
    [I.RIGHT_WRIST, I.RIGHT_INDEX],
];