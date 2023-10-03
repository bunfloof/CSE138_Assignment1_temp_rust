FROM rust:1.72 as builder
WORKDIR /usr/src/cse138
COPY . . 
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/cse138/target/release/pa1 /usr/local/bin/pa1
EXPOSE 8090
CMD ["pa1"]
