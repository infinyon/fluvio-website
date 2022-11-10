FROM rust:1.65.0

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio 

# Copy your Rust project and run it
COPY Cargo.toml .
COPY src ./src
CMD /usr/local/cargo/bin/cargo run