use thiserror::Error;
use wasm_bindgen::JsValue;

pub(crate) mod dance_builder;
pub(crate) mod dance_file_builder;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("exporting a dance fila as RON failed, {0}")]
    DanceFileRonError(#[from] ron::error::Error),
}

impl From<ExportError> for JsValue {
    fn from(value: ExportError) -> Self {
        format!("{value}").into()
    }
}
