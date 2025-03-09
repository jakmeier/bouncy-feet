import { derived, writable } from 'svelte/store';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export_0(addHeapObject(e));
    }
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_4.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_4.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}
/**
 * @param {number} random_seed
 * @param {string} lang
 */
export function init(random_seed, lang) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(lang, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.init(retptr, random_seed, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} url
 * @returns {Promise<void>}
 */
export function loadPoseFile(url) {
    const ptr0 = passStringToWasm0(url, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.loadPoseFile(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} data
 */
export function loadPoseString(data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.loadPoseString(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} data
 */
export function loadDanceString(data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.loadDanceString(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} url
 * @param {string} source
 * @returns {Promise<void>}
 */
export function loadStepFile(url, source) {
    const ptr0 = passStringToWasm0(url, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(source, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.loadStepFile(ptr0, len0, ptr1, len1);
    return takeObject(ret);
}

/**
 * @param {string} url
 * @returns {Promise<void>}
 */
export function loadDanceFile(url) {
    const ptr0 = passStringToWasm0(url, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.loadDanceFile(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} data
 * @param {string} source
 */
export function loadStepString(data, source) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(source, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        wasm.loadStepString(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} data
 * @param {string} lang
 * @returns {Course}
 */
export function parseCourseString(data, lang) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(data, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(lang, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        wasm.parseCourseString(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return Course.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {PoseWrapper[]}
 */
export function poses() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.poses(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {StepWrapper[]}
 */
export function steps() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.steps(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} source
 * @returns {StepWrapper[]}
 */
export function stepsBySource(source) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(source, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.stepsBySource(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} id
 * @param {boolean} flipped
 * @returns {StepWrapper | undefined}
 */
export function stepById(id, flipped) {
    const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.stepById(ptr0, len0, flipped);
    return ret === 0 ? undefined : StepWrapper.__wrap(ret);
}

/**
 * @param {string} step_name
 * @returns {StepWrapper[]}
 */
export function stepsByName(step_name) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(step_name, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.stepsByName(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @returns {DanceWrapper[]}
 */
export function dances() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.dances(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export_1(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} dance_id
 * @returns {DanceBuilder}
 */
export function danceBuilderFromDance(dance_id) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(dance_id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.danceBuilderFromDance(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return DanceBuilder.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {PoseWrapper[]} poses
 */
export function addLocalPoses(poses) {
    const ptr0 = passArrayJsValueToWasm0(poses, wasm.__wbindgen_export_2);
    const len0 = WASM_VECTOR_LEN;
    wasm.addLocalPoses(ptr0, len0);
}

/**
 * @param {StepWrapper[]} steps
 */
export function loadLocalSteps(steps) {
    const ptr0 = passArrayJsValueToWasm0(steps, wasm.__wbindgen_export_2);
    const len0 = WASM_VECTOR_LEN;
    wasm.loadLocalSteps(ptr0, len0);
}

let cachedUint16ArrayMemory0 = null;

function getUint16ArrayMemory0() {
    if (cachedUint16ArrayMemory0 === null || cachedUint16ArrayMemory0.byteLength === 0) {
        cachedUint16ArrayMemory0 = new Uint16Array(wasm.memory.buffer);
    }
    return cachedUint16ArrayMemory0;
}

function getArrayU16FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint16ArrayMemory0().subarray(ptr / 2, ptr / 2 + len);
}
function __wbg_adapter_22(arg0, arg1, arg2) {
    const ret = wasm.__wbindgen_export_5(arg0, arg1, addHeapObject(arg2));
    return takeObject(ret);
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    wasm.__wbindgen_export_6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_398(arg0, arg1, arg2, arg3) {
    wasm.__wbindgen_export_7(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
 * @enum {1 | 2 | 3 | 4 | 5 | 6 | 7}
 */
export const DetectionFailureReason = Object.freeze({
    /**
     * The last match was too recent to have another match.
     */
    TooEarly: 1, "1": "TooEarly",
    /**
     * The timing is off.
     */
    NotOnBeat: 2, "2": "NotOnBeat",
    /**
     * Detection did not match an expected pose.
     */
    WrongPose: 3, "3": "WrongPose",
    /**
     * No data to run detection against.
     */
    NoData: 4, "4": "NoData",
    /**
     * Currently in a state that does not detect.
     */
    DetectionDisabled: 5, "5": "DetectionDisabled",
    /**
     * No *new* data to run detection against.
     */
    NoNewData: 6, "6": "NoNewData",
    /**
     * Nothing to track, hence nothing to detect.
     */
    NoTrackingTarget: 7, "7": "NoTrackingTarget",
});
/**
 * @enum {1 | 2 | 3 | 4 | 5 | 6}
 */
export const DetectionState = Object.freeze({
    /**
     * Neutral state, not detecting anything.
     */
    Init: 1, "1": "Init",
    /**
     * Dance is positioning themselves, detecting the idle position.
     */
    Positioning: 2, "2": "Positioning",
    /**
     * About to go over to live tracking, playing a countdown audio.
     */
    CountDown: 3, "3": "CountDown",
    /**
     * Tracking current movements.
     */
    LiveTracking: 4, "4": "LiveTracking",
    /**
     * The instructor is showing the next moving.
     */
    InstructorDemo: 5, "5": "InstructorDemo",
    /**
     * No longer tracking but the results of the previous tracking are
     * available.
     */
    TrackingDone: 6, "6": "TrackingDone",
});
/**
 * Define in which direction a pose should be oriented.
 * @enum {0 | 1 | 2 | 3 | 4}
 */
export const Orientation = Object.freeze({
    ToCamera: 0, "0": "ToCamera",
    Right: 1, "1": "Right",
    Away: 2, "2": "Away",
    Left: 3, "3": "Left",
    /**
     * It doesn't matter in which direction the pose is done.
     */
    Any: 4, "4": "Any",
});
/**
 * @enum {0 | 1}
 */
export const PoseDirection = Object.freeze({
    /**
     * Dancer faces the camera.
     */
    Front: 0, "0": "Front",
    /**
     * Dancer faces to their right. (Left in non-mirrored video.)
     */
    Right: 1, "1": "Right",
});
/**
 * Best guess for what the dancer needs to change to fit the pose.
 * @enum {0 | 1 | 2 | 3}
 */
export const PoseHint = Object.freeze({
    DontKnow: 0, "0": "DontKnow",
    LeftRight: 1, "1": "LeftRight",
    ZOrder: 2, "2": "ZOrder",
    WrongDirection: 3, "3": "WrongDirection",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}
 */
export const SkeletonLimb = Object.freeze({
    LeftThigh: 0, "0": "LeftThigh",
    LeftShin: 1, "1": "LeftShin",
    LeftArm: 2, "2": "LeftArm",
    LeftForearm: 3, "3": "LeftForearm",
    LeftFoot: 4, "4": "LeftFoot",
    RightThigh: 5, "5": "RightThigh",
    RightShin: 6, "6": "RightShin",
    RightArm: 7, "7": "RightArm",
    RightForearm: 8, "8": "RightForearm",
    RightFoot: 9, "9": "RightFoot",
});
/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15}
 */
export const SkeletonPoint = Object.freeze({
    LeftShoulder: 0, "0": "LeftShoulder",
    LeftElbow: 1, "1": "LeftElbow",
    LeftWrist: 2, "2": "LeftWrist",
    LeftHip: 3, "3": "LeftHip",
    LeftKnee: 4, "4": "LeftKnee",
    LeftAnkle: 5, "5": "LeftAnkle",
    LeftHeel: 6, "6": "LeftHeel",
    LeftToes: 7, "7": "LeftToes",
    RightShoulder: 8, "8": "RightShoulder",
    RightElbow: 9, "9": "RightElbow",
    RightWrist: 10, "10": "RightWrist",
    RightHip: 11, "11": "RightHip",
    RightKnee: 12, "12": "RightKnee",
    RightAnkle: 13, "13": "RightAnkle",
    RightHeel: 14, "14": "RightHeel",
    RightToes: 15, "15": "RightToes",
});
/**
 * Hint to the UI, which information should be shown to the user during the
 * current section.
 * @enum {1 | 2 | 3}
 */
export const TeacherView = Object.freeze({
    /**
     * Show the instructor, no need to show the user camera.
     */
    InstructorOnly: 1, "1": "InstructorOnly",
    /**
     * The user camera should be shown with a tracked avatar.
     */
    UserCameraWithTracking: 2, "2": "UserCameraWithTracking",
    /**
     * Show nothing
     */
    Off: 3, "3": "Off",
});

const AudioEffectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_audioeffect_free(ptr >>> 0, 1));

export class AudioEffect {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(AudioEffect.prototype);
        obj.__wbg_ptr = ptr;
        AudioEffectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        AudioEffectFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_audioeffect_free(ptr, 0);
    }
    /**
     * When the sound should be played, could be in the future.
     * @returns {number}
     */
    get timestamp() {
        const ret = wasm.__wbg_get_audioeffect_timestamp(this.__wbg_ptr);
        return ret;
    }
    /**
     * When the sound should be played, could be in the future.
     * @param {number} arg0
     */
    set timestamp(arg0) {
        wasm.__wbg_set_audioeffect_timestamp(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get soundId() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.audioeffect_soundId(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const Cartesian2dFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cartesian2d_free(ptr >>> 0, 1));

export class Cartesian2d {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Cartesian2d.prototype);
        obj.__wbg_ptr = ptr;
        Cartesian2dFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Cartesian2dFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cartesian2d_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     */
    constructor(x, y) {
        const ret = wasm.cartesian2d_new(x, y);
        this.__wbg_ptr = ret >>> 0;
        Cartesian2dFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Cartesian2d} other
     * @returns {Cartesian2d}
     */
    add(other) {
        _assertClass(other, Cartesian2d);
        const ret = wasm.cartesian2d_add(this.__wbg_ptr, other.__wbg_ptr);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_cartesian2d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_cartesian2d_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_cartesian2d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_cartesian2d_y(this.__wbg_ptr, arg0);
    }
}

const Cartesian3dFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cartesian3d_free(ptr >>> 0, 1));
/**
 * Coordinate for Keypoints
 *
 * The coordinate system is growing down (y-axis), right (x-axis), and away
 * from the camera (z-axis).
 *
 * See the Keypoints section in bouncy_instructor/coordinates.md for visuals
 * and rationale.
 */
export class Cartesian3d {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Cartesian3d.prototype);
        obj.__wbg_ptr = ptr;
        Cartesian3dFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Cartesian3dFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cartesian3d_free(ptr, 0);
    }
    /**
     * left-right direction
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_cartesian3d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * left-right direction
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_cartesian3d_x(this.__wbg_ptr, arg0);
    }
    /**
     * up-down direction
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_cartesian3d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * up-down direction
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_cartesian3d_y(this.__wbg_ptr, arg0);
    }
    /**
     * distance to camera
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_cartesian3d_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * distance to camera
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_cartesian3d_z(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} z
     */
    constructor(x, y, z) {
        const ret = wasm.cartesian3d_new(x, y, z);
        this.__wbg_ptr = ret >>> 0;
        Cartesian3dFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const CourseFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_course_free(ptr >>> 0, 1));

export class Course {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Course.prototype);
        obj.__wbg_ptr = ptr;
        CourseFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CourseFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_course_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.course_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.course_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get explanation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.course_explanation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {Lesson[]}
     */
    get lessons() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.course_lessons(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {StepWrapper | undefined}
     */
    featuredStep() {
        const ret = wasm.course_featuredStep(this.__wbg_ptr);
        return ret === 0 ? undefined : StepWrapper.__wrap(ret);
    }
    /**
     * @param {number} lesson_index
     * @returns {Tracker | undefined}
     */
    tracker(lesson_index) {
        const ret = wasm.course_tracker(this.__wbg_ptr, lesson_index);
        return ret === 0 ? undefined : Tracker.__wrap(ret);
    }
    /**
     * WIP: Create a training session for the given course. At the moment, it
     * is hard coded to give something for testing.
     * @returns {Tracker}
     */
    trainingTracker() {
        const ret = wasm.course_trainingTracker(this.__wbg_ptr);
        return Tracker.__wrap(ret);
    }
}

const DanceBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dancebuilder_free(ptr >>> 0, 1));

export class DanceBuilder {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DanceBuilder.prototype);
        obj.__wbg_ptr = ptr;
        DanceBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DanceBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dancebuilder_free(ptr, 0);
    }
    /**
     * @param {string} id
     */
    constructor(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dancebuilder_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        DanceBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    length() {
        const ret = wasm.dancebuilder_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {string} id
     */
    setId(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.dancebuilder_setId(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} step_id
     */
    addStep(step_id) {
        const ptr0 = passStringToWasm0(step_id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.dancebuilder_addStep(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} pos
     * @returns {string}
     */
    removeStep(pos) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancebuilder_removeStep(retptr, this.__wbg_ptr, pos);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} pos
     * @param {string} step_id
     */
    insertStep(pos, step_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(step_id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.dancebuilder_insertStep(retptr, this.__wbg_ptr, pos, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} pos
     * @param {boolean} flipped
     */
    setOrientation(pos, flipped) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancebuilder_setOrientation(retptr, this.__wbg_ptr, pos, flipped);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} pos
     * @returns {boolean}
     */
    isFlipped(pos) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancebuilder_isFlipped(retptr, this.__wbg_ptr, pos);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    clear() {
        wasm.dancebuilder_clear(this.__wbg_ptr);
    }
    /**
     * @returns {DanceWrapper}
     */
    danceInfo() {
        const ret = wasm.dancebuilder_danceInfo(this.__wbg_ptr);
        return DanceWrapper.__wrap(ret);
    }
}

const DanceDetectorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dancedetector_free(ptr >>> 0, 1));
/**
 * Contains all information about a dance to be detected and has an interface
 * to be used by a Tracker to match tracked skeletons to it.
 */
export class DanceDetector {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DanceDetectorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dancedetector_free(ptr, 0);
    }
}

const DanceFileBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dancefilebuilder_free(ptr >>> 0, 1));

export class DanceFileBuilder {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DanceFileBuilder.prototype);
        obj.__wbg_ptr = ptr;
        DanceFileBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DanceFileBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dancefilebuilder_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.dancefilebuilder_new();
        this.__wbg_ptr = ret >>> 0;
        DanceFileBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} text
     * @returns {DanceFileBuilder}
     */
    static fromRon(text) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.dancefilebuilder_fromRon(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DanceFileBuilder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {DanceBuilder} dance_builder
     */
    addDance(dance_builder) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dance_builder, DanceBuilder);
            wasm.dancefilebuilder_addDance(retptr, this.__wbg_ptr, dance_builder.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {DanceBuilder} dance_builder
     */
    overwriteDance(dance_builder) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dance_builder, DanceBuilder);
            wasm.dancefilebuilder_overwriteDance(retptr, this.__wbg_ptr, dance_builder.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} id
     */
    removeDance(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.dancefilebuilder_removeDance(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancefilebuilder_buildRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildPrettyRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancefilebuilder_buildPrettyRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {DanceWrapper[]}
     */
    dances() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancefilebuilder_dances(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} dance_id
     * @returns {DanceBuilder}
     */
    danceBuilder(dance_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(dance_id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.dancefilebuilder_danceBuilder(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DanceBuilder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const DanceWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dancewrapper_free(ptr >>> 0, 1));

export class DanceWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DanceWrapper.prototype);
        obj.__wbg_ptr = ptr;
        DanceWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DanceWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dancewrapper_free(ptr, 0);
    }
    /**
     * The unique identifier for the dance.
     * @returns {string}
     */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancewrapper_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {number}
     */
    length() {
        const ret = wasm.dancewrapper_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {StepWrapper[]}
     */
    steps() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.dancewrapper_steps(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} beat
     * @returns {Skeleton | undefined}
     */
    skeleton(beat) {
        const ret = wasm.dancewrapper_skeleton(this.__wbg_ptr, beat);
        return ret === 0 ? undefined : Skeleton.__wrap(ret);
    }
    /**
     * The number of subbeats the dance takes for one repetition.
     * @returns {number}
     */
    get subbeats() {
        const ret = wasm.dancewrapper_subbeats(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * How much the body position deviates from the origin.
     * @param {number} beat
     * @returns {Cartesian2d}
     */
    bodyShift(beat) {
        const ret = wasm.dancewrapper_bodyShift(this.__wbg_ptr, beat);
        return Cartesian2d.__wrap(ret);
    }
}

const DetectedStepFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_detectedstep_free(ptr >>> 0, 1));
/**
 * A step detected on a video feed, ready for JS code to render.
 */
export class DetectedStep {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DetectedStep.prototype);
        obj.__wbg_ptr = ptr;
        DetectedStepFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DetectedStepFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_detectedstep_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get start() {
        const ret = wasm.__wbg_get_detectedstep_start(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set start(arg0) {
        wasm.__wbg_set_detectedstep_start(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get end() {
        const ret = wasm.__wbg_get_detectedstep_end(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set end(arg0) {
        wasm.__wbg_set_detectedstep_end(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get error() {
        const ret = wasm.__wbg_get_detectedstep_error(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set error(arg0) {
        wasm.__wbg_set_detectedstep_error(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.detectedstep_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {PoseApproximation[]}
     */
    get poses() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.detectedstep_poses(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {number}
     */
    get bpm() {
        const ret = wasm.detectedstep_bpm(this.__wbg_ptr);
        return ret;
    }
}

const DetectionResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_detectionresult_free(ptr >>> 0, 1));
/**
 * Result of a step or dance detection.
 *
 * A detection potentially includes a list of steps. It can be displayed in the
 * frontend as is, or provided to a tracker to update the detection after more
 * data has been added.
 */
export class DetectionResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DetectionResult.prototype);
        obj.__wbg_ptr = ptr;
        DetectionResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DetectionResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_detectionresult_free(ptr, 0);
    }
    /**
     * If the newest detection was negative, this fields contains information
     * about the reason.
     * @returns {DetectionFailureReason | undefined}
     */
    get failureReason() {
        const ret = wasm.__wbg_get_detectionresult_failureReason(this.__wbg_ptr);
        return ret === 0 ? undefined : ret;
    }
    /**
     * If the newest detection was negative, this fields contains information
     * about the reason.
     * @param {DetectionFailureReason | null} [arg0]
     */
    set failureReason(arg0) {
        wasm.__wbg_set_detectionresult_failureReason(this.__wbg_ptr, isLikeNone(arg0) ? 0 : arg0);
    }
    /**
     * @returns {number}
     */
    get poseMatches() {
        const ret = wasm.__wbg_get_detectionresult_poseMatches(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set poseMatches(arg0) {
        wasm.__wbg_set_detectionresult_poseMatches(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get poseMisses() {
        const ret = wasm.__wbg_get_detectionresult_poseMisses(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set poseMisses(arg0) {
        wasm.__wbg_set_detectionresult_poseMisses(this.__wbg_ptr, arg0);
    }
    constructor() {
        const ret = wasm.detectionresult_new_default();
        this.__wbg_ptr = ret >>> 0;
        DetectionResultFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {DetectedStep[]}
     */
    steps() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.detectionresult_steps(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {PoseHint}
     */
    poseHint() {
        const ret = wasm.detectionresult_poseHint(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {PoseApproximation | undefined}
     */
    poseError() {
        const ret = wasm.detectionresult_poseError(this.__wbg_ptr);
        return ret === 0 ? undefined : PoseApproximation.__wrap(ret);
    }
}

const ExportedFrameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_exportedframe_free(ptr >>> 0, 1));
/**
 * Information of a recorded frame in RON format.
 *
 * Can be useful for creating new poses, new keypoint inputs for tests, or just
 * for general debugging.
 */
export class ExportedFrame {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ExportedFrame.prototype);
        obj.__wbg_ptr = ptr;
        ExportedFrameFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ExportedFrameFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_exportedframe_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get pose() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.exportedframe_pose(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get keypoints() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.exportedframe_keypoints(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const KeypointsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_keypoints_free(ptr >>> 0, 1));

export class Keypoints {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeypointsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_keypoints_free(ptr, 0);
    }
    /**
     * @returns {KeypointsSide}
     */
    get left() {
        const ret = wasm.__wbg_get_keypoints_left(this.__wbg_ptr);
        return KeypointsSide.__wrap(ret);
    }
    /**
     * @param {KeypointsSide} arg0
     */
    set left(arg0) {
        _assertClass(arg0, KeypointsSide);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypoints_left(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {KeypointsSide}
     */
    get right() {
        const ret = wasm.__wbg_get_keypoints_right(this.__wbg_ptr);
        return KeypointsSide.__wrap(ret);
    }
    /**
     * @param {KeypointsSide} arg0
     */
    set right(arg0) {
        _assertClass(arg0, KeypointsSide);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypoints_right(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {boolean}
     */
    get fullyVisible() {
        const ret = wasm.__wbg_get_keypoints_fullyVisible(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set fullyVisible(arg0) {
        wasm.__wbg_set_keypoints_fullyVisible(this.__wbg_ptr, arg0);
    }
    /**
     * @param {KeypointsSide} left
     * @param {KeypointsSide} right
     * @param {boolean} fully_visible
     */
    constructor(left, right, fully_visible) {
        _assertClass(left, KeypointsSide);
        var ptr0 = left.__destroy_into_raw();
        _assertClass(right, KeypointsSide);
        var ptr1 = right.__destroy_into_raw();
        const ret = wasm.keypoints_new(ptr0, ptr1, fully_visible);
        this.__wbg_ptr = ret >>> 0;
        KeypointsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const KeypointsSideFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_keypointsside_free(ptr >>> 0, 1));

export class KeypointsSide {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(KeypointsSide.prototype);
        obj.__wbg_ptr = ptr;
        KeypointsSideFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeypointsSideFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_keypointsside_free(ptr, 0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get shoulder() {
        const ret = wasm.__wbg_get_keypointsside_shoulder(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set shoulder(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_shoulder(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get hip() {
        const ret = wasm.__wbg_get_keypointsside_hip(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set hip(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_hip(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get knee() {
        const ret = wasm.__wbg_get_keypointsside_knee(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set knee(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_knee(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get ankle() {
        const ret = wasm.__wbg_get_keypointsside_ankle(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set ankle(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_ankle(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get heel() {
        const ret = wasm.__wbg_get_keypointsside_heel(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set heel(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_heel(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get toes() {
        const ret = wasm.__wbg_get_keypointsside_toes(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set toes(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_toes(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get elbow() {
        const ret = wasm.__wbg_get_keypointsside_elbow(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set elbow(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_elbow(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Cartesian3d}
     */
    get wrist() {
        const ret = wasm.__wbg_get_keypointsside_wrist(this.__wbg_ptr);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * @param {Cartesian3d} arg0
     */
    set wrist(arg0) {
        _assertClass(arg0, Cartesian3d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_keypointsside_wrist(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {Cartesian3d} shoulder
     * @param {Cartesian3d} hip
     * @param {Cartesian3d} knee
     * @param {Cartesian3d} ankle
     * @param {Cartesian3d} heel
     * @param {Cartesian3d} toes
     * @param {Cartesian3d} elbow
     * @param {Cartesian3d} wrist
     */
    constructor(shoulder, hip, knee, ankle, heel, toes, elbow, wrist) {
        _assertClass(shoulder, Cartesian3d);
        var ptr0 = shoulder.__destroy_into_raw();
        _assertClass(hip, Cartesian3d);
        var ptr1 = hip.__destroy_into_raw();
        _assertClass(knee, Cartesian3d);
        var ptr2 = knee.__destroy_into_raw();
        _assertClass(ankle, Cartesian3d);
        var ptr3 = ankle.__destroy_into_raw();
        _assertClass(heel, Cartesian3d);
        var ptr4 = heel.__destroy_into_raw();
        _assertClass(toes, Cartesian3d);
        var ptr5 = toes.__destroy_into_raw();
        _assertClass(elbow, Cartesian3d);
        var ptr6 = elbow.__destroy_into_raw();
        _assertClass(wrist, Cartesian3d);
        var ptr7 = wrist.__destroy_into_raw();
        const ret = wasm.keypointsside_new(ptr0, ptr1, ptr2, ptr3, ptr4, ptr5, ptr6, ptr7);
        this.__wbg_ptr = ret >>> 0;
        KeypointsSideFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const LessonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lesson_free(ptr >>> 0, 1));

export class Lesson {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Lesson.prototype);
        obj.__wbg_ptr = ptr;
        LessonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LessonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lesson_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lesson_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get explanation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lesson_explanation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get video() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lesson_video(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {LessonPart[]}
     */
    get parts() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lesson_parts(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get iconUrl() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lesson_iconUrl(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const LessonPartFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lessonpart_free(ptr >>> 0, 1));

export class LessonPart {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LessonPart.prototype);
        obj.__wbg_ptr = ptr;
        LessonPartFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LessonPartFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lessonpart_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get stepName() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lessonpart_stepName(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get explanation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lessonpart_explanation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {StepWrapper}
     */
    get step() {
        const ret = wasm.lessonpart_step(this.__wbg_ptr);
        return StepWrapper.__wrap(ret);
    }
    /**
     * @returns {Uint16Array}
     */
    get bpms() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.lessonpart_bpms(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU16FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 2, 2);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const LimbErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_limberror_free(ptr >>> 0, 1));
/**
 * Self-describing error score for a specific limb
 */
export class LimbError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LimbError.prototype);
        obj.__wbg_ptr = ptr;
        LimbErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LimbErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_limberror_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get error() {
        const ret = wasm.__wbg_get_limberror_error(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set error(arg0) {
        wasm.__wbg_set_limberror_error(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get weight() {
        const ret = wasm.__wbg_get_limberror_weight(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set weight(arg0) {
        wasm.__wbg_set_limberror_weight(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.limberror_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {SkeletonV2} skeleton
     * @returns {RenderableSegment}
     */
    render(skeleton) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(skeleton, SkeletonV2);
            wasm.limberror_render(retptr, this.__wbg_ptr, skeleton.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return RenderableSegment.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const PoseApproximationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_poseapproximation_free(ptr >>> 0, 1));
/**
 * The result of fitting keypoints to poses.
 */
export class PoseApproximation {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PoseApproximation.prototype);
        obj.__wbg_ptr = ptr;
        PoseApproximationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PoseApproximationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_poseapproximation_free(ptr, 0);
    }
    /**
     * Total error between 0.0 and 1.0.
     * @returns {number}
     */
    get error() {
        const ret = wasm.__wbg_get_poseapproximation_error(this.__wbg_ptr);
        return ret;
    }
    /**
     * Total error between 0.0 and 1.0.
     * @param {number} arg0
     */
    set error(arg0) {
        wasm.__wbg_set_poseapproximation_error(this.__wbg_ptr, arg0);
    }
    /**
     * Timestamp for which Keypoints were added
     * @returns {number}
     */
    get timestamp() {
        const ret = wasm.__wbg_get_poseapproximation_timestamp(this.__wbg_ptr);
        return ret;
    }
    /**
     * Timestamp for which Keypoints were added
     * @param {number} arg0
     */
    set timestamp(arg0) {
        wasm.__wbg_set_poseapproximation_timestamp(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * List all limbs, order by how well they fit, best fit first.
     * @returns {LimbError[]}
     */
    limbErrors() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_limbErrors(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {ZError[]}
     */
    zErrors() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_zErrors(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {ZWrongOrderError[]}
     */
    zOrderErrors() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_zOrderErrors(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * List the `n` limbs with the highest error contribution to the pose error.
     * @param {number} n
     * @returns {LimbError[]}
     */
    worstLimbs(n) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_worstLimbs(retptr, this.__wbg_ptr, n);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    debugString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.poseapproximation_debugString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const PoseFileWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_posefilewrapper_free(ptr >>> 0, 1));

export class PoseFileWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PoseFileWrapper.prototype);
        obj.__wbg_ptr = ptr;
        PoseFileWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PoseFileWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_posefilewrapper_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.posefilewrapper_new_empty();
        this.__wbg_ptr = ret >>> 0;
        PoseFileWrapperFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} text
     * @returns {PoseFileWrapper}
     */
    static fromRon(text) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.posefilewrapper_fromRon(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return PoseFileWrapper.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {PoseWrapper[]}
     */
    poses() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.posefilewrapper_poses(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {PoseWrapper} new_pose
     */
    addPose(new_pose) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(new_pose, PoseWrapper);
            wasm.posefilewrapper_addPose(retptr, this.__wbg_ptr, new_pose.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {PoseWrapper} new_pose
     */
    overwritePose(new_pose) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(new_pose, PoseWrapper);
            wasm.posefilewrapper_overwritePose(retptr, this.__wbg_ptr, new_pose.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} id
     */
    removePose(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.posefilewrapper_removePose(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.posefilewrapper_buildRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildPrettyRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.posefilewrapper_buildPrettyRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const PoseWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_posewrapper_free(ptr >>> 0, 1));

export class PoseWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PoseWrapper.prototype);
        obj.__wbg_ptr = ptr;
        PoseWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof PoseWrapper)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PoseWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_posewrapper_free(ptr, 0);
    }
    /**
     * @returns {Skeleton}
     */
    skeleton() {
        const ret = wasm.posewrapper_skeleton(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {Skeleton}
     */
    sideSkeleton() {
        const ret = wasm.posewrapper_sideSkeleton(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.posewrapper_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} lang
     * @returns {string}
     */
    name(lang) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(lang, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.posewrapper_name(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v2 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} name
     * @param {string} lang
     */
    setName(name, lang) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(lang, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        wasm.posewrapper_setName(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
     * @param {SkeletonLimb} field
     * @param {number} degree
     */
    setAngle(field, degree) {
        wasm.posewrapper_setAngle(this.__wbg_ptr, field, degree);
    }
    /**
     * Angle in degree
     * @param {SkeletonLimb} field
     * @returns {number}
     */
    getAngle(field) {
        const ret = wasm.posewrapper_getAngle(this.__wbg_ptr, field);
        return ret;
    }
    /**
     * @param {SkeletonPoint} field
     * @param {number} z
     */
    setZ(field, z) {
        wasm.posewrapper_setZ(this.__wbg_ptr, field, z);
    }
    /**
     * @param {SkeletonPoint} field
     * @returns {number}
     */
    getZ(field) {
        const ret = wasm.posewrapper_getZ(this.__wbg_ptr, field);
        return ret;
    }
    /**
     * @param {SkeletonLimb} field
     * @param {number} weight
     */
    setWeight(field, weight) {
        wasm.posewrapper_setWeight(this.__wbg_ptr, field, weight);
    }
    /**
     * Weight of limb in pose detection
     * @param {SkeletonLimb} field
     * @returns {number}
     */
    getWeight(field) {
        const ret = wasm.posewrapper_getWeight(this.__wbg_ptr, field);
        return ret;
    }
    /**
     * @returns {PoseDirection}
     */
    get direction() {
        const ret = wasm.posewrapper_direction(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {PoseDirection} direction
     */
    setDirection(direction) {
        wasm.posewrapper_setDirection(this.__wbg_ptr, direction);
    }
    /**
     * @returns {number}
     */
    get xShift() {
        const ret = wasm.posewrapper_xShift(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x_shift
     */
    set xShift(x_shift) {
        wasm.posewrapper_set_xShift(this.__wbg_ptr, x_shift);
    }
    /**
     * @returns {number}
     */
    get yShift() {
        const ret = wasm.posewrapper_yShift(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} y_shift
     */
    set yShift(y_shift) {
        wasm.posewrapper_set_yShift(this.__wbg_ptr, y_shift);
    }
    /**
     * @returns {number}
     */
    get turnShoulder() {
        const ret = wasm.posewrapper_turnShoulder(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} turn_shoulder
     */
    set turnShoulder(turn_shoulder) {
        wasm.posewrapper_set_turnShoulder(this.__wbg_ptr, turn_shoulder);
    }
    /**
     * @returns {number}
     */
    get turnHip() {
        const ret = wasm.posewrapper_turnHip(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} turn_hip
     */
    set turnHip(turn_hip) {
        wasm.posewrapper_set_turnHip(this.__wbg_ptr, turn_hip);
    }
}

const RenderableSegmentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_renderablesegment_free(ptr >>> 0, 1));
/**
 * Projected line segment with two coordinates and a Z index.
 *
 * This format is perfect for 2D drawing.
 */
export class RenderableSegment {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RenderableSegment.prototype);
        obj.__wbg_ptr = ptr;
        RenderableSegmentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RenderableSegmentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_renderablesegment_free(ptr, 0);
    }
    /**
     * Start of the line segment in the xy plane.
     * @returns {Cartesian2d}
     */
    get start() {
        const ret = wasm.__wbg_get_renderablesegment_start(this.__wbg_ptr);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * Start of the line segment in the xy plane.
     * @param {Cartesian2d} arg0
     */
    set start(arg0) {
        _assertClass(arg0, Cartesian2d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_renderablesegment_start(this.__wbg_ptr, ptr0);
    }
    /**
     * End of the line segment in the xy plane.
     * @returns {Cartesian2d}
     */
    get end() {
        const ret = wasm.__wbg_get_renderablesegment_end(this.__wbg_ptr);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * End of the line segment in the xy plane.
     * @param {Cartesian2d} arg0
     */
    set end(arg0) {
        _assertClass(arg0, Cartesian2d);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_renderablesegment_end(this.__wbg_ptr, ptr0);
    }
    /**
     * Z-Index for draw order
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_renderablesegment_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * Z-Index for draw order
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_renderablesegment_z(this.__wbg_ptr, arg0);
    }
}

const SegmentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_segment_free(ptr >>> 0, 1));
/**
 * Projected line segment, with a x-y angle and a length factor.
 *
 * This format is usable for 2D drawing.
 */
export class Segment {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Segment.prototype);
        obj.__wbg_ptr = ptr;
        SegmentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SegmentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_segment_free(ptr, 0);
    }
    /**
     * The 2D projected angle of the segment, counter-clock wise to the x-axis,
     * in [0, 2*PI).
     * @returns {number}
     */
    get angle() {
        const ret = wasm.__wbg_get_cartesian2d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * The 2D projected angle of the segment, counter-clock wise to the x-axis,
     * in [0, 2*PI).
     * @param {number} arg0
     */
    set angle(arg0) {
        wasm.__wbg_set_cartesian2d_x(this.__wbg_ptr, arg0);
    }
    /**
     * The factor to multiply lengths when drawing the projected segment in 2D.
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_cartesian2d_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * The factor to multiply lengths when drawing the projected segment in 2D.
     * @param {number} arg0
     */
    set r(arg0) {
        wasm.__wbg_set_cartesian2d_y(this.__wbg_ptr, arg0);
    }
    /**
     * Z-Index for draw ordering
     * @returns {number}
     */
    get z() {
        const ret = wasm.__wbg_get_segment_z(this.__wbg_ptr);
        return ret;
    }
    /**
     * Z-Index for draw ordering
     * @param {number} arg0
     */
    set z(arg0) {
        wasm.__wbg_set_segment_z(this.__wbg_ptr, arg0);
    }
}

const SkeletonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeleton_free(ptr >>> 0, 1));
/**
 * A position- and size-independent description of a body pose snapshot for 2d
 * rendering. An intermediate step for [`RenderableSkeleton`].
 *
 * Each limb has a 2D angle in the x-y plane plus a length factor to simulate
 * the third dimension in a 2D projection. X grows to the right, y grows down.
 * Plus, there is a z-index for the order in which segments should be drawn.
 *
 * This format is for exporting to other modules. JS code can easily read it
 * and potentially render it.
 *
 * Note that the skeleton is stripped of position information, it only has
 * angles of all body parts. This means it cannot be used to overlay a video.
 * Use the original keypoints for such matters.
 *
 * TODO: I  don't think there is a good reason to expose internals of this. JS
 * should only worry about final coordinates, which it gets from the
 * RenderableSkeleton struct.
 */
export class Skeleton {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Skeleton.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeleton_free(ptr, 0);
    }
    /**
     * @returns {SkeletonSide}
     */
    get left() {
        const ret = wasm.__wbg_get_skeleton_left(this.__wbg_ptr);
        return SkeletonSide.__wrap(ret);
    }
    /**
     * @param {SkeletonSide} arg0
     */
    set left(arg0) {
        _assertClass(arg0, SkeletonSide);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeleton_left(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SkeletonSide}
     */
    get right() {
        const ret = wasm.__wbg_get_skeleton_right(this.__wbg_ptr);
        return SkeletonSide.__wrap(ret);
    }
    /**
     * @param {SkeletonSide} arg0
     */
    set right(arg0) {
        _assertClass(arg0, SkeletonSide);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeleton_right(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get hip() {
        const ret = wasm.__wbg_get_skeleton_hip(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set hip(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeleton_hip(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get shoulder() {
        const ret = wasm.__wbg_get_skeleton_shoulder(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set shoulder(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeleton_shoulder(this.__wbg_ptr, ptr0);
    }
    /**
     * Does the dancer look more to the side han they face the camera?
     * @returns {boolean}
     */
    get sideway() {
        const ret = wasm.__wbg_get_skeleton_sideway(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Does the dancer look more to the side han they face the camera?
     * @param {boolean} arg0
     */
    set sideway(arg0) {
        wasm.__wbg_set_skeleton_sideway(this.__wbg_ptr, arg0);
    }
    /**
     * Does the dancer face away more than they face the camera?
     * @returns {boolean}
     */
    get backwards() {
        const ret = wasm.__wbg_get_skeleton_backwards(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Does the dancer face away more than they face the camera?
     * @param {boolean} arg0
     */
    set backwards(arg0) {
        wasm.__wbg_set_skeleton_backwards(this.__wbg_ptr, arg0);
    }
    /**
     * @param {boolean} sideway
     * @returns {Skeleton}
     */
    static resting(sideway) {
        const ret = wasm.skeleton_resting(sideway);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {Skeleton}
     */
    restingPose() {
        const ret = wasm.skeleton_restingPose(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    debugString() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.skeleton_debugString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Compute 2d coordinates for the skeleton for rendering.
     *
     * The skeleton will be rendered assuming hard-coded values for body part
     * proportional lengths, multiplied with the size parameter. The hip
     * segment will have its center at the given position.
     * @param {Cartesian2d} hip_center
     * @param {number} size
     * @returns {SkeletonV2}
     */
    render(hip_center, size) {
        _assertClass(hip_center, Cartesian2d);
        var ptr0 = hip_center.__destroy_into_raw();
        const ret = wasm.skeleton_render(this.__wbg_ptr, ptr0, size);
        return SkeletonV2.__wrap(ret);
    }
}

const SkeletonSideFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeletonside_free(ptr >>> 0, 1));

export class SkeletonSide {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SkeletonSide.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonSideFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonSideFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeletonside_free(ptr, 0);
    }
    /**
     * @returns {Segment}
     */
    get thigh() {
        const ret = wasm.__wbg_get_skeletonside_thigh(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set thigh(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonside_thigh(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get shin() {
        const ret = wasm.__wbg_get_skeletonside_shin(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set shin(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonside_shin(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get arm() {
        const ret = wasm.__wbg_get_skeletonside_arm(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set arm(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonside_arm(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get forearm() {
        const ret = wasm.__wbg_get_skeletonside_forearm(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set forearm(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonside_forearm(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Segment}
     */
    get foot() {
        const ret = wasm.__wbg_get_skeletonside_foot(this.__wbg_ptr);
        return Segment.__wrap(ret);
    }
    /**
     * @param {Segment} arg0
     */
    set foot(arg0) {
        _assertClass(arg0, Segment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonside_foot(this.__wbg_ptr, ptr0);
    }
}

const SkeletonSideV2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeletonsidev2_free(ptr >>> 0, 1));

export class SkeletonSideV2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SkeletonSideV2.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonSideV2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonSideV2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeletonsidev2_free(ptr, 0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get thigh() {
        const ret = wasm.__wbg_get_skeletonsidev2_thigh(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set thigh(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonsidev2_thigh(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get shin() {
        const ret = wasm.__wbg_get_skeletonsidev2_shin(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set shin(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonsidev2_shin(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get arm() {
        const ret = wasm.__wbg_get_skeletonsidev2_arm(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set arm(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonsidev2_arm(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get forearm() {
        const ret = wasm.__wbg_get_skeletonsidev2_forearm(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set forearm(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonsidev2_forearm(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get foot() {
        const ret = wasm.__wbg_get_skeletonsidev2_foot(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set foot(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonsidev2_foot(this.__wbg_ptr, ptr0);
    }
}

const SkeletonV2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeletonv2_free(ptr >>> 0, 1));
/**
 * A self-sufficient description of a body position snapshot for 2.5d
 * rendering.
 *
 * In this format, x,y,z values have been computed to fit in a specific area,
 * assuming specific body part lengths. JS code can take it and directly draw
 * it on a Canvas or as an SVG. The z information is an integer describing draw
 * order conditions for the renderer to respect.
 */
export class SkeletonV2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SkeletonV2.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonV2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonV2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeletonv2_free(ptr, 0);
    }
    /**
     * @returns {SkeletonSideV2}
     */
    get left() {
        const ret = wasm.__wbg_get_skeletonv2_left(this.__wbg_ptr);
        return SkeletonSideV2.__wrap(ret);
    }
    /**
     * @param {SkeletonSideV2} arg0
     */
    set left(arg0) {
        _assertClass(arg0, SkeletonSideV2);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonv2_left(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {SkeletonSideV2}
     */
    get right() {
        const ret = wasm.__wbg_get_skeletonv2_right(this.__wbg_ptr);
        return SkeletonSideV2.__wrap(ret);
    }
    /**
     * @param {SkeletonSideV2} arg0
     */
    set right(arg0) {
        _assertClass(arg0, SkeletonSideV2);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonv2_right(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get hip() {
        const ret = wasm.__wbg_get_skeletonv2_hip(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set hip(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonv2_hip(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {RenderableSegment}
     */
    get shoulder() {
        const ret = wasm.__wbg_get_skeletonv2_shoulder(this.__wbg_ptr);
        return RenderableSegment.__wrap(ret);
    }
    /**
     * @param {RenderableSegment} arg0
     */
    set shoulder(arg0) {
        _assertClass(arg0, RenderableSegment);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletonv2_shoulder(this.__wbg_ptr, ptr0);
    }
    /**
     * Does the dancer look more to the side han they face the camera?
     * @returns {boolean}
     */
    get sideway() {
        const ret = wasm.__wbg_get_skeletonv2_sideway(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Does the dancer look more to the side han they face the camera?
     * @param {boolean} arg0
     */
    set sideway(arg0) {
        wasm.__wbg_set_skeletonv2_sideway(this.__wbg_ptr, arg0);
    }
    /**
     * Does the dancer face away more than they face the camera?
     * @returns {boolean}
     */
    get backwards() {
        const ret = wasm.__wbg_get_skeletonv2_backwards(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Does the dancer face away more than they face the camera?
     * @param {boolean} arg0
     */
    set backwards(arg0) {
        wasm.__wbg_set_skeletonv2_backwards(this.__wbg_ptr, arg0);
    }
    /**
     * @param {SkeletonLimb} field
     * @returns {RenderableSegment}
     */
    segment(field) {
        const ret = wasm.skeletonv2_segment(this.__wbg_ptr, field);
        return RenderableSegment.__wrap(ret);
    }
}

const SkeletonWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeletonwrapper_free(ptr >>> 0, 1));

export class SkeletonWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SkeletonWrapper.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeletonwrapper_free(ptr, 0);
    }
    /**
     * @returns {PoseWrapper}
     */
    pose() {
        const ret = wasm.skeletonwrapper_pose(this.__wbg_ptr);
        return PoseWrapper.__wrap(ret);
    }
    /**
     * @returns {Skeleton}
     */
    skeleton() {
        const ret = wasm.skeletonwrapper_set(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {Skeleton}
     */
    set() {
        const ret = wasm.skeletonwrapper_set(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
}

const SkeletonsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_skeletons_free(ptr >>> 0, 1));

export class Skeletons {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Skeletons.prototype);
        obj.__wbg_ptr = ptr;
        SkeletonsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SkeletonsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_skeletons_free(ptr, 0);
    }
    /**
     * @returns {Skeleton}
     */
    get front() {
        const ret = wasm.__wbg_get_skeletons_front(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @param {Skeleton} arg0
     */
    set front(arg0) {
        _assertClass(arg0, Skeleton);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletons_front(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Skeleton}
     */
    get side() {
        const ret = wasm.__wbg_get_skeletons_side(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @param {Skeleton} arg0
     */
    set side(arg0) {
        _assertClass(arg0, Skeleton);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_skeletons_side(this.__wbg_ptr, ptr0);
    }
}

const StepFileWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stepfilewrapper_free(ptr >>> 0, 1));

export class StepFileWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StepFileWrapper.prototype);
        obj.__wbg_ptr = ptr;
        StepFileWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StepFileWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stepfilewrapper_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.stepfilewrapper_new_empty();
        this.__wbg_ptr = ret >>> 0;
        StepFileWrapperFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} text
     * @returns {StepFileWrapper}
     */
    static fromRon(text) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(text, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.stepfilewrapper_fromRon(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StepFileWrapper.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {StepWrapper[]}
     */
    steps() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepfilewrapper_steps(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {StepWrapper} new_step
     */
    addStep(new_step) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(new_step, StepWrapper);
            wasm.stepfilewrapper_addStep(retptr, this.__wbg_ptr, new_step.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {StepWrapper} new_step
     */
    overwriteStep(new_step) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(new_step, StepWrapper);
            wasm.stepfilewrapper_overwriteStep(retptr, this.__wbg_ptr, new_step.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} id
     */
    removeStep(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.stepfilewrapper_removeStep(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepfilewrapper_buildRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @returns {string}
     */
    buildPrettyRon() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepfilewrapper_buildPrettyRon(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const StepPositionBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_steppositionbuilder_free(ptr >>> 0, 1));
/**
 * Used in the editor to add and edit poses of a step.
 */
export class StepPositionBuilder {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StepPositionBuilder.prototype);
        obj.__wbg_ptr = ptr;
        StepPositionBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StepPositionBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_steppositionbuilder_free(ptr, 0);
    }
    /**
     * @param {PoseWrapper} pose
     */
    constructor(pose) {
        _assertClass(pose, PoseWrapper);
        const ret = wasm.steppositionbuilder_new(pose.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        StepPositionBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {PoseWrapper}
     */
    pose() {
        const ret = wasm.steppositionbuilder_pose(this.__wbg_ptr);
        return PoseWrapper.__wrap(ret);
    }
    /**
     * @returns {number | undefined}
     */
    get jumpHeight() {
        const ret = wasm.steppositionbuilder_jumpHeight(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {number} height
     */
    setJumpHeight(height) {
        wasm.steppositionbuilder_setJumpHeight(this.__wbg_ptr, height);
    }
    /**
     * @returns {Orientation}
     */
    get orientation() {
        const ret = wasm.steppositionbuilder_orientation(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Orientation} orientation
     */
    setOrientation(orientation) {
        wasm.steppositionbuilder_setOrientation(this.__wbg_ptr, orientation);
    }
}

const StepWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stepwrapper_free(ptr >>> 0, 1));

export class StepWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StepWrapper.prototype);
        obj.__wbg_ptr = ptr;
        StepWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof StepWrapper)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StepWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stepwrapper_free(ptr, 0);
    }
    /**
     * @param {string} id
     * @param {string} name
     * @param {string} source
     */
    constructor(id, name, source) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(source, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.stepwrapper_new_empty(ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret >>> 0;
        StepWrapperFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * The unique identifier for the step.
     * @returns {string}
     */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * The descriptive name for the step. The same name is used for variations
     * of the same step.
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {string} name
     */
    set name(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        wasm.stepwrapper_set_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} beat
     * @returns {Skeleton}
     */
    skeleton(beat) {
        const ret = wasm.stepwrapper_skeleton(this.__wbg_ptr, beat);
        return Skeleton.__wrap(ret);
    }
    /**
     * How much the body position deviates from the origin.
     * @param {number} beat
     * @returns {Cartesian2d}
     */
    bodyShift(beat) {
        const ret = wasm.stepwrapper_bodyShift(this.__wbg_ptr, beat);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * Applies a rotation (in degree) and returns the resulting skelton.
     * @param {number} beat
     * @param {number} rotation
     * @returns {Skeleton}
     */
    rotatedSkeleton(beat, rotation) {
        const ret = wasm.stepwrapper_rotatedSkeleton(this.__wbg_ptr, beat, rotation);
        return Skeleton.__wrap(ret);
    }
    /**
     * @param {number} beat
     * @returns {number | undefined}
     */
    jumpHeight(beat) {
        const ret = wasm.stepwrapper_jumpHeight(this.__wbg_ptr, beat);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * Description identifier for the translated text which describes how the
     * variation is different from the original.
     *
     * For example: "left-first" can be used for all steps which are the same
     * as the original but instead of starting with the right foot, it starts
     * with the left foot first. The app shows a translated text like "Left Leg First".
     * @returns {string}
     */
    get variation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_variation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * The number of subbeats the step takes for one repetition.
     * @returns {number}
     */
    get subbeats() {
        const ret = wasm.stepwrapper_subbeats(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Look up poses from the global collection, do not use for courses that
     * require a custom collection.
     * @returns {PoseWrapper[]}
     */
    poses() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_poses(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Positions with poses from the global collection, do not use for courses
     * that require a custom collection.
     * @returns {StepPositionBuilder[]}
     */
    positions() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_positions(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Add poses from the global collection, do not use for courses that
     * require a custom collection.
     * @param {StepPositionBuilder} position
     */
    addPosition(position) {
        _assertClass(position, StepPositionBuilder);
        wasm.stepwrapper_addPosition(this.__wbg_ptr, position.__wbg_ptr);
    }
    /**
     * @param {number} index
     */
    removePosition(index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stepwrapper_removePosition(retptr, this.__wbg_ptr, index);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} index
     * @param {StepPositionBuilder} position
     */
    insertPosition(index, position) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(position, StepPositionBuilder);
            wasm.stepwrapper_insertPosition(retptr, this.__wbg_ptr, index, position.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const TextEffectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_texteffect_free(ptr >>> 0, 1));

export class TextEffect {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(TextEffect.prototype);
        obj.__wbg_ptr = ptr;
        TextEffectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TextEffectFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_texteffect_free(ptr, 0);
    }
    /**
     * When the text should be displayed, could be in the future.
     * @returns {number}
     */
    get timestamp() {
        const ret = wasm.__wbg_get_audioeffect_timestamp(this.__wbg_ptr);
        return ret;
    }
    /**
     * When the text should be displayed, could be in the future.
     * @param {number} arg0
     */
    set timestamp(arg0) {
        wasm.__wbg_set_audioeffect_timestamp(this.__wbg_ptr, arg0);
    }
    /**
     * How long to show the text, in ms
     * @returns {number}
     */
    get duration() {
        const ret = wasm.__wbg_get_texteffect_duration(this.__wbg_ptr);
        return ret;
    }
    /**
     * How long to show the text, in ms
     * @param {number} arg0
     */
    set duration(arg0) {
        wasm.__wbg_set_texteffect_duration(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get text() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.texteffect_text(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const TrackerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tracker_free(ptr >>> 0, 1));
/**
 * A Tracker gathers skeletons over time and passes it on to a DanceDetector.
 */
export class Tracker {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Tracker.prototype);
        obj.__wbg_ptr = ptr;
        TrackerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TrackerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tracker_free(ptr, 0);
    }
    /**
     * Create a tracker for all known steps.
     */
    constructor() {
        const ret = wasm.tracker_new_from_global_collection();
        this.__wbg_ptr = ret >>> 0;
        TrackerFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Track one specific step, by name, including its variations (with the same name).
     * @param {string} step_name
     * @returns {Tracker}
     */
    static StepTracker(step_name) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(step_name, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.tracker_StepTracker(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Tracker.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Track one specific step, by ID, excluding its variations (with the same name).
     *
     * This is not intended for general dance detection but rather for a
     * specific training session without much regard for timing etc.
     * @param {string} step_id
     * @returns {Tracker}
     */
    static UniqueStepTracker(step_id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(step_id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
            const len0 = WASM_VECTOR_LEN;
            wasm.tracker_UniqueStepTracker(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Tracker.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Mix a warmup with the given steps, by name.
     *
     * @param {string[]} step_names
     * @param {number} num_beats
     * @returns {Tracker}
     */
    static WarmUp(step_names, num_beats) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayJsValueToWasm0(step_names, wasm.__wbindgen_export_2);
            const len0 = WASM_VECTOR_LEN;
            wasm.tracker_WarmUp(retptr, ptr0, len0, num_beats);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Tracker.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    finishTracking() {
        wasm.tracker_finishTracking(this.__wbg_ptr);
    }
    clear() {
        wasm.tracker_clear(this.__wbg_ptr);
    }
    /**
     * Insert keypoints of a new frame for tracking.
     *
     * This is the main method to insert data into the tracker.
     * @param {Keypoints} keypoints
     * @param {number} timestamp
     * @returns {Skeletons}
     */
    addKeypoints(keypoints, timestamp) {
        _assertClass(keypoints, Keypoints);
        var ptr0 = keypoints.__destroy_into_raw();
        const ret = wasm.tracker_addKeypoints(this.__wbg_ptr, ptr0, timestamp);
        return Skeletons.__wrap(ret);
    }
    /**
     * @param {number} bpm
     */
    setBpm(bpm) {
        wasm.tracker_setBpm(this.__wbg_ptr, bpm);
    }
    /**
     * @param {number} first_beat
     */
    alignBeat(first_beat) {
        wasm.tracker_alignBeat(this.__wbg_ptr, first_beat);
    }
    /**
     * @param {boolean} yes
     */
    enforceBeat(yes) {
        wasm.tracker_enforceBeat(this.__wbg_ptr, yes);
    }
    /**
     * @param {number} error_threshold
     */
    setErrorThreshold(error_threshold) {
        wasm.tracker_setErrorThreshold(this.__wbg_ptr, error_threshold);
    }
    /**
     * Goes over all data and detects the best fitting dance.
     *
     * There is no re-use or consistency between calls. It always starts at 0
     * and computes the global best fit.
     *
     * Use [`Tracker::run_detection`] for incremental detection.
     * @returns {DetectionResult}
     */
    detectDance() {
        const ret = wasm.tracker_detectDance(this.__wbg_ptr);
        return DetectionResult.__wrap(ret);
    }
    /**
     * @returns {DetectionResult}
     */
    runDetection() {
        const ret = wasm.tracker_runDetection(this.__wbg_ptr);
        return DetectionResult.__wrap(ret);
    }
    /**
     * @returns {PoseHint}
     */
    poseHint() {
        const ret = wasm.tracker_poseHint(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {PoseApproximation | undefined}
     */
    currentPoseError() {
        const ret = wasm.tracker_currentPoseError(this.__wbg_ptr);
        return ret === 0 ? undefined : PoseApproximation.__wrap(ret);
    }
    /**
     * @param {number} t
     * @returns {TeacherView}
     */
    currentView(t) {
        const ret = wasm.tracker_currentView(this.__wbg_ptr, t);
        return ret;
    }
    /**
     * @returns {ReadableDetectionState}
     */
    get detectionState() {
        const ret = wasm.tracker_detectionState(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
     * @returns {number}
     */
    get trackedSubbeats() {
        const ret = wasm.tracker_trackedSubbeats(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number | null} [now]
     * @returns {number}
     */
    nextSubbeat(now) {
        const ret = wasm.tracker_nextSubbeat(this.__wbg_ptr, !isLikeNone(now), isLikeNone(now) ? 0 : now);
        return ret;
    }
    /**
     * @returns {number}
     */
    get timeBetweenPoses() {
        const ret = wasm.tracker_timeBetweenPoses(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {AudioEffect | undefined}
     */
    nextAudioEffect() {
        const ret = wasm.tracker_nextAudioEffect(this.__wbg_ptr);
        return ret === 0 ? undefined : AudioEffect.__wrap(ret);
    }
    /**
     * @param {number} after
     * @returns {TextEffect | undefined}
     */
    nextTextEffect(after) {
        const ret = wasm.tracker_nextTextEffect(this.__wbg_ptr, after);
        return ret === 0 ? undefined : TextEffect.__wrap(ret);
    }
    /**
     * Return a skeleton for a pose.
     * @param {string} id
     * @returns {Skeleton | undefined}
     */
    poseSkeleton(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.tracker_poseSkeleton(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : Skeleton.__wrap(ret);
    }
    /**
     * Return a skeleton that's expected now.
     *
     * Only implemented to work properly for trackers of unique steps.
     *
     * (experimenting with live instructor, I probably want to change this when cleaning up the impl)
     * @returns {Skeleton}
     */
    expectedPoseSkeleton() {
        const ret = wasm.tracker_expectedPoseSkeleton(this.__wbg_ptr);
        return Skeleton.__wrap(ret);
    }
    /**
     * @param {number} t
     * @returns {number}
     */
    subbeat(t) {
        const ret = wasm.tracker_subbeat(this.__wbg_ptr, t);
        return ret;
    }
    /**
     * @param {number} subbeat
     * @returns {Skeleton}
     */
    poseSkeletonAtSubbeat(subbeat) {
        const ret = wasm.tracker_poseSkeletonAtSubbeat(this.__wbg_ptr, subbeat);
        return Skeleton.__wrap(ret);
    }
    /**
     * @returns {Cartesian2d}
     */
    expectedPoseBodyShift() {
        const ret = wasm.tracker_expectedPoseBodyShift(this.__wbg_ptr);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * @param {number} beat
     * @returns {Cartesian2d}
     */
    poseBodyShiftAtSubbeat(beat) {
        const ret = wasm.tracker_poseBodyShiftAtSubbeat(this.__wbg_ptr, beat);
        return Cartesian2d.__wrap(ret);
    }
    /**
     * @returns {DetectionResult}
     */
    get lastDetection() {
        const ret = wasm.tracker_lastDetection(this.__wbg_ptr);
        return DetectionResult.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    numDetectedPoses() {
        const ret = wasm.tracker_numDetectedPoses(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} timestamp
     * @returns {Cartesian3d}
     */
    hipPosition(timestamp) {
        const ret = wasm.tracker_hipPosition(this.__wbg_ptr, timestamp);
        return Cartesian3d.__wrap(ret);
    }
    /**
     * Fit frames in a time interval against all poses and return the best fit.
     *
     * This API is exported mostly for debugging. To extract fitted dances, use
     * `detect_dance` instead.
     * @param {number} start
     * @param {number} end
     * @returns {PoseApproximation | undefined}
     */
    bestFitPose(start, end) {
        const ret = wasm.tracker_bestFitPose(this.__wbg_ptr, start, end);
        return ret === 0 ? undefined : PoseApproximation.__wrap(ret);
    }
    /**
     * Fit a single frame against all poses and return all errors
     * @param {number} timestamp
     * @returns {PoseApproximation[]}
     */
    allPoseErrors(timestamp) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.tracker_allPoseErrors(retptr, this.__wbg_ptr, timestamp);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_1(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * @param {number} timestamp
     * @returns {Skeleton | undefined}
     */
    skeletonAt(timestamp) {
        const ret = wasm.tracker_skeletonAt(this.__wbg_ptr, timestamp);
        return ret === 0 ? undefined : Skeleton.__wrap(ret);
    }
    /**
     * @param {number} timestamp
     * @returns {SkeletonWrapper | undefined}
     */
    skeletonWrapperAt(timestamp) {
        const ret = wasm.tracker_skeletonWrapperAt(this.__wbg_ptr, timestamp);
        return ret === 0 ? undefined : SkeletonWrapper.__wrap(ret);
    }
    /**
     * The original keypoints rendered as skeleton, at the given time frame.
     * @param {number} timestamp
     * @param {number} width
     * @param {number} height
     * @returns {SkeletonV2 | undefined}
     */
    renderedKeypointsAt(timestamp, width, height) {
        const ret = wasm.tracker_renderedKeypointsAt(this.__wbg_ptr, timestamp, width, height);
        return ret === 0 ? undefined : SkeletonV2.__wrap(ret);
    }
    /**
     * @param {DetectionState} state
     * @param {number} timestamp
     */
    devSetState(state, timestamp) {
        wasm.tracker_devSetState(this.__wbg_ptr, state, timestamp);
    }
    /**
     * @param {number} timestamp
     * @returns {ExportedFrame}
     */
    exportFrame(timestamp) {
        const ret = wasm.tracker_exportFrame(this.__wbg_ptr, timestamp);
        return ExportedFrame.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    exportKeypoints() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.tracker_exportKeypoints(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ZErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_zerror_free(ptr >>> 0, 1));

export class ZError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ZError.prototype);
        obj.__wbg_ptr = ptr;
        ZErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ZErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_zerror_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get error() {
        const ret = wasm.__wbg_get_cartesian2d_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set error(arg0) {
        wasm.__wbg_set_cartesian2d_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get quadrant_error() {
        const ret = wasm.__wbg_get_zerror_quadrant_error(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set quadrant_error(arg0) {
        wasm.__wbg_set_zerror_quadrant_error(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    get name() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.zerror_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

const ZWrongOrderErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_zwrongordererror_free(ptr >>> 0, 1));

export class ZWrongOrderError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ZWrongOrderError.prototype);
        obj.__wbg_ptr = ptr;
        ZWrongOrderErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ZWrongOrderErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_zwrongordererror_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get expected() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.zwrongordererror_expected(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getCachedStringFromWasm0(r0, r1);
            if (r0 !== 0) { wasm.__wbindgen_export_1(r0, r1, 1); }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_call_75b89300dd530ca6 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_d68488931693e6ee = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_dancewrapper_new = function(arg0) {
        const ret = DanceWrapper.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_derived_45e38bcbdc084248 = function(arg0, arg1) {
        const ret = derived(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_detectedstep_new = function(arg0) {
        const ret = DetectedStep.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        if (arg0 !== 0) { wasm.__wbindgen_export_1(arg0, arg1, 1); }
        console.error(v0);
    };
    imports.wbg.__wbg_fetch_ce78c45327c6d552 = function(arg0, arg1) {
        const ret = getObject(arg0).fetch(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_globalThis_59c7794d9413986f = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_04c81bad83a72129 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Response_d1b3f08d4983dc43 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Response;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_47f723ed0409d724 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_lesson_new = function(arg0) {
        const ret = Lesson.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_lessonpart_new = function(arg0) {
        const ret = LessonPart.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_limberror_new = function(arg0) {
        const ret = LimbError.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_log_80a50dc3901559aa = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_new_51fd3aeaa6d3d03f = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_398(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_fe7e106c48aadd7e = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        const ret = new Function(v0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithstr_6fe489b9fca490e5 = function() { return handleError(function (arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        const ret = new Request(v0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_poseapproximation_new = function(arg0) {
        const ret = PoseApproximation.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_posewrapper_new = function(arg0) {
        const ret = PoseWrapper.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_posewrapper_unwrap = function(arg0) {
        const ret = PoseWrapper.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_5a8a9131f3f0b37b = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_queueMicrotask_6d79674585219521 = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_resolve_33aaa312c39e688c = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_c9a63b952bd22cbd = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_ae981da1abf3e248 = function(arg0, arg1) {
        getObject(arg0).set(takeObject(arg1));
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_steppositionbuilder_new = function(arg0) {
        const ret = StepPositionBuilder.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stepwrapper_new = function(arg0) {
        const ret = StepWrapper.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stepwrapper_unwrap = function(arg0) {
        const ret = StepWrapper.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_text_9847c497903c7989 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).text();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_2f5c60c96e823bd2 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_acd4f2d41ed1cf58 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_window_81304a10d2638125 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_writable_344683951acf29e2 = function(arg0) {
        const ret = writable(takeObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_zerror_new = function(arg0) {
        const ret = ZError.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_zwrongordererror_new = function(arg0) {
        const ret = ZWrongOrderError.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_closure_wrapper1466 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 154, __wbg_adapter_25);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper754 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 83, __wbg_adapter_22);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_2, wasm.__wbindgen_export_3);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint16ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;



    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('bouncy_instructor_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
