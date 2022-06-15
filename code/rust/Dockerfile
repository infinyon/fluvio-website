# This image is for CI testing purposes only
FROM rust:1.61
WORKDIR /root
COPY Cargo.toml .
COPY src ./src
RUN /usr/local/cargo/bin/cargo build
CMD /usr/local/cargo/bin/cargo run