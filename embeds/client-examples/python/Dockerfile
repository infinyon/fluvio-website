FROM python:3

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio 

# Copy your Rust project ng Fluvio CLI for creating topics in example
RUN curl -fsS https://packages.fluvio.io/v1/install.sh | bash

## Add `fluvio` to PATH
ENV PATH="/home/fluvio/.fluvio/bin:${PATH}"

# Install dependencies
RUN pip install --upgrade pip
RUN pip3 install fluvio

# Copy your Python project and run it
COPY --chown=$USER:$USER hello-python.py .
CMD python hello-python.py