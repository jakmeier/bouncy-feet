/**
 * @param {Blob} file
 */
export function fileToUrl(file) {
    return URL.createObjectURL(file);
}

/**
 * @param {HTMLVideoElement} video
 */
export function waitForVideoMetaLoaded(video) {
    return new Promise((resolve) => {
        video.onloadedmetadata = () => {
            resolve(video);
        };
    });
}
