FROM node:7.9

WORKDIR /install

RUN curl https://sh.rustup.rs > install_rust.sh && sh install_rust.sh -y
ENV PATH="$PATH:/root/.cargo/bin"

WORKDIR /app
COPY . .

WORKDIR /app/rust_backend
RUN cargo build --release && mv target/release/tekken_scorecard_backend . && rm -rf target && rm -rf /root/.cargo

CMD /app/rust_backend/tekken_scorecard_backend
