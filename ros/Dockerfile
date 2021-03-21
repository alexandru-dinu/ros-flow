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
        vim && \
    apt-get clean

# ros
RUN apt-get install -y python3-rosmsg

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

CMD /bin/bash

# Build application
# TODO

