FROM rust:latest as builder

WORKDIR /usr/src/rust-api

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian

COPY --from=builder /usr/src/rust-api/target/release/rust-api /usr/local/bin/rust-api

EXPOSE 8080

CMD ["rust-api"]