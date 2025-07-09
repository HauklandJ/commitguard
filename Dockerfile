# Multi-stage build for Rust application
FROM rust:1.70 as builder

# Set working directory
WORKDIR /app

# Copy Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage - use smaller base image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -s /bin/bash appuser

# Copy binary from builder stage
COPY --from=builder /app/target/release/guard /usr/local/bin/guard

# Switch to non-root user
USER appuser

# Set the entrypoint
ENTRYPOINT ["guard"]
CMD ["--help"]
