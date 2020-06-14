FROM openfaas/of-watchdog:0.7.7 as watchdog

FROM austinrivas/rust-warp-builder:stable

WORKDIR /root/

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

ENV FUNC_HOME_DIR=/home/app
ENV TARGET_ENV=x86_64-unknown-linux-musl

RUN mkdir -p ${FUNC_HOME_DIR} \
  && cp -r /home/rust/main /home/app/ \
  && cp -r /home/rust/function /home/app/ \
  && chmod +x /usr/bin/fwatchdog \
  && addgroup --system --gid 12000 fn_group \ 
  && adduser --system --home ${FUNC_HOME_DIR} -u 12000 --gid 12000 fn_user \
  && chown fn_user:fn_group -R ${FUNC_HOME_DIR} \
  && chown fn_user:fn_group -R /usr/local \
  && chmod 777 /tmp

WORKDIR ${FUNC_HOME_DIR}

RUN echo 'Welcome to your development environment. Happy coding!' > /etc/mod \
    && echo 'export PS1="\[\e[32m\]okteto\[\e[m\]> "' >> .bashrc

USER fn_user

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
ENV TEMPLATE_SOURCE_URL=https://raw.githubusercontent.com/austinrivas/rust-warp-template/master/template/rust-warp/main

ADD --chown=fn_user:fn_group ${TEMPLATE_SOURCE_URL}/src/main.rs ./main/src/
ADD --chown=fn_user:fn_group ${TEMPLATE_SOURCE_URL}/Cargo.toml ./main/
ADD --chown=fn_user:fn_group ${TEMPLATE_SOURCE_URL}/Cargo.lock ./main/

