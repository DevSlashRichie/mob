FROM rust as builder

WORKDIR /app/src/

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=builder /app/src/target/release/mob ./

CMD ["/app/mob"]
