# Etapa de construcción
FROM rust:1.78 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

# Etapa de ejecución
FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/api_rust_graphql /usr/local/bin/api_rust_graphql

COPY .env.example .env

EXPOSE 8080

CMD ["api_rust_graphql"]
