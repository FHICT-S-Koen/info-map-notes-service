FROM rust:latest AS builder

RUN apt-get update && apt-get install libssl1.0.0 libssl-dev
RUN update-ca-certificates

ENV USER=user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /app

COPY ./ .

ARG AUTHORITY=abc

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

COPY --from=builder /app/target/release/notes-service ./

USER user:user

CMD ["/app/notes-service"]