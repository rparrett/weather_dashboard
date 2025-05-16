# Builder image
FROM rust:1-bookworm as builder
WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Get compiled binary and other files from builder's cargo install directory
COPY --from=builder /usr/src/app/target/release/server .
COPY --from=builder /usr/src/app/server/config.toml ./config.toml
COPY --from=builder /usr/src/app/server/templates ./templates
COPY --from=builder /usr/src/app/server/static ./static

EXPOSE 8080

# Run the app
CMD ./server
