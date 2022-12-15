FROM rust:1.65.0-slim-bullseye

RUN groupadd --gid 1000 ferris && \
    useradd -s /bin/bash --uid 1000 --gid 1000 -m ferris

RUN apt update && \
    apt install -y \
        curl \
        git \
        sudo
RUN echo 'ferris ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | bash
# formatter
RUN rustup component add rustfmt
