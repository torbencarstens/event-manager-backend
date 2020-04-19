FROM rust:1.42.0-slim-stretch

WORKDIR /var/app

ADD . .

RUN apt update
RUN apt install -y libpq-dev libssl-dev pkg-config

RUN rustup update nightly
RUN cargo -v search --limit 0

RUN cargo +nightly build --release
CMD ./target/release/event_manager
