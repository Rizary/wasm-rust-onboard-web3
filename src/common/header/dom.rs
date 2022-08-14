use std::sync::Arc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use strum::IntoEnumIterator;
use crate::utils::routes::{AdminRoute, Route};
use crate::utils::wallet::{WalletStatus, connect, init};
use crate::home::Home;
use crate::common::header::state::Header;
use crate::common::button::{Button, ButtonStyle, ButtonStyleIcon};

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
                                .class(["inline-flex","items-center","px-1","pt-1","border-b-2","text-sm","font-medium"])
                                .prop("kind", page_link.kind_str())
                                .prop("href", &page_link.route())
                                .prop("icon", "icon")
                                .text(page_link.kind_str())
                            })
                        }))
                    }),
                    html!("div", {
                        .child_signal(state.app.storage.wallet.connection.signal_ref(clone!(state => move |connection| {
                            Some(if connection == &WalletStatus::Disconnected {
                                let style = ButtonStyle::Icon(ButtonStyleIcon::Metamask);
                                let button = Button::new_label(style, String::from("connect"), clone!(state => move || {
                                    init();
                                    connect();
                                    
                                }));
                                Button::render(button)
                            } else {
                                let style = ButtonStyle::Icon(ButtonStyleIcon::Metamask);
                                let button = Button::new_label(style, String::from("disconnect"), clone!(state => move || {
                                    // state.stop();
                                    init();
                                    connect();
                                }));
                                Button::render(button)
                            })
                        })))
                    }),
                ])
            })
        )
    })
}