FROM rust:1.76-slim-buster

COPY ./ ./

# Build 
RUN cargo build --release & rm src/*.rs

# Run the binary
# CMD ["./target/release/text-splitter"] --input-files /media/user777/ssd/aiken/files2process.txt --dir  /media/user777/ssd/aiken/aiken-lang-docs/src/ -o /media/user777/ssd/aiken/aiken-docs-processed/ --minchar 200 --maxchar 700 -v
