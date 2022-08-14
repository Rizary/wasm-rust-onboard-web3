mod router;
mod home;
mod docs;
mod common;
mod dashboard;
mod utils;

use utils::wallet::{init, connect};
use wasm_bindgen::prelude::*;
use router::Router;

use std::sync::Arc;
use cfg_if::cfg_if;

#[wasm_bindgen(start)]
pub async fn main_js() {
    setup_logger();
    init();
    connect();
    let router = Arc::new(Router::new());
    dominator::append_dom(&dominator::get_id("app"), Router::render(Router::new()));

    std::mem::forget(Box::new(router));
}

cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
            console_error_panic_hook::set_once();
        }
    } else {
        fn setup_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}