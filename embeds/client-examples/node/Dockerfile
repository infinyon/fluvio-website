FROM node:18

# Run as the `fluvio` user instead of root
ENV USER=fluvio
RUN useradd --create-home "$USER"
USER $USER
WORKDIR /home/fluvio 

# Copy your Node project
COPY --chown=$USER:$USER *.ts ./
COPY --chown=$USER:$USER *.json ./

# Install dependencies
RUN mkdir -p ~/.local/bin && \
    npm config set prefix '~/.local/' && \
    npm init -y && \
    npm install -D typescript ts-node @types/node && \
    npm install -S @fluvio/client

# Run your Node project
CMD npx ts-node example.ts