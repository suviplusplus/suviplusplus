FROM rust:latest AS builder

WORKDIR /usr/src/backend
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/backend/target/release/backend .
EXPOSE 8080

CMD ["./backend"]
