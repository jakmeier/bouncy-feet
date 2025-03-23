export function selectMediaMimeType() {
    // While in Firefox webm seems to be default, on Safari this is not
    // supported. Thus, let's try a few codecs and hope one works.
    // Note that "video/mp4" might fail isTypeSupported while more specific types works.
    const preferredTypes = [
        'video/mp4;codecs=avc1',
        'video/mp4;codecs="avc1.42E01E, mp4a.40.2"',
        'video/webm;codecs=vp9',
        'video/webm;codecs=vp8',
        'video/webm',
    ];

    for (const type of preferredTypes) {
        if (MediaRecorder.isTypeSupported(type)) {
            return type;
        }
    }
    return null;
}