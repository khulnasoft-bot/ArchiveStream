FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin archive-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/archive-api /usr/local/bin/archive-api
EXPOSE 3001
CMD ["archive-api"]
