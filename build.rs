fn main() {
    tonic_build::compile_protos("proto/CryptManagerGrpcService.proto").unwrap();
}