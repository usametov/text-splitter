FROM rust:1.76-slim-buster

COPY ./ ./

# Build 
RUN cargo build --release & rm src/*.rs

# Run the binary
# CMD ["./target/release/text-splitter"] --input-files /media/user777/ssd/aiken/files2process.txt --dir  /media/p00la-by/ssd1/aiken/aiken-lang-docs/src/ -o /media/p00la-by/ssd1/aiken/aiken-docs-processed/ --minchar 200 --maxchar 700 -v