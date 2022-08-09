use crate::home::Home;


use dominator::{html, Dom};
use futures_signals::signal::Signal;
use web_sys::console::log;
use crate::utils::routes::{HomeRoute, Route};
use std::sync::Arc;

pub struct Router {}

impl Router {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn render(state: Arc<Self>) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal(state))
        })
    }

    fn dom_signal(state: Arc<Self>) -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(move |url| {
            let route = Route::from_url(url);
            log::info!("route [{}]", route);
            match route {
                Route::Home(route) => match route {
                    HomeRoute::Home => Some(Arc::new(Home::new(state.clone())).render()),
                    _ => None,
                },
                _ => None,
            }
        })
    }
}