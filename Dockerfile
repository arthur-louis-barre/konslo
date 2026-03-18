FROM rust:latest AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/konslo-api .
CMD ["/app/konslo-api"]