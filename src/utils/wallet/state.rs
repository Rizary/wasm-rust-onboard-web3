use futures_signals::signal::Mutable;

use super::{init, connect};


pub struct AppWallet {
    pub connection: Mutable<WalletStatus>, 
    pub wallet: Mutable<String>,
}

impl AppWallet {
    pub fn new() -> Self {
        // init();
        // connect();
        // // my_address();
        Self {
            connection: Mutable::new(WalletStatus::Disconnected),
            wallet: Mutable::new(String::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletStatus {
    Connected,
    Disconnected
}
