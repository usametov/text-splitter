FROM rust:1.76-slim-buster

COPY ./ ./
RUN rm -rf data/output
RUN mkdir -p data/output

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y pkg-config libssl-dev g++ && rm -rf /var/lib/apt/lists/*
# Build 
RUN cargo build --release 
# Run the binary
CMD ["./target/release/text-splitter"]





