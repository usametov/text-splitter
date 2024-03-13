FROM rust:1.76-slim-buster

COPY ./ ./

# Build 
RUN cargo build --release & rm src/*.rs

# Run the binary
# CMD ["./target/release/text-splitter"] --input-files /data/files2process.txt --dir  /data/aiken/aiken-lang-docs/src/ -o /data/aiken/aiken-docs-processed/ --minchar 200 --maxchar 700 -v
