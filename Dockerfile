FROM jimmycuadra/rust:1.17.0

# Melbourne timezone
ENV TZ=Australia/Melbourne
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get install libpq-dev -y

WORKDIR /app
RUN mkdir src && touch src/main.rs
COPY rust_backend/Cargo.toml .
COPY rust_backend/Cargo.lock .

RUN cargo fetch

COPY rust_backend/ .

RUN cargo build --release && mv target/release/tekken_scorecard_backend . && rm -rf target

CMD /app/tekken_scorecard_backend
