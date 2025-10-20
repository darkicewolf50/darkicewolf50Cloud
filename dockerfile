# ----------- Build Stage -----------
FROM rust:alpine AS builder

WORKDIR /darkicewolf50_cloud
    
# Install build dependencies
RUN apk add --no-cache pkgconfig musl-dev

# Copy source and build
COPY . .
RUN cargo build --release
    
# ----------- Runtime Stage -----------
FROM alpine:latest
    
# Install runtime dependencies
RUN apk add --no-cache pkgconfig musl-dev
    
WORKDIR /darkicewolf50_cloud
COPY --from=builder /darkicewolf50_cloud/target/release/darkicewolf50_cloud .
    
EXPOSE 5050
CMD ["./darkicewolf50_cloud"]
    