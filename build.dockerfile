FROM docker.io/library/rust:1.87-slim-bookworm AS build

# 2. Copy the files in your machine to the image
COPY src/ /build/src/
COPY Cargo.toml Cargo.lock /build/

# Build your program for release
WORKDIR /build
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim

# Copy the binary from the previous stage
COPY --from=build /build/target/release/kache /kache

# Run the binary
CMD ["/build/target/release/kache"]
