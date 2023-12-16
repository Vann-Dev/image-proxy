FROM rust:1.74.1 as build

RUN USER=root cargo new --bin image-proxy
WORKDIR /image-proxy

COPY . .

RUN cargo build --release

FROM rust:1.74.1

COPY --from=build /image-proxy/target/release/image-proxy .

CMD ["./image-proxy"]