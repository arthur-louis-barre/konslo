FROM rust:latest AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY Cargo.toml Cargo.lock ./
COPY konslo-core/Cargo.toml ./konslo-core/
COPY konslo-api/Cargo.toml ./konslo-api/
COPY .sqlx ./.sqlx
RUN mkdir konslo-core/src && echo "" > konslo-core/src/lib.rs
RUN mkdir konslo-api/src && echo "fn main() {}" > konslo-api/src/main.rs
RUN cargo build
COPY konslo-core/src konslo-core/src
COPY konslo-api/src konslo-api/src
RUN touch konslo-core/src/lib.rs
RUN touch konslo-api/src/main.rs
RUN cargo build

FROM debian:bookworm-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/debug/konslo-api .
CMD ["/app/konslo-api"]