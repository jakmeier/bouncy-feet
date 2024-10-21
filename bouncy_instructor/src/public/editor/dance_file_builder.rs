use crate::dance_file::DanceFile;
use crate::editor::ExportError;
use crate::wrapper::dance_wrapper::DanceWrapper;
use crate::{dance_file, STATE};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::dance_builder::DanceBuilder;

#[wasm_bindgen]
pub struct DanceFileBuilder {
    version: u16,
    dances: Vec<DanceWrapper>,
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
        if self
            .dances
            .iter()
            .any(|dance| dance.definition().id == dance_builder.id)
        {
            return Err("Dance ID already exists".to_owned());
        }
        self.dances.push(DanceWrapper::new(
            dance_builder.build(),
            dance_builder.global_collection.clone(),
        ));
        Ok(())
    }

    #[wasm_bindgen(js_name = "overwriteDance")]
    pub fn overwrite_dance(&mut self, dance_builder: &DanceBuilder) -> Result<(), String> {
        let Some(index) = self
            .dances
            .iter()
            .position(|dance| dance.definition().id == dance_builder.id)
        else {
            return Err("Dance ID does not exist".to_owned());
        };
        self.dances[index] = DanceWrapper::new(
            dance_builder.build(),
            dance_builder.global_collection.clone(),
        );
        Ok(())
    }

    #[wasm_bindgen(js_name = "removeDance")]
    pub fn remove_dance(&mut self, id: String) -> Result<(), String> {
        if let Some(index) = self
            .dances
            .iter()
            .position(|dance| dance.definition().id == id)
        {
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
                .map(DanceWrapper::definition)
                .cloned()
                .collect(),
        };
        let string = ron::ser::to_string(&file_data)?;
        Ok(string)
    }

    #[wasm_bindgen(js_name = "buildPrettyRon")]
    pub fn build_pretty_ron(&self) -> Result<String, ExportError> {
        let file_data = DanceFile {
            version: self.version,
            dances: self
                .dances
                .iter()
                .map(DanceWrapper::definition)
                .cloned()
                .collect(),
        };
        let string = ron::ser::to_string_pretty(&file_data, ron::ser::PrettyConfig::default())?;
        Ok(string)
    }

    pub fn dances(&self) -> Vec<DanceWrapper> {
        self.dances.clone()
    }

    #[wasm_bindgen(js_name = "danceBuilder")]
    pub fn dance_builder(&self, dance_id: String) -> Result<DanceBuilder, String> {
        self.dances
            .iter()
            .find(|dance| dance.id() == dance_id)
            .map(|dance| DanceBuilder::from(dance.clone()))
            .ok_or_else(|| format!("missing dance {dance_id} in dance file builder"))
    }
}

impl From<DanceFile> for DanceFileBuilder {
    fn from(other: DanceFile) -> Self {
        let global_collection = STATE.with_borrow(|state| {
            // FIXME: This clone is inefficient and fragile to modifications.
            Rc::new(state.global_db.clone())
        });
        Self {
            version: other.version,
            dances: other
                .dances
                .into_iter()
                .map(|d| DanceWrapper::new(d, global_collection.clone()))
                .collect(),
        }
    }
}
