FROM rust:1.66 AS builder
WORKDIR /home/rust
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN trunk build

FROM nginx:1.23.3
COPY nginx.conf /etc/nginx/conf.d/configfile.template
COPY --from=builder /home/rust/dist /usr/share/nginx/html
ENV BASE_PATH /
ENV PORT 8080
ENV HOST 0.0.0.0
EXPOSE 8080
CMD sh -c "cat /etc/nginx/conf.d/configfile.template | envsubst '\$PORT' | envsubst '\$BASE_PATH' > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"

