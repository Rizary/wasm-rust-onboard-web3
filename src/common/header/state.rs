use std::sync::Arc;

use crate::{utils::routes::{HomeRoute, Route}, router::Router};
use strum_macros::{Display, EnumIter};

pub struct Header {
    app: Arc<Router>
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
}

impl PageLinks {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Home => "home",
            // Self::Admin => "community",
        }
    }
    pub fn route(&self) -> String {
        match self {
            Self::Home => Route::Home(HomeRoute::Home).to_string(),
            // Self::Admin => "javascript:alert(\"Coming soon\")".to_string(),
        }
    }
}