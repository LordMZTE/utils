FROM rust:alpine as builder
LABEL maintainer="LordMZTE <https://github.com/lordmzte>"
RUN apk add build-base

WORKDIR /usr/src/utils
COPY Cargo.toml ./

COPY mcstat/ mcstat/
COPY tmod/ tmod/
COPY figclock/ figclock/

RUN cargo install --path mcstat
RUN cargo install --path tmod
RUN cargo install --path figclock

FROM alpine
COPY --from=builder /usr/local/cargo/bin/ /usr/bin
