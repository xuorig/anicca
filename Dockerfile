FROM rust:1.53
LABEL org.opencontainers.image.source https://github.com/xuorig/anicca
WORKDIR cli
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/cli"]
