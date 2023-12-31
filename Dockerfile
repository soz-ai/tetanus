FROM rust

COPY . /app
WORKDIR /app

RUN RUSTFLAGS="-C opt-level=3" cargo build --release
CMD ["/app/target/release/tetanus"]
