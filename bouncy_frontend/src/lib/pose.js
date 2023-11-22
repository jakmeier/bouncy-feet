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

// some state is owned by the lib because it is heavy to load, we want it to be a singleton
let mp = null;

export class PoseDetection {
    // don't use this directly, always use PoseDetection.new()
    constructor(consumer) {
        this.consumer = consumer;
    }

    /**
     * Start a detector with a callback that is called on every frame result.
     * 
     * @param {import('@mediapipe/tasks-vision').PoseLandmarkerCallback} consumer 
     * @returns 
     */
    static async new(consumer) {
        if (mp === null) {
            mp = await initMediaPipeBackend();
        }
        return new PoseDetection(consumer);
    }

    /**
     * 
     * @param {HTMLVideoElement} videoElement
     */
    trackVideoFrame(videoElement) {
        if (mp) {
            mp.detectForVideo(videoElement, videoElement.currentTime, this.resultCallback.bind(this));
        }
    }

    /**
     * @param {import('@mediapipe/tasks-vision').PoseLandmarkerResult} result
     */
    resultCallback(result) {
        this.consumer(result);
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
