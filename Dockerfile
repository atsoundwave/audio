FROM rust:latest

WORKDIR /user/src/audio
COPY . .
EXPOSE 80

RUN cargo build --release

CMD ["./target/release/audio"]
