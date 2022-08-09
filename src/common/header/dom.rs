use std::sync::Arc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use strum::IntoEnumIterator;
use crate::utils::routes::{Route};
use crate::home::Home;
use crate::common::header::state::Header;

use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use crate::common::header::state::PageLinks;

const STR_HOME: &str = "Sign up";
const STR_DASHBOARD: &str = "Dashboard";
const STR_DOCS: &str = "Docs";

pub fn render(
    state: Arc<Header>
) -> Dom {
    html!("header", {
        .child(
            html!("div", {
                .children(&mut [
                    html!("a", {
                        .prop("href", PageLinks::Home.route())
                        .text("LOGO")
                    }),
                    html!("ul", {
                        .children(PageLinks::iter().map(|page_link| {
                            html!("a", {
                                .prop("kind", page_link.kind_str())
                                .prop("href", &page_link.route())
                                .text(page_link.kind_str())
                            })
                        }))
                    })
                ])
            })
        )
    })
}