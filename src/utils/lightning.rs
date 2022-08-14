#[derive(Debug, Clone)]
pub enum LightningMode {
    Dark,
    Light
}

impl From<&LightningMode> for String {
    fn from(lightning_mode: &LightningMode) -> Self {
        match lightning_mode {
            LightningMode::Dark => "dark".to_string(),
            LightningMode::Light => "light".to_string(),
        }
    }
}