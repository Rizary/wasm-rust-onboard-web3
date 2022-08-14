use wasm_bindgen::prelude::*;

use crate::utils::wallet;

#[wasm_bindgen(module = "/js/wallet.js")]
extern "C" {
    fn _init();
    fn _connect();
    fn _disconnect();
    fn _myAddress() -> String;
    
}

pub fn init() {
    _init();
}

pub fn connect(){
    _connect();
}

pub fn disconnect() {
    _disconnect();
}

pub fn my_address() -> String {
    let address = _myAddress();
    log::info!("walletConnected: {}", address);
    return address
}