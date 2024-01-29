mod app_ctx;
mod grpc_server;

mod keys_repository;
mod settings;

use app_ctx::AppContext;
use grpc_server::GrpcService;

use crate::crypt_manager_grpc::crypt_manager_grpc_service_server::CryptManagerGrpcServiceServer;

pub mod crypt_manager_grpc {
    tonic::include_proto!("crypt_manager");
}

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".reach-pay").await;

    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app = AppContext::new(settings_reader).await;
    let app = Arc::new(app);

    let grpc_service = GrpcService::new(app.clone());

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(CryptManagerGrpcServiceServer::new(grpc_service.clone()))
    });

    service_context.start_application().await;

    /*
    let settings_reader = SettingsReader::new(".merchant-message-crypt-manager").await;
    let settings_reader = Arc::new(settings_reader);

    let app = AppContext::new(&settings_reader).await;
    let app = Arc::new(app);

    let telemetry_writer =
        my_telemetry_writer::MyTelemetryWriter::new(APP_NAME.to_string(), settings_reader.clone());

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    http_is_alive_shared::start_up::start_server(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        app.app_states.clone(),
    );

    start_grpc_server(app.clone(), 8888);
    app.app_states.wait_until_shutdown().await;

     */
}
