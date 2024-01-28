use std::sync::Arc;

use crate::{keys_repository::KeysRepository, settings::SettingsReader};

//pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub settings_reader: Arc<SettingsReader>,
    pub keys_store: KeysRepository,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<crate::settings::SettingsReader>) -> Self {
        let keys_store = KeysRepository::new();

        Self {
            keys_store,
            settings_reader,
        }
    }
}
