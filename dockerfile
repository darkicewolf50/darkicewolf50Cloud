# ----------- Build Stage -----------
FROM rust:1.87-slim AS builder

WORKDIR /darkicewolf50_cloud
    
# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
    
# Copy source and build
COPY . .
RUN cargo build --release
    
# ----------- Runtime Stage -----------
FROM debian:bookworm-slim
    
# Install runtime dependencies (e.g., for OpenSSL if needed)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
    
WORKDIR /darkicewolf50_cloud
COPY --from=builder /darkicewolf50_cloud/target/release/darkicewolf50_cloud .
    
EXPOSE 8000
CMD ["./darkicewolf50_cloud"]
    