FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /myapp

COPY . .

ARG AUTHORITY=abc

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

WORKDIR /myapp

COPY --from=builder /myapp/target/x86_64-unknown-linux-musl/release/myapp ./

CMD ["/myapp/myapp"]
