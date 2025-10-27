# BUILD
FROM rust:1.89.0 AS builder
WORKDIR /app
ADD . /app
RUN cargo build --release

# PROD
EXPOSE 1971
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=1971
FROM gcr.io/distroless/cc
COPY --from=builder /app/static /static
COPY --from=builder /app/target/release/main /
CMD ["./main"]
