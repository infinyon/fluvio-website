FROM node:18
WORKDIR /root
#COPY node_modules/ ./node_modules/
COPY *.ts ./
COPY *.json ./
RUN npm init -y && \
    npm install -D typescript ts-node @types/node && \
    npm install -S @fluvio/client
CMD npx ts-node example.ts
