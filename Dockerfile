FROM rust:1.74.1-buster as builder

WORKDIR /app
RUN apt update && apt install pkg-config libssl-dev libc-dev -y
COPY . .
RUN cargo build --release


FROM debian:buster-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/actix-demo actix-demo
ENTRYPOINT ["./actix-demo"]