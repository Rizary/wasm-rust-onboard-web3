use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};
use crate::utils::storage;

pub const WALLET_STORAGE_NAME: &str = "X-WALLET";

pub fn load_wallet() -> Option<String> {
    let res = storage::get_local_storage()
        .unwrap()
        .get(WALLET_STORAGE_NAME)
        .unwrap();

    if res.is_none() {
        log::warn!("unable to load Wallet!");
    }

    res
}

pub fn save_wallet(wallet: &str) {
    let local_storage = storage::get_local_storage().unwrap();

    local_storage.set(WALLET_STORAGE_NAME, wallet).unwrap()
}

pub fn delete_wallet() -> Result<(), JsValue> {
    let local_storage = storage::get_local_storage().unwrap();

    local_storage.remove_item(WALLET_STORAGE_NAME)
}