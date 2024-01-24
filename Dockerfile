FROM rust:1.69 as build

RUN USER=root cargo new --bin core
WORKDIR /core

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /core/target/release/ /core

CMD ["/core"]