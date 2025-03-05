use wasm_bindgen::prelude::wasm_bindgen;

/// Hint to the UI, which information should be shown to the user during the
/// current section.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TeacherView {
    /// Show the instructor, no need to show the user camera.
    InstructorOnly = 1,
    /// The user camera should be shown with a tracked avatar.
    UserCameraWithTracking = 2,
    /// Show nothing
    Off = 3,
}
