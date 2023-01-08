FROM rust:1.66 as build
RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
WORKDIR /ipmon
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update && apt -y install ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /ipmon/target/release/ipmon ./
ENTRYPOINT ["/app/ipmon"]
