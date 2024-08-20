FROM rust:1.80.1-slim-bullseye

# ARG APP_NAME=

WORKDIR /app

COPY . .

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

RUN cargo build --release

# RUN cp ./target/release/$APP_NAME /bin/server

CMD ["./target/release/service-health-checker"]
