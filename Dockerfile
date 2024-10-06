FROM rust:1.81 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
FROM debian:buster-slim as runner
# Install OpenSSL and its dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/amtrack_rs /usr/local/bin/amtrack_rs
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 1972
CMD ["amtrack_rs"]
