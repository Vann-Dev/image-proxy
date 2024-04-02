FROM rust:1.77.1 as build

WORKDIR /image-proxy

COPY . .

RUN cargo build --release

FROM rust:1.77.1

COPY --from=build /image-proxy/target/release/image-proxy .

CMD ["./image-proxy"]
