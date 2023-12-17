FROM rust:1.74.1 as build

WORKDIR /image-proxy

COPY . .

RUN cargo build --release

FROM rust:1.74.1

COPY --from=build /image-proxy/target/release/image-proxy .

EXPOSE 3000

CMD ["./image-proxy"]