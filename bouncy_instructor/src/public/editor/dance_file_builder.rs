use crate::dance_file::DanceFile;
use crate::editor::ExportError;
use crate::{dance_file, intern, DanceInfo};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::dance_builder::DanceBuilder;

#[wasm_bindgen]

pub struct DanceFileBuilder {
    version: u16,
    dances: Vec<intern::dance::Dance>,
}

#[wasm_bindgen]
impl DanceFileBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            version: dance_file::CURRENT_VERSION,
            dances: vec![],
        }
    }

    #[wasm_bindgen(js_name = "fromRon")]
    pub fn from_ron(text: &str) -> Result<DanceFileBuilder, JsValue> {
        let file = DanceFile::from_str(text)?;
        Ok(file.into())
    }

    #[wasm_bindgen(js_name = "addDance")]
    pub fn add_dance(&mut self, dance_builder: &DanceBuilder) -> Result<(), String> {
        if self.dances.iter().any(|dance| dance.id == dance_builder.id) {
            return Err("Dance ID already exists".to_owned());
        }
        self.dances.push(dance_builder.build());
        Ok(())
    }

    #[wasm_bindgen(js_name = "removeDance")]
    pub fn remove_dance(&mut self, id: String) -> Result<(), String> {
        if let Some(index) = self.dances.iter().position(|dance| dance.id == id) {
            self.dances.remove(index);
            Ok(())
        } else {
            Err("Dance ID does not exists".to_owned())
        }
    }

    #[wasm_bindgen(js_name = "buildRon")]
    pub fn build_ron(&self) -> Result<String, ExportError> {
        let file_data = DanceFile {
            version: self.version,
            dances: self
                .dances
                .iter()
                .map(|dance| dance_file::Dance {
                    id: dance.id.clone(),
                    steps: dance.step_ids.clone(),
                })
                .collect(),
        };
        let string = ron::ser::to_string(&file_data)?;
        Ok(string)
    }

    pub fn dances(&self) -> Vec<DanceInfo> {
        self.dances.iter().map(DanceInfo::from).collect()
    }
}

impl From<DanceFile> for DanceFileBuilder {
    fn from(other: DanceFile) -> Self {
        Self {
            version: other.version,
            dances: other
                .dances
                .into_iter()
                .map(|d| intern::dance::Dance {
                    id: d.id,
                    step_ids: d.steps,
                })
                .collect(),
        }
    }
}
