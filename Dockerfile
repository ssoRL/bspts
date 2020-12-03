FROM rust:latest as builder
# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /usr/src/bspts
COPY ./data/ ./data/
COPY ./backend/ ./backend/
COPY ./frontend/ ./frontend/
COPY Cargo.toml Cargo.toml
# Build the data types
RUN cargo build --release -p data
# Build the frontend
RUN wasm-pack build frontend --release --target web --out-name bspts --out-dir ../wasm_scripts
# Build the backend
RUN cargo build --release -p backend

FROM debian:buster-slim
# Download a few runtime dependencies
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/bspts/target/release/backend .
COPY ./site/index.html ./site/index.html
COPY ./site/index.js ./site/index.js
COPY ./site/assets ./site/assets
COPY --from=builder /usr/src/bspts/wasm_scripts ./site/scripts
COPY compose.env .env
ENTRYPOINT ["./backend"]