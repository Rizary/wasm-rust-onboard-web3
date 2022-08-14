use std::sync::Arc;

use crate::{utils::routes::{HomeRoute, DashboardRoute, DocsRoute, AdminRoute, Route}, router::Router};
use strum_macros::{Display, EnumIter};

pub struct Header {
    pub app: Arc<Router>
}

impl Default for Header {
    fn default() -> Self {
        Self::new(Router::new())
    }
}

impl Header {
    pub fn new(app: Arc<Router>) -> Self {
        Self { app }
    }
}

#[derive(Clone, Debug, PartialEq, EnumIter)]
pub enum PageLinks {
    Home,
    Dashboard,
    Docs,
    // Admin,
}

impl PageLinks {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Dashboard => "dashboard",
            Self::Docs => "docs",
            // Self::Admin => "community",
        }
    }
    pub fn route(&self) -> String {
        match self {
            Self::Home => Route::Home(HomeRoute::Home).to_string(),
            Self::Dashboard => Route::Dashboard(DashboardRoute::Dashboard).to_string(),
            Self::Docs => Route::Docs(DocsRoute::Docs).to_string(),
            // Self::Admin => "javascript:alert(\"Coming soon\")".to_string(),
        }
    }
}