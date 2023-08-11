FROM rust:1.67 as builder
WORKDIR /server
COPY server .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY maps maps
EXPOSE 8888
CMD server