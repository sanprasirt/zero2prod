# Use the latest Rust stable release as base image
FROM rust:1.95.0 AS builder
# Let's switch our working directory tp `app` (equivalent to `cd app`)
# The `app` folder will be create for us by Docker
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Install sqlx-cli so the builder stage can be reused as a migration runner
# RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy all files from our working directory to Docker image
COPY . .
# Force SQLx to use the local metadata cache
ENV SQLX_OFFLINE=true
# Let's build binary!
RUN cargo build --release

# runtime stage
# FROM rust:1.95.0-slim AS runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT=production
# When docker run is executed, launch the application
ENTRYPOINT ["./zero2prod"]