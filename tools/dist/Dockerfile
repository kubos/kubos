# kubos/kubos-dev:1.23.0

FROM ubuntu:22.04

MAINTAINER marshall@xplore.com

RUN apt-get update -y
RUN apt-get install --no-install-recommends -y \
    software-properties-common

RUN add-apt-repository 'deb http://archive.ubuntu.com/ubuntu xenial main universe' && apt-key update

RUN apt-get update -y
RUN apt-get install --no-install-recommends -y \
    bc \
    build-essential \
    cmake \
    cpio \
    curl \
    doxygen \
    libboost-program-options1.58.0 \
    libboost-system1.58.0 \
    libsqlite3-dev \
    libssl-dev \
    ncurses-dev \
    file \
    git \
    graphviz \
    pkg-config \
    plantuml \
    python3 \
    python3-pip \
    python3-setuptools \
    rsync \
    sqlite3 \
    ssh \
    unzip \
    wget

# Custom gcc toolchains
RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/iobc_toolchain.tar.gz && \
    tar -xf ./iobc_toolchain.tar.gz -C /usr/bin && \
    rm ./iobc_toolchain.tar.gz

RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/bbb_toolchain.tar.gz && \
    tar -xf ./bbb_toolchain.tar.gz -C /usr/bin && \
    rm ./bbb_toolchain.tar.gz

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin:/usr/bin/bbb_toolchain/usr/bin"

# Install all Kubos Python dependencies
RUN python3 -m pip install wheel poetry==1.2.0

WORKDIR /kubos-py
COPY apis /kubos-py/apis
COPY libs /kubos-py/libs
COPY services /kubos-py/services
RUN ls -la /kubos-py

WORKDIR /kubos-py/libs/kubos
RUN poetry config virtualenvs.create false
RUN poetry install --no-interaction --no-ansi

WORKDIR /root
RUN rm -rf /kubos-py

# Setup rust targets and build tools
ENV PATH "$PATH:/root/.cargo/bin"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
RUN rustup default 1.64.0 && rm -rf /root/.rustup/toolchains/*/share/doc
RUN rustup target install arm-unknown-linux-gnueabihf
RUN rustup target install armv5te-unknown-linux-gnueabi
RUN rustup target install aarch64-unknown-linux-gnu
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN /root/.cargo/bin/cargo install --git https://github.com/kubos/cargo-kubos
RUN /root/.cargo/bin/cargo install cross --git https://github.com/cross-rs/cross
COPY tools/dist/cargo_config /root/.cargo/config

# Install NOS3 dependencies
ENV NOS3_PKGS "https://github.com/nasa/nos3/raw/59568804a8271672a53ae7ea09c4dae76dad3ba3/support/packages/ubuntu"
ENV NOS3_ITC_COMMON "itc-common-cxx11-Release_1.9.1_amd64.deb"
ENV NOS3_NOS_ENGINE "nos-engine-cxx11-Release_1.4.0_amd64.deb"

RUN wget $NOS3_PKGS/$NOS3_ITC_COMMON && \
    apt-get install -y ./$NOS3_ITC_COMMON && \
    rm $NOS3_ITC_COMMON
RUN wget $NOS3_PKGS/$NOS3_NOS_ENGINE && \
    apt-get install -y ./$NOS3_NOS_ENGINE && \
    rm $NOS3_NOS_ENGINE

# Docker engine for cross-rs builds
RUN wget https://get.docker.com -O get-docker.sh && \
    chmod 755 get-docker.sh && \
    ./get-docker.sh && \
    rm get-docker.sh
