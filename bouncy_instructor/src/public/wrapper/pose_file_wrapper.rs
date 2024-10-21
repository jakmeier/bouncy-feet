use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::editor::ExportError;
use crate::pose_file::PoseFile;
use crate::wrapper::pose_wrapper::PoseWrapper;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PoseFileWrapper {
    pose_file: Rc<RefCell<PoseFile>>,
    // cache must always be kept in sync
    poses_cache: Rc<RefCell<Vec<PoseWrapper>>>,
}

#[wasm_bindgen]
impl PoseFileWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new_empty() -> Self {
        Self::new(PoseFile::new())
    }

    #[wasm_bindgen(js_name = "fromRon")]
    pub fn from_ron(text: &str) -> Result<PoseFileWrapper, JsValue> {
        let file = PoseFile::from_str(text)?;

        Ok(Self::new(file))
    }

    pub fn poses(&self) -> Vec<PoseWrapper> {
        self.poses_cache.as_ref().borrow().clone()
    }

    #[wasm_bindgen(js_name = "addPose")]
    pub fn add_pose(&mut self, new_pose: &PoseWrapper) -> Result<(), String> {
        let file = self.pose_file.as_ref().borrow();
        if file
            .poses
            .iter()
            .any(|pose| new_pose.definition().id == pose.id)
        {
            return Err("Pose ID already exists".to_owned());
        }
        drop(file);
        self.push_pose_internal(new_pose.clone());
        Ok(())
    }

    #[wasm_bindgen(js_name = "overwritePose")]
    pub fn overwrite_pose(&mut self, new_pose: &PoseWrapper) -> Result<(), String> {
        let file = self.pose_file.as_ref().borrow();

        if let Some(index) = file
            .poses
            .iter()
            .position(|pose| pose.id == new_pose.definition().id)
        {
            drop(file);
            self.set_pose(index, new_pose.clone());
            Ok(())
        } else {
            Err("Pose ID does not exists".to_owned())
        }
    }

    #[wasm_bindgen(js_name = "removePose")]
    pub fn remove_pose(&mut self, id: String) -> Result<(), String> {
        let file = self.pose_file.as_ref().borrow();
        if let Some(index) = file.poses.iter().position(|pose| pose.id == id) {
            drop(file);
            self.remove_pose_internal(index);
            Ok(())
        } else {
            Err("Pose ID does not exists".to_owned())
        }
    }

    #[wasm_bindgen(js_name = "buildRon")]
    pub fn build_ron(&self) -> Result<String, ExportError> {
        let file_data = self.pose_file.as_ref().borrow();
        let string = ron::ser::to_string(&*file_data)?;
        Ok(string)
    }

    #[wasm_bindgen(js_name = "buildPrettyRon")]
    pub fn build_pretty_ron(&self) -> Result<String, ExportError> {
        let file_data = self.pose_file.as_ref().borrow();
        let string = ron::ser::to_string_pretty(&*file_data, ron::ser::PrettyConfig::default())?;
        Ok(string)
    }
}

impl PoseFileWrapper {
    fn new(file: PoseFile) -> Self {
        let poses = file.poses.iter().cloned().map(PoseWrapper::new).collect();
        Self {
            pose_file: Rc::new(RefCell::new(file)),
            poses_cache: Rc::new(RefCell::new(poses)),
        }
    }

    fn push_pose_internal(&self, pose: PoseWrapper) {
        self.pose_file
            .borrow_mut()
            .poses
            .push(pose.definition().clone());
        self.poses_cache.borrow_mut().push(pose);
    }

    fn remove_pose_internal(&self, index: usize) {
        self.poses_cache.borrow_mut().remove(index);
        self.pose_file.borrow_mut().poses.remove(index);
    }

    fn set_pose(&self, index: usize, pose: PoseWrapper) {
        self.pose_file.borrow_mut().poses[index] = pose.definition().clone();
        self.poses_cache.borrow_mut()[index] = pose;
    }
}
