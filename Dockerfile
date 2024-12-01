# docker build -t tinyhttpd:0.2.0 .
# docker run -d -p 80:8000 --read-only -v your_htdocs:/htdocs --rm tinyhttpd:0.2.0

FROM rust:latest AS builder
WORKDIR /usr/src/tinyhttpd
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get upgrade -y \
 && rm -rf /var/lib/apt/lists/* /var/cache/apt/*
COPY --from=builder /usr/local/cargo/bin/tinyhttpd /usr/local/bin/tinyhttpd
WORKDIR /htdocs
EXPOSE 8000
CMD ["tinyhttpd"]
