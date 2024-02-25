FROM rust

WORKDIR /app

RUN cargo install wasm-pack

COPY . .

CMD ["wasm-pack", "build", "--target", "web", "--out-dir", "chess-core"]
