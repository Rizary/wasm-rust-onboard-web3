use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};

pub fn get_local_storage() -> Result<Storage, JsValue> {
    window()
        .unwrap()
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("could not get local storage!"))
}