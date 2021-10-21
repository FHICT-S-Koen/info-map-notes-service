FROM rust:latest AS builder

WORKDIR /app

COPY . .

ARG AUTHORITY=abc

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/notes-service ./

USER user:user

CMD ["/app/notes-service"]