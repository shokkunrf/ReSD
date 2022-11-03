FROM node:16.16.0-bullseye-slim

RUN apt update && \
    apt install -y \
        sudo
RUN echo 'node ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY ./entrypoint.sh /entrypoint
RUN chmod +x /entrypoint
ENTRYPOINT [ "/entrypoint" ]

USER node
