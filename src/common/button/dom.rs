use super::state::*;
use dominator::{clone, html, Dom};
use log;
use std::sync::Arc;

use dominator::events;

impl Button {
    pub fn render(state: Arc<Self>) -> Dom {
        let icon_prop = state.icon_str();
        log::info!("icon: {}", icon_prop.unwrap());

        let label = state.label.as_ref();

        html!("button", {
            .apply_if(state.on_click.is_some(), |dom| {
                dom.event(clone!(state => move |_evt:events::Click| {
                    (state.on_click.as_ref().unwrap()) ();
                }))
            })
            .text(label.unwrap())
        })
    }
}
