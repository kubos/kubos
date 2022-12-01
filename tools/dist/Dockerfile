# kubos/kubos-dev:1.22.0

FROM ubuntu:22.04

MAINTAINER marshall@xplore.com

RUN apt-get update -y

RUN apt-get upgrade --no-install-recommends -y python3
RUN apt-get install --no-install-recommends -y pkg-config build-essential git cmake unzip wget sqlite3 libsqlite3-dev libssl-dev curl git ssh

# Linux build dependencies
RUN apt-get install --no-install-recommends -y file rsync bc cpio ncurses-dev 

#Tools to generate docs
RUN apt-get install --no-install-recommends -y doxygen graphviz plantuml

# Install pip for Python2 and Python3
RUN apt-get install --no-install-recommends -y python3-pip python3-setuptools

# So that we have bdist_wheel available when installing other packages
RUN python3 -m pip install wheel poetry==1.2.0

#Kubos Linux setup
RUN echo "Installing Kubos Linux Toolchain"

RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/iobc_toolchain.tar.gz && tar -xf ./iobc_toolchain.tar.gz -C /usr/bin && rm ./iobc_toolchain.tar.gz

RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/bbb_toolchain.tar.gz && tar -xf ./bbb_toolchain.tar.gz -C /usr/bin && rm ./bbb_toolchain.tar.gz

# Install all Kubos Python dependencies
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

# Setup rust stuff
ENV PATH "$PATH:/root/.cargo/bin"
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
RUN rustup default 1.64.0 && rm -rf /root/.rustup/toolchains/*/share/doc
RUN rustup target install arm-unknown-linux-gnueabihf
RUN rustup target install armv5te-unknown-linux-gnueabi
RUN rustup target install aarch64-unknown-linux-gnu
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN /root/.cargo/bin/cargo install --git https://github.com/kubos/cargo-kubos
COPY tools/dist/cargo_config /root/.cargo/config

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin:/usr/bin/bbb_toolchain/usr/bin"

# Install NOS3 dependencies
RUN apt-get --no-install-recommends install -y software-properties-common
RUN add-apt-repository 'deb http://archive.ubuntu.com/ubuntu xenial main universe' && apt-key update
RUN apt-get update -y
RUN wget https://github.com/nasa/nos3/raw/59568804a8271672a53ae7ea09c4dae76dad3ba3/support/packages/ubuntu/itc-common-cxx11-Release_1.9.1_amd64.deb && \
    apt-get install -y ./itc-common-cxx11-Release_1.9.1_amd64.deb && \
    rm itc-common-cxx11-Release_1.9.1_amd64.deb
RUN wget https://github.com/nasa/nos3/raw/59568804a8271672a53ae7ea09c4dae76dad3ba3/support/packages/ubuntu/nos-engine-cxx11-Release_1.4.0_amd64.deb && \
    apt-get install -y ./nos-engine-cxx11-Release_1.4.0_amd64.deb && \
    rm nos-engine-cxx11-Release_1.4.0_amd64.deb
RUN apt-get install -y libboost-system1.58.0 libboost-program-options1.58.0 