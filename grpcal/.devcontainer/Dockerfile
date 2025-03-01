FROM rust:1.85.0-bookworm
RUN apt update && apt install -y protobuf-compiler
RUN useradd --create-home bouzuya \
  && mkdir -p /home/bouzuya/app
USER bouzuya