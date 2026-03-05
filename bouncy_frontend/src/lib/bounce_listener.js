import * as api from '$lib/peertube-openapi';

/** @param {api.VideoDetails} videoMeta */
export async function detectBpm(videoMeta) {
    if (!videoMeta.files || videoMeta.files.length === 0) {
        console.error("need web video to be available for bpm detection", videoMeta);
        return;
    }
    if (!videoMeta.duration) {
        console.error("video has no duration", videoMeta);
        return;
    }
    // use smallest video, only audio is required anyway
    const file = videoMeta.files.toSorted((f0, f1) => (f0.size || 0) - (f1.size || 0))[0];
    /** @type {string} */
    const url = file.fileUrl;

    // offline context (rendering as fast as possible, not live)
    const audioContext = new AudioContext();
    const response = await fetch(url, { mode: "cors" });
    if (!response.ok) {
        console.error("failed reading video source with status", response.status);
        const errBody = await response.text();
        console.error(errBody, response);
        return;
    }
    const arrayBuffer = await response.arrayBuffer();
    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

    // const sampleRate = 44100;
    // 8k sample rate is plenty for the Hz range we care about
    const sampleRate = 8000;
    const offlineCtx = new OfflineAudioContext(2, sampleRate * videoMeta.duration, sampleRate);

    const source = offlineCtx.createBufferSource();
    source.buffer = audioBuffer;

    // const filter = lowpassFilter(offlineCtx);
    const filter = bandpassFilter(offlineCtx);

    source.connect(filter);
    filter.connect(offlineCtx.destination);

    source.start(0);

    /** @type {{ ms: number; count: number; offset: number }[]} */
    let candidates = [];
    /** @type {(reason?: any) => void} */
    let resolve;
    /** @type {(reason?: any) => void} */
    let reject;
    const promise = new Promise((res, rej) => { resolve = res; reject = rej; });
    offlineCtx.oncomplete = function (event) {
        try {
            const filteredBuffer = event.renderedBuffer;

            // There might be two or more channels. Ideally, I should average them.
            // But using only one should be fine.
            const data = filteredBuffer.getChannelData(0);

            const minStep = Math.floor(sampleRate / 5); // 200ms

            // if (true) {
            // console.log("simpleThresholdPeaks + intervalHistogramBpmDetection");

            const peakIndices = simpleThresholdPeaks(data, minStep);
            // histogram in range minStep to X * minStep
            const bucketPrecision = Math.ceil(sampleRate / 200); // 5ms
            candidates = intervalHistogramBpmDetection(sampleRate, minStep, peakIndices, bucketPrecision);

            // candidates.forEach(c => {
            //     console.log(c.ms, "ms:", c.count);
            // });

            // }

            // if (true) {
            //     console.log("energyEnvelopeDerivative + localMaxPeaks + intervalHistogramBpmDetection");

            //     const envelopeSizeMs = 1;
            //     const derivatives = energyEnvelopeDerivative(data, envelopeSizeMs, sampleRate);
            //     const peakIndices = localMaxPeaks(derivatives, minStep, envelopeSizeMs, sampleRate);
            //     console.log("derivatives", derivatives);
            //     console.log("peakIndices", peakIndices);

            //     const envelopeSampleRate = 1000 / envelopeSizeMs;
            //     const envelopeMinStep = 200 / envelopeSizeMs; // 200ms
            //     const bucketPrecision = Math.ceil(envelopeSampleRate / 500); // 2ms
            //     const candidates = intervalHistogramBpmDetection(envelopeSampleRate, envelopeMinStep, peakIndices, bucketPrecision);
            //     candidates.forEach(c => {
            //         console.log(c.ms, "ms:", c.count);
            //     });
            // }

            // if (true) {
            //     console.log("energyEnvelopeDerivative + autocorrelation");

            //     const envelopeSizeMs = 10;
            //     const derivatives = energyEnvelopeDerivative(data, envelopeSizeMs, sampleRate);

            //     const downsampleFactor = 2;
            //     const down = downsample(derivatives, downsampleFactor);

            //     const ac = autocorrelate(down);

            //     const envelopeRate = 1000 / envelopeSizeMs / downsampleFactor;
            //     const candidates = findBestTempo(ac, envelopeRate);

            //     candidates.forEach(c => {
            //         console.log(c.ms, "ms:", c.count);
            //     });
            // }
            resolve();
        } catch { reject() }
    };

    await offlineCtx.startRendering();
    await promise;

    const best = candidates.sort((a, b) => b.count - a.count)[0];

    return {
        ms: best.ms,
        bpm: 60000 / best.ms,
        offset: best.offset,
    };
}

/**
 * @param {number} sampleRate
 * @param {number} minStep
 * @param {number[]} peakIndices
 * @param {number} bucketPrecision
 * @returns {{ ms: number; count: number; offset: number}[]}
 */
function intervalHistogramBpmDetection(sampleRate, minStep, peakIndices, bucketPrecision) {
    const minBucket = minStep;
    const maxBucket = 10 * minStep;
    const buckets = Array(Math.ceil((maxBucket - minBucket) / bucketPrecision + 1)).fill(0);

    for (var j = 0; j < peakIndices.length - 1; j++) {
        // look at intervals up to 5 peak detections ahead (5 minSteps should be plenty)
        for (var lookAhead = 1; lookAhead < 5; lookAhead++) {
            if (j + lookAhead > peakIndices.length) {
                break;
            }
            const delta = peakIndices[j + lookAhead] - peakIndices[j];
            const b = Math.max(0, Math.floor((delta - minStep) / bucketPrecision));
            buckets[Math.min(b, buckets.length - 1)] += 1;
        }
    }

    /**
     * @type {{ ms: number; count: any; }[]}
     */
    const candidates = [];
    buckets.forEach((count, b) => {
        // if (count > 0) {
        if (count > 1) {
            candidates.push({
                ms: Math.round((b * bucketPrecision + minBucket) / sampleRate * 1000),
                count,
            });
        }
    });

    // find offset in ms (= signal phase)
    const samplesPerMs = sampleRate / 1000;
    return candidates.map(c => {
        const samplesPerBeat = c.ms * samplesPerMs;
        const histogram = Array(samplesPerBeat).fill(0);

        peakIndices.forEach(peakIndex => {
            const phase = peakIndex % samplesPerBeat;
            histogram[phase] += 1;
        });

        const bestSampleOffset = histogram.indexOf(Math.max(...histogram));

        return {
            ...c,
            offset: bestSampleOffset / sampleRate * 1000,
        };
    });
}

/**
 * @param {OfflineAudioContext} offlineCtx
 */
function lowpassFilter(offlineCtx) {
    const filter = offlineCtx.createBiquadFilter();
    filter.type = "lowpass";
    // filter.Q = 1.0; // db
    filter.frequency.setValueAtTime(70, offlineCtx.currentTime); // Hz
    return filter;
}

/**
 * @param {OfflineAudioContext} offlineCtx
 */
function bandpassFilter(offlineCtx) {
    const filter = offlineCtx.createBiquadFilter();
    filter.type = "bandpass";
    // Goal: 60Hz - 200Hz
    // center = 130Hz
    filter.frequency.setValueAtTime(130, offlineCtx.currentTime); // Hz
    // range = +/-70Hz
    filter.Q.setValueAtTime(70, offlineCtx.currentTime); // Hz, I think, docs are unclear
    return filter;
}

/**
 * @param {Float32Array<ArrayBuffer>} data
 * @param {number} minStep
 * @returns {number[]}
 */
function simpleThresholdPeaks(data, minStep) {
    const threshold = percentileThresholdGuesstimate(data);
    // console.log("data", data);
    // console.log("using threshold", threshold);

    const peakIndices = [];
    var length = data.length;
    for (var i = 0; i < length; i++) {
        if (Math.abs(data[i]) > threshold) {
            peakIndices.push(i);
            i += minStep;
        }
    }
    return peakIndices;
}

/**
 * @param {Float32Array<ArrayBuffer>} data
 * @param {number} envelopeSizeMs
 * @param {number} sampleRate
 * @returns {number[]}
 */
function energyEnvelopeDerivative(data, envelopeSizeMs, sampleRate) {

    const envelopeSize = Math.floor(sampleRate * envelopeSizeMs / 1000);
    const hopSize = envelopeSize; // no overlap to keep it simple

    // Compute short-term energy in envelops
    const energy = [];
    for (let i = 0; i < data.length - envelopeSize; i += hopSize) {
        let sum = 0;
        for (let j = 0; j < envelopeSize; j++) {
            const s = data[i + j];
            sum += s * s;
        }
        energy.push(sum);
    }
    // console.log("energy", energy);

    // smoothen envelope
    const windowSize = 4;
    const smoothed = [];
    for (let i = 0; i < energy.length; i++) {
        let sum = 0;
        let count = 0;

        for (let j = -windowSize; j <= windowSize; j++) {
            const idx = i + j;
            if (idx >= 0 && idx < energy.length) {
                sum += energy[idx];
                count++;
            }
        }

        smoothed.push(sum / count);
    }
    // console.log("smoothed", smoothed);

    // get first derivate of energy with half-wave rectification, for detecting rising edges
    const derivative = [0];
    for (let i = 1; i < smoothed.length; i++) {
        const diff = smoothed[i] - smoothed[i - 1];
        // derivative.push(Math.max(0, diff)); // half-wave rectification
        derivative.push(diff); // half-wave rectification
    }
    // console.log("derivative", derivative);
    return derivative;

}
/**
 * detect peaks at local maximums
 *
 * @param {number[]} derivative
 * @param {number} minStep
 * @param {number} envelopeSizeMs
 * @param {number} sampleRate
 * @returns {number[]}
 */
function localMaxPeaks(derivative, minStep, envelopeSizeMs, sampleRate) {

    const peaks = [];
    const envelopeSize = Math.floor(sampleRate * envelopeSizeMs / 1000);
    const envelopeMinStep = minStep / envelopeSize;

    let lastPeak = -Infinity;
    for (let i = 0; i < derivative.length - 1; i++) {
        const isLocalMax =
            derivative[i] <= 0 &&
            derivative[i + 1] > 0;
        // derivative[i] > derivative[i - 1] &&
        // derivative[i] > derivative[i + 1];

        const farEnough = i - lastPeak > envelopeMinStep;

        if (isLocalMax && farEnough) {
            peaks.push(i);
            lastPeak = i;
        }
    }

    return peaks;

}

function downsample(array, factor) {
    const result = [];

    for (let i = 0; i < array.length; i += factor) {
        result.push(array[i]);
    }

    return result;
}

/** @param {string | any[]} signal */
function autocorrelate(signal) {
    const n = signal.length;
    const result = new Array(n).fill(0);

    for (let lag = 0; lag < n; lag++) {
        let sum = 0;

        for (let i = 0; i < n - lag; i++) {
            sum += signal[i] * signal[i + lag];
        }

        result[lag] = sum;
    }

    return result;
}


/**
 * @param {any[]} autocorr
 * @param {number} envelopeRate
 * @returns {{ ms: number; count: number; }[]}
 */
function findBestTempo(autocorr, envelopeRate, minBPM = 70, maxBPM = 180) {

    const minLag = Math.floor(envelopeRate * 60 / maxBPM);
    const maxLag = Math.floor(envelopeRate * 60 / minBPM);

    const candidates = [];
    for (let lag = minLag; lag <= maxLag; lag++) {

        candidates.push({
            ms: lag / envelopeRate * 1000,
            count: autocorr[lag],
        });
    }

    // only show top 5
    const threshold = candidates.map(c => c.count).toSorted()[candidates.length - 5];
    return candidates.filter(a => a.count >= threshold);
}

// TODO: how to find threshold? for now, pick a range in the middle and use a percentile
/**
 * @param {any[] | Float32Array<ArrayBuffer>} data
 */
function percentileThresholdGuesstimate(data, percentile = 0.95) {
    const midpoint = data.length / 2;
    const range = data.slice(midpoint, Math.min(midpoint + 100000, data.length)).map(n => Math.abs(n)).toSorted();
    const threshold = range[Math.floor(range.length * percentile)];
    return threshold;
}
