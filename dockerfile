FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/notes-service
COPY . .

ARG DATABASE_URL=abc
ARG AUTHORITY=abc

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/notes-service /usr/local/bin/notes-service

CMD ["notes-service"]
