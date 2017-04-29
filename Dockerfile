FROM jimmycuadra/rust

WORKDIR /app
RUN mkdir src && touch src/main.rs
COPY rust_backend/Cargo.toml .
COPY rust_backend/Cargo.lock .

RUN cargo fetch

COPY rust_backend/ .

RUN cargo build --release && mv target/release/tekken_scorecard_backend . && rm -rf target

CMD /app/tekken_scorecard_backend
