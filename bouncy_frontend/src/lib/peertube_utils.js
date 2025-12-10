import * as api from '$lib/peertube-openapi';
import { VIDEO_PRIVACY } from './peertube';

/**
 * @param {api.VideoPrivacySet} privacy
 * @returns {string}
 */
export function privacySymbol(privacy) {
    switch (privacy) {
        case VIDEO_PRIVACY.PRIVATE:
            return 'public_off';
        case VIDEO_PRIVACY.UNLISTED:
            return 'group';
        case VIDEO_PRIVACY.PUBLIC:
            return 'public';
        default:
            return 'question_mark';
    }
}

/**
 * @param {api.VideoPrivacySet} privacy
 * @returns {string}
 */
export function privacyText(privacy) {
    switch (privacy) {
        case VIDEO_PRIVACY.PRIVATE:
            return 'video.private-description';
        case VIDEO_PRIVACY.UNLISTED:
            return 'video.unlisted-description';
        case VIDEO_PRIVACY.PUBLIC:
            return 'video.public-description';
        default:
            return 'video.unknown-privacy';
    }
}