FROM rust

RUN apt-get update -y && apt-get install -y clang
