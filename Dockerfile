FROM rust:1.76-slim-buster

COPY ./ ./

# Build 
RUN cargo build --release & rm src/*.rs
RUN mkdir -p data/output
# Run the binary
ENTRYPOINT ./target/release/text-splitter




