# ---- Build stage ----
FROM rust:1-slim AS builder
WORKDIR /app

# Cache dependencies first.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build the real sources (migrations are embedded via sqlx::migrate!).
COPY migrations ./migrations
COPY src ./src
# Bust the dependency-cache layer: freshen source mtimes and drop the placeholder
# binary so cargo always relinks the real application (not the `fn main(){}` stub).
RUN find src -name '*.rs' -exec touch {} + \
    && rm -f target/release/tricoach target/release/deps/tricoach-* \
    && cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/tricoach /usr/local/bin/tricoach
ENV BIND_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["tricoach"]
