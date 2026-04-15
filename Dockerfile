
# ---- Build stage ----
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true

RUN cargo build --release

# ---- Run stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/askeet_api_rust .

EXPOSE 3000

CMD ["./askeet_api_rust"]
