FROM rust:1.66 AS builder
WORKDIR /home/rust
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
COPY . .
ARG BASE_PATH=/
RUN cat /home/rust/nginx.conf | sed "s#\$BASE_PATH#${BASE_PATH}#" > /home/rust/nginx-build-path.conf
RUN trunk build --public-url "${BASE_PATH}"

FROM nginx:1.23.3
COPY --from=builder /home/rust/nginx-build-path.conf /etc/nginx/conf.d/configfile.template
COPY --from=builder /home/rust/dist /usr/share/nginx/html
ENV PORT 8080
ENV HOST 0.0.0.0
EXPOSE 8080
CMD sh -c "cat /etc/nginx/conf.d/configfile.template | envsubst '\$PORT' > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"

