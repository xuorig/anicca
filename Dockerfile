FROM rust:1.53
WORKDIR cli
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/cli"]
