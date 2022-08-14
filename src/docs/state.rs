use std::sync::Arc;

use crate::router::Router;

pub struct Docs {
    pub app: Arc<Router>
}

impl Docs {
    pub fn new(app: Arc<Router> ) -> Self {
        Self { app }
    }
}