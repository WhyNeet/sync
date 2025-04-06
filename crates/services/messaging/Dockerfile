FROM rust:1.85-slim AS builder

RUN rustup target add --toolchain nightly aarch64-unknown-linux-musl
RUN rustup default nightly
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app

# Cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./services/messaging/Cargo.toml ./services/messaging/Cargo.toml
RUN mkdir -p ./services/messaging/src && echo "fn main() {}" > services/messaging/src/main.rs
RUN cargo build --package messaging --target aarch64-unknown-linux-musl --release
RUN rm services/messaging/src/main.rs

COPY ./services/messaging/src ./services/messaging/src
RUN rm ./target/aarch64-unknown-linux-musl/release/messaging
RUN cargo build --package messaging --target aarch64-unknown-linux-musl --release


FROM alpine

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/messaging ./

EXPOSE 8080

CMD ["./messaging"]