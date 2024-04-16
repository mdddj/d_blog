FROM rust:1.77.2 as builder
RUN USER=root cargo new --bin dokcer-rust-web
WORKDIR ./docker-rust-web
COPY ./Cargo.toml ./Cargo.toml


RUN cargo build --release \
    && rm src/*.rs target/release/deps/docker-rust-web*


ADD . ./
RUN cargo build --release

FROM debian:buster-slim

ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 5800

ENV TZ=Etc/UTC
ENV APP_USER=appuser

RUN groupadd  $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /docker-rust-web/target/release/docker-rust-web ${APP}/docker-rust-web

RUN chown -R $APP_USER:$APP_USER ${App}

USER $APP_USER
WORKDIR ${APP}
CMD ["./docker-rust-web"]