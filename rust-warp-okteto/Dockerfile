FROM openfaas/of-watchdog:0.7.7 as watchdog

FROM austinrivas/rust-warp-builder:stable

RUN mkdir -p /home/app/main/src \
    && mkdir -p /home/app/function/src \
    && echo 'Welcome to your development environment. Happy coding!' > /etc/mod \
    && echo 'export PS1="\[\e[32m\]okteto\[\e[m\]> "' >> .bashrc

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

WORKDIR /home/app

ADD https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/src/main.rs ./main/src/
ADD https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/Cargo.toml ./main/
ADD https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/Cargo.lock ./main/

# Set up watchdog for HTTP mode
ENV fprocess="cargo run --manifest-path ./main/Cargo.toml"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

