# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

COPY . . 

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/ .

ENTRYPOINT ["/core"]
EXPOSE 3000