FROM rust:slim
COPY ./target/release/crypt-manager ./target/release/crypt-manager
ENTRYPOINT ["./target/release/crypt-manager"]