# syntax=docker/dockerfile:1.10.0
FROM rust:1.84.1 AS build

ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y git curl cmake libssl-dev libclang-dev libprotobuf-dev protobuf-compiler g++ pkg-config libfontconfig libfontconfig1-dev clang libpq-dev

WORKDIR /service

RUN cargo init

COPY ./Cargo.toml /service/Cargo.toml
COPY ./Cargo.lock /service/Cargo.lock

RUN cargo fetch

RUN cargo build --release

COPY ./src ./src

RUN touch ./src/main.rs && cargo build --release

FROM ubuntu:rolling AS runtime

USER 1000:1000

WORKDIR /service

COPY --from=build /service/target/release/service /service/service

EXPOSE 80

CMD ["/service/service"]

