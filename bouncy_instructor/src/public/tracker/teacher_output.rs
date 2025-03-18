use wasm_bindgen::prelude::wasm_bindgen;

/// Hint to the UI, which information should be shown to the user during the
/// current section.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TeacherView {
    /// Show nothing
    Off = 0,
    /// Show the instructor, no need to show the user camera.
    InstructorOnly = 1,
    /// The user camera should be shown with a tracked avatar.
    UserCameraWithTracking = 2,
    /// Show the camera (without tracking) and the instructor.
    InstructorAndCamera = 3,
    /// Show just the plain camera.
    CameraOnly = 4,
}

/// A pointer to a pose inside a tracked activity or dance.
///
/// Can be used to reference a pose independently of pace.
/// Useful also for checking if a pose changed.
#[wasm_bindgen]
pub struct DanceCursor {
    pub(crate) subbeat: u32,
    pub(crate) step_index: usize,
    pub(crate) pose_index: usize,
}

#[wasm_bindgen]
impl DanceCursor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            subbeat: 0,
            step_index: 0,
            pose_index: 0,
        }
    }

    /// Whether both cursors show to the same pose slot.
    ///
    /// Note: If the same pose is repeated and two cursors show to two different
    /// duplicated, this function will count them as two different poses.
    #[wasm_bindgen(js_name = "isSamePose")]
    pub fn is_same_pose(&self, other: &DanceCursor) -> bool {
        self.step_index == other.step_index && self.pose_index == other.pose_index
    }

    #[wasm_bindgen(js_name = "isSameSubbeat")]
    pub fn is_same_subbeat(&self, other: &DanceCursor) -> bool {
        self.subbeat == other.subbeat
    }
}
