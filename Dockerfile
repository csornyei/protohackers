FROM rust:1-slim-buster as builder

ENV PORT 8080

RUN USER=root cargo new --bin smoke_test
WORKDIR /smoke_test

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/smoke_test*
RUN cargo build --release

FROM debian:buster-slim

ENV PORT 8080

COPY --from=builder /smoke_test/target/release/smoke_test /usr/local/bin/smoke_test

CMD ["smoke_test"]