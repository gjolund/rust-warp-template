FROM openfaas/of-watchdog:0.7.7 as watchdog

FROM alpine:3.12

# Set up watchdog for HTTP mode
ENV fprocess="main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

# Install packages and add non-root user
RUN addgroup -S app && adduser -S -g app app

ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1