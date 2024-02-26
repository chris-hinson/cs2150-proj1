FROM rust:1.76

WORKDIR /usr/src/myapp
COPY . .
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler

RUN cargo build
CMD cargo run --bin driver
