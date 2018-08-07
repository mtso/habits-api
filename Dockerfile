# build

FROM mtso/rust-rocksdb-build:5.14.2

WORKDIR /root

RUN /install-rust.sh

COPY Cargo.toml .
COPY src src

RUN cargo build --release


# copy artifacts for deploy

FROM debian:jessie-slim

# RUN apt-get install -y ca-certificates

ARG SERVICE_NAME

WORKDIR /app

COPY Rocket.toml .

COPY --from=0 /root/target/release/$SERVICE_NAME .

ENV SERVICE_NAME $SERVICE_NAME
ENTRYPOINT /app/$SERVICE_NAME
