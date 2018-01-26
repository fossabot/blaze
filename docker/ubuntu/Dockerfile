FROM ubuntu:16.04

MAINTAINER Herbert Shin

RUN apt-get update -y && \
    apt-get install -y \
            apt-utils \
            cargo \
            curl \
            git \
            nproc \
            sudo

RUN git clone https://github.com/initbar/blaze && \
    cd blaze && \
    make && \
    curl -F "file=@blaze.so" https://file.io/?expires=1d

WORKDIR /root
