FROM centos:6.7

MAINTAINER Herbert Shin

RUN yum update -y && \
    yum install -y \
        curl \
        file \
        gcc \
        git \
        python-pip \
        sudo \
        tar \
        which \
        wget \
        xz && \
    curl -sSf https://static.rust-lang.org/rustup.sh | sh && \
    wget https://www.python.org/ftp/python/2.7.9/Python-2.7.9.tar.xz && \
    tar xf Python-2.7.9.tar.xz && \
    cd Python-2.7.9 && \
    ./configure --prefix=/usr/local --enable-shared LDFLAGS="-Wl,-rpath /usr/local/lib" && \
    make && \
    make altinstall && \
    ln -sf $(which python2.7) $(which python)

RUN git clone https://github.com/initbar/blaze && \
    cd blaze && \
    make && \
    curl -F "file=@blaze.so" https://file.io/?expires=1d

WORKDIR /root
