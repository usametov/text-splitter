FROM rust:1.76-slim-buster

COPY ./ ./

# Build 
RUN cargo build --release & rm src/*.rs
RUN mkdir -p data/output
# Run the binary
CMD ["./target/release/text-splitter"] --input-files data/files2process.txt --dir  ./data -o ./data/output --minchar 200 --maxchar 500 -v
