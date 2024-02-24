use crate::dance_file::DanceFile;
use crate::editor::ExportError;
use crate::{dance_file, intern};
use wasm_bindgen::prelude::wasm_bindgen;

use super::dance_builder::DanceBuilder;

#[wasm_bindgen]

pub struct DanceFileBuilder {
    version: u16,
    dances: Vec<intern::dance::Dance>,
}

#[wasm_bindgen]
impl DanceFileBuilder {
    pub fn new() -> Self {
        Self {
            version: dance_file::CURRENT_VERSION,
            dances: vec![],
        }
    }

    #[wasm_bindgen(js_name = "addDance")]
    pub fn add_dance(&mut self, dance_builder: &DanceBuilder) {
        self.dances.push(dance_builder.build());
    }

    pub fn build(&self) -> Result<String, ExportError> {
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
        let config = ron::ser::PrettyConfig::default();
        let string = ron::ser::to_string_pretty(&file_data, config)?;
        return Ok(string);
    }
}
