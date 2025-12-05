use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::app::start_from_configuration as start_from_configuration_impl;
use crate::app::submit_timeline_frame as submit_timeline_frame_impl;
use crate::app::start_from_server_info as start_from_server_info_impl;

#[wasm_bindgen]
pub fn start_from_configuration(configuration: JsValue, assets: JsValue, canvas_id: Option<String>) -> Result<(), JsValue>{
    start_from_configuration_impl(configuration, assets, canvas_id)
        .map_err(|e: anyhow::Error| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn submit_timeline_frame(frame: JsValue) -> Result<(), JsValue> {
    submit_timeline_frame_impl(frame)
        .map_err(|e: anyhow::Error| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    Ok(())
}

#[wasm_bindgen]
pub fn start_from_server_info(url: String, token: Option<String>, canvas_id: Option<String>) -> Result<(), JsValue>{
    start_from_server_info_impl(url, token, canvas_id)
        .map_err(|e: anyhow::Error| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    Ok(())
}

