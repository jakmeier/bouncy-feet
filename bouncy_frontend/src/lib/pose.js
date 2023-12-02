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
import { Cartesian3d, Keypoints, KeypointsSide, loadPoseFile } from './instructor/bouncy_instructor';


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
    return new Keypoints(left, right);
}

export class PoseDetection {
    // don't use this directly, always use PoseDetection.new()
    /**
     * @param {(result: import("@mediapipe/tasks-vision").PoseLandmarkerResult, timestamp: number) => void} consumer
     * @param {PoseLandmarker} mp
     */
    constructor(consumer, mp) {
        this.consumer = consumer;
        this.tZero = new Date().getTime();
        this.tPrev = -1;
        // media pipe `PoseLandmarker`
        this.mp = mp;
    }

    /**
     * Start a detector with a callback that is called on every frame result.
     * 
     * @param {(result: import( '@mediapipe/tasks-vision').PoseLandmarkerResult, timestamp: number) => void} consumer 
     * @returns PoseDetection
     */
    static async new(consumer) {
        const mp = await initMediaPipeBackend();
        await loadPoseFile('/pose.ron').catch((e) => console.error(e));
        return new PoseDetection(consumer, mp);
    }


    /**
     * 
     * @param {import('@mediapipe/tasks-vision').ImageSource} videoElement
     * @param {number} timestamp
    */
    trackFrame(videoElement, timestamp) {
        if (timestamp === undefined || timestamp === null) {
            timestamp = this.currentTimestamp();
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
        this.mp.detectForVideo(videoElement, timestamp, ((result) => this.resultCallback(result, timestamp)));
    }

    currentTimestamp() {
        return new Date().getTime() - this.tZero;
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
        "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@latest/wasm"
    );

    return await PoseLandmarker.createFromOptions(
        wasmSet,
        {
            baseOptions: {
                // loading from a path (could also load from buffer)
                modelAssetPath: 'models/pose_landmarker_full.task',
            },
            runningMode: "VIDEO",
            numPoses: 1,
            minPoseDetectionConfidence: 0.2,
            minPosePresenceConfidence: 0.2,
            minTrackingConfidence: 0.2,
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