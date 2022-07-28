FROM rust:1.62-bullseye  as builder

RUN USER=root cargo new --bin migrate
WORKDIR ./migrate
RUN rm -r *

ADD ./Cargo.toml ./Cargo.toml
ADD .env .env

RUN cargo install diesel_cli

COPY ./migrations ./migrations

CMD ["diesel","setup"]