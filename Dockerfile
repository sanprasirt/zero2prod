# Use the latest Rust stable release as base image
FROM rust:1.95.0
# Let's switch our working directory tp `app` (equivalent to `cd app`)
# The `app` folder will be create for us by Docker
WORKDIR /app

# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working directory to Docker image
COPY . .
# Force SQLx to use the local metadata cache
ENV SQLX_OFFLINE=true
# Let's build binary!
RUN cargo build --release

# When docker run is executed, launch the application
ENTRYPOINT ["./target/release/zero2prod"]