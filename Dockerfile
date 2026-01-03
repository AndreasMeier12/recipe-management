FROM rust:1.92 as builder
ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=3000
WORKDIR /app
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-gnu
FROM ubuntu:latest
RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN apt-get install -y ca-certificates

WORKDIR /app
COPY --from=builder /app /app
ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=3000
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
CMD ["cargo","run","--release"]
