FROM rust:1.94 AS builder
WORKDIR /app
ARG BUILD_CONFIG=release
ENV SQLX_OFFLINE=true
COPY Cargo.toml Cargo.lock ./
COPY konslo-core/Cargo.toml ./konslo-core/
COPY konslo-api/Cargo.toml ./konslo-api/
COPY konslo-core/.sqlx ./konslo-core/.sqlx
COPY konslo-core/queries ./konslo-core/queries
COPY konslo-core/migrations ./konslo-core/migrations
RUN mkdir konslo-core/src && echo "" > konslo-core/src/lib.rs
RUN mkdir konslo-api/src && echo "fn main() {}" > konslo-api/src/main.rs
RUN if [ "$BUILD_CONFIG" = "release" ]; then cargo build --release; else cargo build; fi
COPY konslo-core/src konslo-core/src
COPY konslo-api/src konslo-api/src
RUN touch konslo-core/src/lib.rs
RUN touch konslo-api/src/main.rs
RUN if [ "$BUILD_CONFIG" = "release" ]; then cargo build --release; else cargo build; fi

FROM debian:bookworm-slim as runtime
WORKDIR /app
ARG BUILD_CONFIG=release
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/${BUILD_CONFIG}/konslo-api .
CMD ["/app/konslo-api"]

