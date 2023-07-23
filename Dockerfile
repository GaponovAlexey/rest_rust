# FROM messense/rust-musl-cross:x86_64-musl AS build
# ENV SQLX_OFFLINE=true
# RUN cargo install cargo-chef
# WORKDIR /app
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
# RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# RUN cargo build --release --target x86_64-unknown-linux-musl

# FROM alpine:latest
# COPY --from=build /app/target/x86_64-unknown-linux-musl/release/rest_rust /rest_rust
# EXPOSE 3000
# ENV DATABASE_URL=postgres://postgres:1234567890@db:5432
# ENTRYPOINT ["/rest_rust"]

# Stage 1: Building the application
FROM rust AS builder

# Install musl tools and copy the current folder into the build folder
RUN apt-get update && apt-get install -y musl-tools 
WORKDIR /app
COPY . .

# Build the app for the target
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Creating the minimal runtime image
FROM alpine:latest

# Install curl (may be needed for health checks, etc.)
RUN apk --no-cache add curl

# Copy the runtime files
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rest_rust /app/rest_rust

WORKDIR /app
EXPOSE 3000
ENV DATABASE_URL=postgres://postgres:1234567890@db:5432

# Start the Rust server
CMD ["/app/rest_rust"]