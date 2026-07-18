# ---- Build stage ----
FROM rust:1-slim AS builder
WORKDIR /app

# SQLite dev headers for libsqlite3-sys (system linking).
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies first.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build the real sources (migrations are embedded via sqlx::migrate!).
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/tricoach /usr/local/bin/tricoach
# Persist the SQLite database under /data (mount a volume here).
ENV BIND_ADDR=0.0.0.0:8080 \
    DATABASE_URL=sqlite:///data/tricoach.db
RUN mkdir -p /data
EXPOSE 8080
CMD ["tricoach"]
