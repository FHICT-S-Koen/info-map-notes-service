FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=myapp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /myapp

COPY . .

ARG AUTHORITY=abc

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myapp

COPY --from=builder /myapp/target/x86_64-unknown-linux-musl/release/myapp ./

CMD ["/myapp/myapp"]
