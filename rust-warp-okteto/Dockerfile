FROM openfaas/of-watchdog:0.7.7 as watchdog

FROM frolvlad/alpine-gcc

WORKDIR /root/

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

ENV FUNC_HOME_DIR=/home/app
ENV FUNC_HANDLER_DIR=${FUNC_HOME_DIR}/function
ENV FUNC_MAIN_DIR=${FUNC_HOME_DIR}/main

RUN mkdir -p ${FUNC_HANDLER_DIR} \
  && mkdir -p ${FUNC_MAIN_DIR} \
  && chmod +x /usr/bin/fwatchdog \
  && apk --no-cache add ca-certificates rust cargo \
  && addgroup -S -g 12000 fnGroup \
  && adduser -S --home ${FUNC_HOME_DIR} -u 12000 -g 12000 fnUser \
  && chown fnUser:fnGroup -R ${FUNC_HOME_DIR} \
  && chmod 777 /tmp

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

# OpenFaaS vars
ENV cgi_headers="true"
ENV exec_timeout="10s"
ENV write_timeout="15s"
ENV read_timeout="15s"
# Set up watchdog for HTTP mode
ENV fprocess="cargo run --manifest-path ./main/Cargo.toml"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

WORKDIR ${FUNC_HOME_DIR}

RUN chown fnUser:fnGroup -R /usr/local \
    && apk --no-cache add bash \
    && echo 'Welcome to your development environment. Happy coding!' > /etc/mod \
    && echo 'export PS1="\[\e[32m\]okteto\[\e[m\]> "' >> .bashrc

USER fnUser

ADD --chown=fnUser:fnGroup https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/src/main.rs ./main/src/
ADD --chown=fnUser:fnGroup https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/Cargo.toml ./main/
ADD --chown=fnUser:fnGroup https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main/Cargo.lock ./main/

