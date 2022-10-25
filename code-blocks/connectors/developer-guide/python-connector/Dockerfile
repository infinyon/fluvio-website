# Dockerfile for Python cat-facts connector
FROM python

# Copy our python script into the connector image
COPY get-cat-facts.py /usr/local/sbin/get-cat-facts.py
RUN chmod +x /usr/local/sbin/get-cat-facts.py

# This is required to connect to a cluster
# Connectors run as the `fluvio` user
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER

# Install Rust development environment
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/fluvio/.cargo/bin:${PATH}"
RUN pip install --upgrade pip

# Install dependencies
RUN pip3 install fluvio requests

# Start script on start
ENTRYPOINT get-cat-facts.py