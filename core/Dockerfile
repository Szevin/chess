FROM rust

WORKDIR /app

COPY . .

RUN cargo install wasm-pack

CMD ["wasm-pack", "build", "--target", "web", "--out-dir", "chess-core"]
