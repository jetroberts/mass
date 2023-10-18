FROM rust:1.73 AS builder

WORKDIR /usr/src/server

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

CMD ["server"]
