# Use rust:alpine as the builder stage
FROM rust:alpine AS builder

RUN apk update && apk add musl-dev
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl
FROM alpine:latest

RUN apk --no-cache add curl
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rest_rust /app/rest_rust

WORKDIR /app

EXPOSE 3000
ENV DATABASE_URL=postgres://postgres:1234567890@db:5432
CMD ["/app/rest_rust"]
