FROM ubuntu:latest

ENV LANG en_US.UTF-8  
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8

RUN apt-get update && apt upgrade -y && apt-get install -y \
    zsh neovim ranger \
    language-pack-en-base \
    curl wget iproute2 \
    git openssh-client \
    build-essential pkg-config make cmake libssh-dev \
    nodejs npm \
    && rm -rf /var/lib/apt/lists/* 

#COPY Core Core
WORKDIR /root
COPY rust_setup.zsh rust_setup.zsh
RUN ["./rust_setup.zsh"]
COPY image_setup.zsh image_setup.zsh
RUN ["./image_setup.zsh"]
COPY container_setup.zsh container_setup.zsh

SHELL ["zsh"]
