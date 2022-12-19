FROM rust:1.66-alpine AS builder
WORKDIR /home/rust
RUN apk update && \
  apk add --no-cache musl-dev openssl-dev protobuf-dev protoc
COPY . .
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/local/cargo/bin/twiq-light /usr/local/bin/twiq-light
CMD ["twiq-light"]
