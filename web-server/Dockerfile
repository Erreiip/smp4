FROM rust:1.91.1-bullseye AS builder
ARG BUILD_PATH=/app/build

ARG HTTP_PROXY
ARG HTTPS_PROXY

WORKDIR ${BUILD_PATH}

ENV http_proxy ${HTTP_PROXY}
ENV https_proxy ${HTTPS_PROXY}

# Download needed dependencies to build the app
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    clang \
    llvm \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

# Copy sources
COPY Cargo.toml Cargo.toml
COPY src src
COPY migrations migrations

# Build the executable
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.19 AS server
ARG CUSTOM_USER=rustuser
ARG BUILD_PATH=/app/build
ARG APP_PATH=/app

WORKDIR ${APP_PATH}

# Needed environment var by the app
ARG EXPOSE_IP=0.0.0.0
ARG EXPOSE_PORT=8888
ARG DATABASE_URL
ARG JWT_SECRET
ARG HASH_SECRET

# Create the custom user
RUN addgroup -S ${CUSTOM_USER} && adduser -S ${CUSTOM_USER} -G ${CUSTOM_USER}
RUN chown ${CUSTOM_USER}:${CUSTOM_USER} ${APP_PATH}

# Copy the built executable
COPY --from=builder --chown=${CUSTOM_USER}:${CUSTOM_USER} \
  ${BUILD_PATH}/target/x86_64-unknown-linux-musl/release/SMP4_web_server \
  SMP4_web_server

EXPOSE 8888

ENTRYPOINT ["./SMP4_web_server"]