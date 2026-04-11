# Build: docker build -t lyangpiler .
# Run:   docker run --rm -v "$PWD:/work" -w /work lyangpiler run ./hello.nbh --vm
FROM rust:1-bookworm AS builder
WORKDIR /src
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/target/release/lyangpiler /usr/local/bin/lyangpiler
WORKDIR /work
ENTRYPOINT ["/usr/local/bin/lyangpiler"]
