FROM rust

WORKDIR /app

RUN cargo install wasm-pack

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli

COPY . .

CMD ["wasm-pack", "build", "--out-name", "index", "--out-dir", "chess-core"]
