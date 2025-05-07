FROM rust:1.86 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install pipx -y && rm -rf /var/lib/apt/lists/*
RUN pipx install apprise
# pipx bin bath
ENV PATH="$PATH:/root/.local/bin"
WORKDIR /
COPY --from=builder /build/target/release/ipmon .
ENTRYPOINT ["/ipmon"]
