FROM rust:1.66-bullseye as build
RUN apt-get install g++-aarch64-linux-gnu libc6-dev-arm64-cross -y
WORKDIR /ipmon
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /ipmon/target/release/ipmon .
ENTRYPOINT ["./ipmon"]
