# ---- Build stage ----
FROM rust:1.75 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# ---- Run stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copie uniquement le binaire compilé
COPY --from=builder /app/target/release/askeet_api_rust .

EXPOSE 3000

CMD ["./askeet_api_rust"]
