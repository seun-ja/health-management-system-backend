FROM rust:1.89-bookworm as builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY . .
RUN cargo install sqlx-cli --no-default-features --features postgres
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /app/target/release/health_mgt_system /usr/local/bin/health_mgt_system
COPY migrations /migrations

EXPOSE 8080

CMD sqlx migrate run --source /migrations && health_mgt_system
