use std::sync::Arc;

use crate::utils::wallet::AppWallet;

pub struct AppStorage {
    pub wallet: AppWallet,
}

impl AppStorage {
    pub fn new() -> Self {
        Self {
            wallet: AppWallet::new(),
        }
    }
}