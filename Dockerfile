FROM rust:1.80.0

WORKDIR /app

RUN rustup component add rustfmt && \
    cargo install cargo-make && \
    cargo install cargo-watch

COPY . .

CMD ["cargo", "make", "watch"]