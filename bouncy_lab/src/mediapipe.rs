use crate::{
    ffmpeg::VideoInput,
    generated::pose_bindings::{
        pose_landmarker_create, pose_landmarker_detect_for_video, BaseOptions,
        ImageProcessingOptions, MPRectF, MpImageCreateFromUint8Data,
        MpImageFormat_kMpImageFormatSrgb, MpImageFree, MpImagePtr, MpStatus_kMpOk,
        PoseLandmarkerOptions, PoseLandmarkerResult, RunningMode_VIDEO,
    },
};
use ::std::os::raw::c_char;
use anyhow::Context;
use ffmpeg_next::{self as ffmpeg};
use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

pub fn detect_pose(mut video: VideoInput) -> anyhow::Result<()> {
    let path = std::fs::read_link("./models/pose_landmarker_heavy.task")
        .context("failed reading model task symlink")?;
    let model_asset = std::fs::read(&path).context("failed reading model task file")?;
    let model_asset_path = CString::new(path.to_string_lossy().to_string())?;

    // The model uses 256 x 256 pixels with RGB24.
    // See https://ai.google.dev/edge/mediapipe/solutions/vision/pose_landmarker.
    // We use ffmpeg to scale it accordingly, potentially squeezing the image
    // out of its original proportion.

    let mut scaler = ffmpeg::software::scaling::context::Context::get(
        video.decoder.format(),
        video.decoder.width(),
        video.decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        256,
        256,
        ffmpeg::software::scaling::flag::Flags::FAST_BILINEAR,
    )?;

    let mut opt = PoseLandmarkerOptions {
        base_options: BaseOptions {
            model_asset_buffer: model_asset.as_ptr() as *const _, // this can be a nullpointer if `count` is set to zero
            model_asset_buffer_count: model_asset.len() as u32,
            model_asset_path: model_asset_path.as_ptr(),
        },
        running_mode: RunningMode_VIDEO,
        // max detectable poses, set to 1 for now, experiment with increasing later
        num_poses: 1,
        min_pose_detection_confidence: 0.4,
        min_pose_presence_confidence: 0.4,
        min_tracking_confidence: 0.4,
        output_segmentation_masks: false,
        // only used for livestream option
        result_callback: None,
    };

    let landmarker;
    unsafe {
        let mut error_msg = FFIResult::new();
        landmarker = pose_landmarker_create(&mut opt, error_msg.as_mut());
        error_msg.check()?;
        assert!(!landmarker.is_null());
    }

    for (stream, packet) in video.ictx.packets() {
        if stream.index() == video.video_stream_index {
            video.decoder.send_packet(&packet)?;
            let mut frame = ffmpeg::util::frame::Video::empty();
            if video.decoder.receive_frame(&mut frame).is_ok() {
                let pts = frame.pts().unwrap_or(0);
                let time_base = stream.time_base();
                let timestamp_ms = (pts as f64 * f64::from(time_base) * 1000.0).round() as i64;

                let mut rgb_frame = ffmpeg::util::frame::Video::empty();
                scaler.run(&frame, &mut rgb_frame)?;

                let width = rgb_frame.width() as i32;
                let height = rgb_frame.height() as i32;
                let data = rgb_frame.data(0);
                let data_size = rgb_frame.stride(0) * height as usize;

                // Create MpImage
                let mut image: MpImagePtr = std::ptr::null_mut();
                let status = unsafe {
                    MpImageCreateFromUint8Data(
                        MpImageFormat_kMpImageFormatSrgb,
                        width,
                        height,
                        data.as_ptr(),
                        data_size as i32,
                        &mut image,
                    )
                };

                if status != MpStatus_kMpOk {
                    panic!("Failed to create MpImage, error code {}", status);
                }

                unsafe {
                    let region_of_interest = MPRectF {
                        left: 0.0,
                        top: 0.0,
                        bottom: 1.0,
                        right: 1.0,
                    };
                    let options = ImageProcessingOptions {
                        // The optional region-of-interest to crop from the image.
                        // If has_region_of_interest is 0, the full image is used.
                        // Coordinates must be in [0,1] with 'left' < 'right' and 'top' < 'bottom'.
                        has_region_of_interest: 0,
                        region_of_interest,
                        // The rotation to apply to the image (or cropped region-of-interest),
                        // in degrees clockwise. The rotation must be a multiple (positive or
                        // negative) of 90°.
                        rotation_degrees: 0,
                    };

                    // optional paramter may be null?
                    // let options: *const ImageProcessingOptions = null();

                    let mut result = PoseLandmarkerResult {
                        segmentation_masks: null_mut(),
                        segmentation_masks_count: 0,
                        pose_landmarks: null_mut(),
                        pose_landmarks_count: 0,
                        pose_world_landmarks: null_mut(),
                        pose_world_landmarks_count: 0,
                    };

                    let mut error_msg = FFIResult::new();
                    let _err_code = pose_landmarker_detect_for_video(
                        landmarker,
                        image,
                        &options,
                        timestamp_ms,
                        &mut result,
                        error_msg.as_mut(),
                    );
                    error_msg.check()?;

                    let landmarks = *result.pose_landmarks;
                    println!("Detected {} landmarks", landmarks.landmarks_count);

                    MpImageFree(image);
                }
            }
        }
    }

    drop(model_asset_path);

    Ok(())
}

struct FFIResult {
    ptr: *mut c_char,
}

impl FFIResult {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn check(self) -> anyhow::Result<()> {
        if !self.ptr.is_null() {
            let msg = unsafe { CStr::from_ptr(self.ptr) }
                .to_string_lossy()
                .to_string();
            anyhow::bail!("{msg}")
        } else {
            Ok(())
        }
    }

    fn as_mut(&mut self) -> *mut *mut i8 {
        &mut self.ptr
    }
}

impl Drop for FFIResult {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                libc::free(self.ptr as *mut libc::c_void);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}
