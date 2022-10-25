# This image is for CI testing purposes only
FROM python:3

# This is required to connect to a cluster
# Connectors run as the `fluvio` user
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER

# Installing Fluvio CLI for creating topics in example
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | bash

# Install Rust development environment
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add `fluvio` and `cargo` to PATH
ENV PATH="/home/fluvio/.fluvio/bin:/home/fluvio/.cargo/bin:${PATH}"

# Install dependencies
RUN pip install --upgrade pip
RUN pip3 install fluvio

COPY hello-python.py .
CMD python hello-python.py