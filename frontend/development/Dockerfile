FROM ubuntu:latest

ENV LANG en_US.UTF-8  
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8

RUN apt-get update && apt upgrade -y && apt-get install -y \
    zsh neovim ranger \
    language-pack-en-base \
    curl wget iproute2 \
    git openssh-client \
    build-essential pkg-config make cmake libssh-dev\
    && rm -rf /var/lib/apt/lists/* 

#COPY Core Core
WORKDIR /root
COPY image_setup.sh image_setup.sh
RUN ["./image_setup.sh"]
COPY container_setup.sh container_setup.sh

SHELL ["zsh"]
EXPOSE 16
