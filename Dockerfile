FROM node:7.9

WORKDIR /install

RUN curl https://sh.rustup.rs > install_rust.sh && sh install_rust.sh -y
ENV PATH="$PATH:/root/.cargo/bin"

WORKDIR /app/frontend
ADD frontend/package.json package.json
ADD frontend/yarn.lock yarn.lock
RUN yarn

WORKDIR /app
COPY . .

WORKDIR /app/rust_backend
RUN cargo build --release && mv target/release/tekken_scorecard_backend . && rm -rf target && rm -rf /root/.cargo

WORKDIR /app
RUN (cd frontend && yarn run build)

CMD /app/rust_backend/tekken_scorecard_backend
EXPOSE 4000
ENV STATIC_DIR=/app/frontend/build
