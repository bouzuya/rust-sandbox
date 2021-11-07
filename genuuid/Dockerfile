FROM rust:1.56 as builder
WORKDIR /usr/src/genuuid
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src/
RUN echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/genuuid*
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
ENV PORT=8080
COPY --from=builder /usr/local/cargo/bin/genuuid /usr/local/bin/genuuid
ENTRYPOINT ["genuuid"]
