/**
 * @param {Blob} file
 */
export function fileToUrl(file) {
    var reader = new FileReader();

    const promise = new Promise((resolve) => {
        reader.onload = function (e) {
            resolve(e.target.result);
        };
    });

    reader.readAsDataURL(file);

    return promise;
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
