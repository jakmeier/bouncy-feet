use crate::dance_file::{DanceFile, DanceStep};
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

    #[wasm_bindgen(js_name = "overwriteDance")]
    pub fn overwrite_dance(&mut self, dance_builder: &DanceBuilder) -> Result<(), String> {
        let Some(index) = self
            .dances
            .iter()
            .position(|dance| dance.id == dance_builder.id)
        else {
            return Err("Dance ID does not exist".to_owned());
        };
        self.dances[index] = dance_builder.build();
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
                    steps: dance
                        .step_ids
                        .iter()
                        .zip(dance.flip_orientation.iter())
                        .map(|(id, &flip_orientation)| DanceStep {
                            id: id.clone(),
                            flip_orientation,
                        })
                        .collect(),
                })
                .collect(),
        };
        let string = ron::ser::to_string(&file_data)?;
        Ok(string)
    }

    pub fn dances(&self) -> Vec<DanceInfo> {
        self.dances.iter().map(DanceInfo::from).collect()
    }

    #[wasm_bindgen(js_name = "danceBuilder")]
    pub fn dance_builder(&self, dance_id: String) -> Result<DanceBuilder, String> {
        self.dances
            .iter()
            .find(|dance| dance.id == dance_id)
            .map(|dance| DanceBuilder::from(dance.clone()))
            .ok_or_else(|| format!("missing dance {dance_id} in dance file builder"))
    }
}

impl From<DanceFile> for DanceFileBuilder {
    fn from(other: DanceFile) -> Self {
        Self {
            version: other.version,
            dances: other
                .dances
                .into_iter()
                .map(intern::dance::Dance::from)
                .collect(),
        }
    }
}
