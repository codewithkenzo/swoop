# Use the official Rust image with a slim Debian base
FROM rust:1.79.0-slim-bullseye AS builder

# Install build essentials, which might be needed for some dependencies
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev

# Create a new empty shell project
WORKDIR /usr/src/swoop
COPY . .

# Build the TUI binary in release mode
RUN cargo build --release --bin swoop-tui

# Final stage, copy the binary from the builder stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/swoop/target/release/swoop-tui /usr/local/bin/swoop-tui
COPY ./urls.txt /usr/local/bin/urls.txt

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/swoop-tui"]
