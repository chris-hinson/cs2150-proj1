FROM rust:1.76

WORKDIR /usr/src/myapp
COPY . .

RUN cargo run --bin driver

