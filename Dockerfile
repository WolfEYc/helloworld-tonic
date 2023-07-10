FROM rust:latest

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler

WORKDIR /usr/src/helloworld-tonic
COPY . .

RUN cargo install --bin helloworld-server --path .

EXPOSE 50051


CMD ["helloworld-server"]