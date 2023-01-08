FROM rust:1.66-bullseye as build
RUN apt install build-essential -y
WORKDIR /ipmon
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /ipmon/target/release/ipmon .
ENTRYPOINT ["./ipmon"]
