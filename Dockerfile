FROM alpine:latest

RUN apk add --no-cache curl nodejs npm gcc g++ make python3

ENV SSL_VERSION=1.0.2k

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH

RUN npm install -g neon-cli

WORKDIR /app