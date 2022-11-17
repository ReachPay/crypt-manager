use std::sync::Arc;

use crypt_manager::{SettingsReader, AppContext, APP_NAME, APP_VERSION, start_grpc_server};

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".merchant-message-crypt-manager").await;
    let settings_reader = Arc::new(settings_reader);

    let app = AppContext::new(&settings_reader).await;
    let app = Arc::new(app);

    let telemetry_writer = my_telemetry_writer::MyTelemetryWriter::new(
        APP_NAME.to_string(),
        settings_reader.clone(),
    );
    

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    http_is_alive_shared::start_up::start_server(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        app.app_states.clone(),
    );

    start_grpc_server(app.clone(), 8888);
    app.app_states.wait_until_shutdown().await;
}
