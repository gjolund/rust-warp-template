FROM rust:1.44.0

WORKDIR /home/rust

ENV TARGET_ENV x86_64-unknown-linux-musl

RUN rustup target add ${TARGET_ENV} \
    && USER=root cargo new --bin main \
    && USER=root cargo new --lib function

COPY ./Cargo.lock ./Cargo.toml ./main/

RUN cd main \
    && cargo build \
        --target ${TARGET_ENV} \
        --release \
    && rm src/*.rs