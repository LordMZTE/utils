FROM rust:alpine as builder
LABEL maintainer="LordMZTE <https://github.com/lordmzte>"
RUN apk add build-base

WORKDIR /usr/src/utils
COPY Cargo.toml ./

COPY mcstat/ mcstat/
COPY tmod/ tmod/

RUN cargo install --path mcstat
RUN cargo install --path tmod
