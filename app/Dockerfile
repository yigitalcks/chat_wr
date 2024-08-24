# Build stage
FROM rust:1.80 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Production stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app

# Expose port 8080
EXPOSE 8080

CMD ["app"]