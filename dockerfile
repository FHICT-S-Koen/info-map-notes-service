FROM rust:latest AS builder

WORKDIR /myapp

COPY . .

ARG AUTHORITY=abc

RUN cargo build

WORKDIR /myapp

CMD ["/myapp/notes-service"]
