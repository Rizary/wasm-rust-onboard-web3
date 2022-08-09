use std::sync::Arc;

use crate::router::Router;

pub struct Home {
    pub app: Arc<Router>,
}

impl Home {
    pub fn new(app: Arc<Router>) -> Self {
        Self { app }
    }
}