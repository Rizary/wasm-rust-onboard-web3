use crate::home::Home;
use crate::dashboard::Dashboard;
use crate::docs::Docs;
use crate::utils::storage::AppStorage;
use crate::utils::futures::AsyncLoader;


use dominator::{html, Dom};
use futures_signals::signal::Signal;
use web_sys::console::log;
use crate::utils::routes::{HomeRoute, DashboardRoute, DocsRoute, AdminRoute, Route};
use crate::utils::language::{Language, LanguageCode, LanguageName};
use crate::utils::lightning::LightningMode;
use std::sync::Arc;

pub struct Router {
    pub loader: AsyncLoader,
    pub language: Language,
    pub mode: LightningMode,
    pub storage: AppStorage,
}

impl Router {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            loader: AsyncLoader::new(),
            language: Language(LanguageCode::En, LanguageName::English),
            mode: LightningMode::Light,
            storage: AppStorage::new(),
        })
    }

    pub fn render(state: Arc<Self>) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal(state))
        })
    }

    fn dom_signal(state: Arc<Self>) -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(move |url| {
            let route = Route::from_url(url);
            match route {
                Route::Home(route) => match route {
                    HomeRoute::Home => Some(Arc::new(Home::new(state.clone())).render()),
                },
                Route::Dashboard(route) => match route {
                    DashboardRoute::Dashboard => Some(Arc::new(Dashboard::new(state.clone())).render()),
                },
                Route::Docs(route) => match route {
                    DocsRoute::Docs => Some(Arc::new(Docs::new(state.clone())).render()),
                },
                _ => None,
            }
        })
    }
}