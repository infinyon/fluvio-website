FROM rust:1.78

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio

# Copy your Rust project and run it
COPY --chown=$USER:$USER Cargo.toml .
COPY --chown=$USER:$USER src ./src
CMD /usr/local/cargo/bin/cargo run