FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/amtrack_rs /usr/local/bin/amtrack_rs
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 1972
CMD ["amtrack_rs"]
