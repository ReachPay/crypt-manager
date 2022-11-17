use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{
    
    SettingsModel, KeysRepository,
    
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub keys_store: KeysRepository,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        
        let settings = settings_reader.get_settings().await;
        let app_states = Arc::new(AppStates::create_initialized());
        let keys_store = KeysRepository::new();

        Self {
            keys_store,
            app_states,
            settings,
            
        }
    }
}

