use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::client::start_from_configuration as start_from_configuration_impl;

#[wasm_bindgen]
pub fn start_from_configuration(configuration: JsValue, canvas_id: Option<String>) -> Result<(), JsValue>{
    start_from_configuration_impl(configuration, canvas_id)
        .map_err(|e: anyhow::Error| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    Ok(())
}
