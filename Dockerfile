FROM rust:1.60.0

# Preparing build dependencies for yew.
# See yew tutorial: https://yew.rs/ja/docs/tutorial
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . /app
# Build client app.
RUN trunk build --release

# Build server app.
RUN cargo build --release

RUN ls

ENTRYPOINT ["/app/target/release/discover-hollywood"]
