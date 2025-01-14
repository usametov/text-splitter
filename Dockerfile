#FROM rust:1.76-slim-buster as build-env
FROM rust:1 as build-env

COPY ./ ./
RUN rm -rf data/output
RUN mkdir -p data/output

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y pkg-config libssl-dev g++ && rm -rf /var/lib/apt/lists/*
# Build 
RUN cargo build --release 

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/text-splitter /text-splitter

# Run the binary
CMD ["/text-splitter"]





