FROM rust:1.85-slim AS builder

RUN rustup target add --toolchain nightly aarch64-unknown-linux-musl
RUN rustup default nightly
RUN apt update && apt install -y musl-tools musl-dev cmake gcc-aarch64-linux-gnu g++-aarch64-linux-gnu curl xz-utils
RUN curl -L https://musl.cc/aarch64-linux-musl-cross.tgz | tar xz -C /usr/local
RUN update-ca-certificates
ENV CC=/usr/local/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc
ENV CXX=/usr/local/aarch64-linux-musl-cross/bin/aarch64-linux-musl-g++

WORKDIR /app

# Cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Initialize packages with empty entrypoints
COPY ./services/messaging/Cargo.toml ./services/messaging/Cargo.toml
RUN mkdir -p ./services/messaging/src && echo "fn main() {}" > services/messaging/src/main.rs
COPY ./libs/auth/Cargo.toml ./libs/auth/Cargo.toml
RUN mkdir -p ./libs/auth/src && echo "fn main() {}" > libs/auth/src/main.rs
COPY ./libs/common/Cargo.toml ./libs/common/Cargo.toml
RUN mkdir -p ./libs/common/src && echo "fn main() {}" > libs/common/src/main.rs


# Cache auth
RUN cargo build --package auth --target aarch64-unknown-linux-musl --release
RUN rm libs/auth/src/main.rs

COPY ./libs/auth/src ./libs/auth/src
RUN cargo build --package auth --target aarch64-unknown-linux-musl --release

# Cache common
RUN cargo build --package common --target aarch64-unknown-linux-musl --release
RUN rm libs/common/src/main.rs

COPY ./libs/common/src ./libs/common/src
RUN cargo build --package common --target aarch64-unknown-linux-musl --release

# Cache binary
# Dummy main is already present
RUN cargo build --package messaging --target aarch64-unknown-linux-musl --release
RUN rm services/messaging/src/main.rs

COPY ./services/messaging/src ./services/messaging/src
RUN rm ./target/aarch64-unknown-linux-musl/release/messaging
RUN cargo build --package messaging --target aarch64-unknown-linux-musl --release


FROM alpine

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/messaging ./

EXPOSE 8080

CMD ["./messaging"]