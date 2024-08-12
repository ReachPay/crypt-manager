use crypt_utils::*;
service_sdk::macros::use_grpc_server!();
use serde_json::Value;

use super::server::GrpcService;
use crate::crypt_manager_grpc::crypt_manager_grpc_service_server::CryptManagerGrpcService;
use crate::crypt_manager_grpc::*;

#[tonic::async_trait]
impl CryptManagerGrpcService for GrpcService {
    #[with_telemetry]
    async fn crypt_message(
        &self,
        request: tonic::Request<CryptCallbackMessageRequest>,
    ) -> Result<tonic::Response<CryptCallbackMessageResponse>, tonic::Status> {
        let request = request.into_inner();

        let CryptCallbackMessageRequest {
            message,
            merchant_id,
        } = request;

        let key = match self
            .app
            .keys_store
            .get_merchant_private_key(&merchant_id)
            .await
        {
            Some(key) => key,
            None => {
                let (private, _) = generate_rsa_key_pair().unwrap();
                self.app
                    .keys_store
                    .update_key(&merchant_id, private.clone())
                    .await;
                private.clone()
            }
        };

        let message: Value = serde_json::from_str(&message).unwrap();
        let result = encode_message_to_jwt_rsa(message, &key).unwrap();

        return Ok(tonic::Response::new(CryptCallbackMessageResponse {
            message: result,
        }));
    }

    #[with_telemetry]
    async fn get_public_key(
        &self,
        request: tonic::Request<GetMerchantPublicKeyRequest>,
    ) -> Result<tonic::Response<GetMerchantPublicKeyResponse>, tonic::Status> {
        let request = request.into_inner();

        let key = match self
            .app
            .keys_store
            .get_merchant_private_key(&request.merchant_id)
            .await
        {
            Some(key) => key,
            None => {
                let (private, _) = generate_rsa_key_pair().unwrap();
                self.app
                    .keys_store
                    .update_key(&request.merchant_id, private.clone())
                    .await;
                private.clone()
            }
        };

        println!("Get public key request key: {:?}", key);

        let result = get_public_rsa_key_from_private(key).unwrap();

        println!("Get public key request key: {:?}", result);
        return Ok(tonic::Response::new(GetMerchantPublicKeyResponse {
            key_content: Some(String::from_utf8(result).unwrap()),
        }));
    }

    #[with_telemetry]
    async fn reset_key_pair(
        &self,
        request: tonic::Request<ResetKeyPairRequest>,
    ) -> Result<tonic::Response<GetMerchantPublicKeyResponse>, tonic::Status> {
        println!("Reset key pair request");
        let request = request.into_inner();

        let (private, public) = generate_rsa_key_pair().unwrap();
        println!(
            "Reset key pair request key: p:{:?}, p: {:?}",
            private, public
        );
        self.app
            .keys_store
            .update_key(&request.merchant_id, private.clone())
            .await;

        return Ok(tonic::Response::new(GetMerchantPublicKeyResponse {
            key_content: Some(String::from_utf8(public).unwrap()),
        }));
    }

    async fn ping(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
