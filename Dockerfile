# Linera Poker - Production Docker Image
# Base: Rust 1.86 with Linera SDK 0.15.8 and Node.js LTS

FROM rust:1.86-slim

LABEL maintainer="Linera Poker Team"
LABEL description="Linera Poker Wave 5 - Cross-chain poker game"
LABEL version="1.0.0"

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies required for Linera SDK and WASM compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    protobuf-compiler \
    clang \
    make \
    curl \
    git \
    python3 \
    libssl-dev \
    ca-certificates \
    gnupg \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 22 LTS directly (simpler than nvm for Docker)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Linera SDK 0.15.8 from crates.io (latest release, Dec 2024)
RUN cargo install linera-service@0.15.8 linera-storage-service@0.15.8

# Add WebAssembly target for contract compilation
RUN rustup target add wasm32-unknown-unknown

# Verify installations
RUN linera --version && \
    rustc --version && \
    cargo --version && \
    node --version && \
    npm --version

# Set working directory
WORKDIR /build

# Expose required ports
# 5173: Vite frontend development server
# 8080: Linera faucet service
# 9001: Linera GraphQL query service
# 13001: Linera validator/shard service
EXPOSE 5173 8080 9001 13001

# Healthcheck: Verify frontend is accessible
# Start period is 5 min to allow for contract build/deployment
HEALTHCHECK --interval=30s --timeout=10s --start-period=300s --retries=5 \
    CMD curl -f http://localhost:5173/ || exit 1

# Default entrypoint: Convert line endings (CRLF to LF) and run the script
# This handles Windows volume mounts where files may have CRLF endings
# Copy to temp, convert, then run (handles read-only mounts)
ENTRYPOINT ["bash", "-c", "cp /build/run.bash /tmp/run.bash && sed -i 's/\\r$//' /tmp/run.bash && bash /tmp/run.bash"]
