

FROM rust:1.92.0 AS builder
WORKDIR /usr/src/recipe-management
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/recipe-management /usr/local/bin/recipe-management
CMD ["recipe-management"]

