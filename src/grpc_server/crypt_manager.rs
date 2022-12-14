use crypt_utils::{
    encode_message_to_jwt_rsa, generate_rsa_keypeir, get_public_rsa_key_from_private,
};
use serde_json::Value;

use super::server::GrpcService;
use crate::crypt_manager_grpc::crypt_manager_grpc_service_server::CryptManagerGrpcService;
use crate::crypt_manager_grpc::*;

#[tonic::async_trait]
impl CryptManagerGrpcService for GrpcService {
    async fn crypt_message(
        &self,
        request: tonic::Request<CryptCallbackMessgeRequest>,
    ) -> Result<tonic::Response<CryptCallbackMessgeResponse>, tonic::Status> {
        println!("Got a request");
        let CryptCallbackMessgeRequest {
            message,
            merchant_id,
        } = request.into_inner();

        let key = match self
            .app
            .keys_store
            .get_merchant_private_key(&merchant_id)
            .await
        {
            Some(key) => key,
            None => {
                let (private, _) = generate_rsa_keypeir().unwrap();
                self.app
                    .keys_store
                    .update_key(&merchant_id, private.clone())
                    .await;
                private.clone()
            }
        };

        let message: Value = serde_json::from_str(&message).unwrap();
        let result = encode_message_to_jwt_rsa(message, &key).unwrap();

        return Ok(tonic::Response::new(CryptCallbackMessgeResponse {
            message: result,
        }));
    }

    async fn get_public_key(
        &self,
        request: tonic::Request<GetMerchantPublicKeyRequest>,
    ) -> Result<tonic::Response<GetMerchantPublicKeyResponse>, tonic::Status> {
        println!("Get public key request");

        let GetMerchantPublicKeyRequest { merchant_id } = request.into_inner();

        let key = match self
            .app
            .keys_store
            .get_merchant_private_key(&merchant_id)
            .await
        {
            Some(key) => key,
            None => {
                let (private, _) = generate_rsa_keypeir().unwrap();
                self.app
                    .keys_store
                    .update_key(&merchant_id, private.clone())
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

    async fn reset_key_pair(
        &self,
        request: tonic::Request<GetMerchantPublicKeyRequest>,
    ) -> Result<tonic::Response<GetMerchantPublicKeyResponse>, tonic::Status> {
        println!("Reset key pair request");
        let GetMerchantPublicKeyRequest { merchant_id } = request.into_inner();

        let (private, public) = generate_rsa_keypeir().unwrap();
        println!(
            "Reset key pair request key: p:{:?}, p: {:?}",
            private, public
        );
        self.app
            .keys_store
            .update_key(&merchant_id, private.clone())
            .await;

        return Ok(tonic::Response::new(GetMerchantPublicKeyResponse {
            key_content: Some(String::from_utf8(public).unwrap()),
        }));
    }
}
