FROM rust:1.88 AS dev

WORKDIR /app

# Instala cargo-watch
RUN cargo install cargo-watch

COPY . .

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]