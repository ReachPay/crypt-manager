FROM ubuntu:22.04
COPY ./target/release/crypt-manager ./target/release/crypt-manager
ENTRYPOINT ["./target/release/crypt-manager"]