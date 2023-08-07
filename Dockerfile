FROM rust:1.67 as builder
RUN apt-get update; apt-get --no-install-recommends install -y libsdl2-dev
WORKDIR /server
COPY server .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update; apt-get --no-install-recommends install -y libsdl2-dev
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY maps maps
EXPOSE 8888
CMD server