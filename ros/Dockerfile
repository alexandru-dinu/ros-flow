FROM ros:foxy

ENV TERM=xterm-256color

WORKDIR ws

# generic
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        apt-utils \
        curl \
        git \
        ssh \
        libssl-dev \
        pkg-config \
        vim \
        wget && \
    apt-get clean

# ros
RUN apt-get install -y python3-rosmsg

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/bash -s -- --no-modify-path -y

ENV PATH=$PATH:/root/.cargo/bin

# Create empty project for building dependencies
RUN cargo new --bin project

WORKDIR ./project

# Copy manifest files
COPY ./Cargo.* ./

# Build dependencies and remove dummy main
RUN cargo build && rm -rf ./src/*

# Copy source code
COPY ./src/* ./src/

# Build application
RUN cargo build

# Entry point
CMD cargo run

