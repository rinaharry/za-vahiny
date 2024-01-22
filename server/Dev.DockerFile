FROM rust:1.75
# Install dependencies
RUN apt-get update && apt-get install -y libpq5 libpq-dev libssl-dev pkg-config
WORKDIR /app
COPY . .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]