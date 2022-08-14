use std::sync::Arc;

use crate::router::Router;

pub struct Dashboard {
    pub app: Arc<Router>
}

impl Dashboard {
    pub fn new(app: Arc<Router>) -> Self {
        Self { app }
    }
}
