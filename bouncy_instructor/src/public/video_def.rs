use wasm_bindgen::prelude::wasm_bindgen;

use crate::parsing;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct VideoDef {
    variant: VideoDefEnum,
}

#[derive(Debug, Clone)]
enum VideoDefEnum {
    /// Defined with additional meta data
    Full {
        path: String,
        beats: Vec<f64>,
        start_markers: Vec<f64>,
        step_markers: Vec<StepMarker>,
    },
    /// Defined by just the path
    Simple(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Marker {
    Start,
    Step(String),
}

#[derive(Debug, Clone, PartialEq)]
#[wasm_bindgen]
pub struct StepMarker {
    pub timestamp: f64,
    step: String,
}

#[wasm_bindgen]
impl VideoDef {
    pub fn path(&self) -> String {
        self.borrow_path().to_owned()
    }

    #[wasm_bindgen(js_name = "isEmpty")]
    pub fn is_empty(&self) -> bool {
        self.borrow_path().is_empty()
    }

    pub fn beats(&self) -> Vec<f64> {
        self.borrow_beats().to_vec()
    }

    #[wasm_bindgen(js_name = "startMarkers")]
    pub fn start_markers(&self) -> Vec<f64> {
        self.borrow_start_markers().to_vec()
    }

    #[wasm_bindgen(js_name = "stepMarkers")]
    pub fn step_markers(&self) -> Vec<StepMarker> {
        self.borrow_step_markers().to_vec()
    }
}

impl VideoDef {
    fn borrow_path(&self) -> &str {
        match &self.variant {
            VideoDefEnum::Full { path, .. } => path,
            VideoDefEnum::Simple(path) => path,
        }
    }

    fn borrow_beats(&self) -> &[f64] {
        match &self.variant {
            VideoDefEnum::Full { beats, .. } => beats,
            VideoDefEnum::Simple(_) => &[],
        }
    }

    fn borrow_start_markers(&self) -> &[f64] {
        match &self.variant {
            VideoDefEnum::Full { start_markers, .. } => start_markers,
            VideoDefEnum::Simple(_) => &[],
        }
    }

    fn borrow_step_markers(&self) -> &[StepMarker] {
        match &self.variant {
            VideoDefEnum::Full { step_markers, .. } => step_markers,
            VideoDefEnum::Simple(_) => &[],
        }
    }
}

impl From<parsing::video_def::VideoDef> for VideoDef {
    fn from(other: parsing::video_def::VideoDef) -> Self {
        let variant = match other {
            parsing::video_def::VideoDef::Full {
                path,
                beats,
                markers,
            } => {
                let start_markers = markers
                    .iter()
                    .filter_map(|(timestamp, marker)| match marker {
                        parsing::video_def::Marker::Start => Some(*timestamp as f64),
                        parsing::video_def::Marker::Step(_) => None,
                    })
                    .collect();
                let step_markers = markers
                    .iter()
                    .filter_map(|(timestamp, marker)| match marker {
                        parsing::video_def::Marker::Start => None,
                        parsing::video_def::Marker::Step(id) => Some(StepMarker {
                            timestamp: *timestamp as f64,
                            step: id.clone(),
                        }),
                    })
                    .collect();

                VideoDefEnum::Full {
                    path,
                    beats: beats.iter().map(|t| *t as f64).collect(),
                    start_markers,
                    step_markers,
                }
            }
            parsing::video_def::VideoDef::Simple(path) => VideoDefEnum::Simple(path),
        };
        Self { variant }
    }
}

impl From<parsing::video_def::Marker> for Marker {
    fn from(other: parsing::video_def::Marker) -> Self {
        match other {
            parsing::video_def::Marker::Start => Marker::Start,
            parsing::video_def::Marker::Step(path) => Marker::Step(path),
        }
    }
}
