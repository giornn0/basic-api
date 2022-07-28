FROM rust:1.62-bullseye  as builder

RUN USER=root cargo new --bin base-api
WORKDIR ./base-api
RUN rm -r *

ADD . .

RUN cargo build --release


FROM debian:bullseye-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/* 

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /base-api/target/release/base-api ${APP}/base-api
COPY --from=builder /base-api/.env ${APP}/.env
COPY --from=builder /base-api/.tls ${APP}/.tls

RUN chown -R $APP_USER:$APP_USER ${APP}

EXPOSE 8080

USER $APP_USER
WORKDIR ${APP}

CMD ["./base-api"]