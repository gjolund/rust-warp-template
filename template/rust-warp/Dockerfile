FROM austinrivas/rust-warp-builder:stable as builder

COPY ./main/Cargo.lock ./main/Cargo.toml ./main/
COPY ./main/src ./main/src
COPY ./function/Cargo.lock ./function/Cargo.toml ./function/
COPY ./function/src ./function/src

RUN cd main \
    && cargo build \
        --target x86_64-unknown-linux-musl \
        --release

FROM austinrivas/rust-warp-runner:stable as runner

COPY --from=builder /home/rust/main/target/x86_64-unknown-linux-musl/release/main /usr/bin/main

# OpenFaaS vars
ENV cgi_headers="true"
ENV exec_timeout="10s"
ENV write_timeout="15s"
ENV read_timeout="15s"

CMD ["fwatchdog"]
