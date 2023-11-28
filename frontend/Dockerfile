FROM rust:1.73.0 AS chef

WORKDIR /usr/src/awesome-rust-frontend

RUN cargo install cargo-chef; \
    rm -rf $CARGO_HOME/registry

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/awesome-rust-frontend/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json
RUN apt install pkg-config

COPY . .
RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /app

COPY --from=builder /usr/src/awesome-rust-frontend/target/release/frontend .

CMD ["./frontend"]
