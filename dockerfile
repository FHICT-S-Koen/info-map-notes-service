FROM rust:latest AS builder

RUN update-ca-certificates

ENV USER=myip
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /myip

COPY ./ .

ARG AUTHORITY=abc

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

COPY --from=builder /myip/target/release/myip ./

USER myip:myip

CMD ["/myip/myip"]