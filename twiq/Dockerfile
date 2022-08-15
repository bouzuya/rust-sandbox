FROM ekidd/rust-musl-builder:stable AS builder
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN mkdir -p crates/domain/src/
RUN echo "fn main() {}" > ./crates/domain/src/main.rs
COPY ./crates/domain/Cargo.toml ./crates/domain/Cargo.toml
RUN mkdir -p crates/import_twitter_data/src/
RUN echo "fn main() {}" > ./crates/import_twitter_data/src/main.rs
COPY ./crates/import_twitter_data/Cargo.toml ./crates/import_twitter_data/Cargo.toml
RUN mkdir -p crates/twitter_user_id/src/
RUN echo "fn main() {}" > ./crates/twitter_user_id/src/main.rs
COPY ./crates/twitter_user_id/Cargo.toml ./crates/twitter_user_id/Cargo.toml
RUN mkdir -p crates/web/src/
RUN echo "fn main() {}" > ./crates/web/src/main.rs
COPY ./crates/web/Cargo.toml ./crates/web/Cargo.toml

RUN cargo build --release

RUN rm ./crates/domain/src/main.rs
RUN rm ./crates/import_twitter_data/src/main.rs
RUN rm ./crates/twitter_user_id/src/main.rs
RUN rm ./crates/web/src/main.rs

COPY . .
RUN cargo install --path ./crates/web

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/web /usr/local/bin/web
ENTRYPOINT ["web"]
