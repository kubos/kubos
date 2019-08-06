# kubos/kubos-dev:1.16.0

FROM ubuntu:18.04

MAINTAINER catherine@kubos.com, ryan@kubos.com

RUN apt-get update -y

RUN apt-get upgrade --no-install-recommends -y python3.5
RUN apt-get install --no-install-recommends -y pkg-config build-essential git cmake unzip wget sqlite3 libsqlite3-dev libssl-dev curl git ssh

# Linux build dependencies
RUN apt-get install --no-install-recommends -y bc cpio ncurses-dev libc6-i386 lib32stdc++6 lib32z1

#Tools to generate docs
RUN apt-get install --no-install-recommends -y doxygen graphviz plantuml

# Install pip for Python2 and Python3
RUN apt-get install --no-install-recommends -y python3-pip python3-setuptools

# So that we have bdist_wheel available when installing other packages
RUN pip3 install wheel

#Kubos Linux setup
RUN echo "Installing Kubos Linux Toolchain"

RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/iobc_toolchain.tar.gz
RUN tar -xf ./iobc_toolchain.tar.gz -C /usr/bin
RUN rm ./iobc_toolchain.tar.gz

RUN wget https://s3.amazonaws.com/kubos-world-readable-assets/bbb_toolchain.tar.gz
RUN tar -xf ./bbb_toolchain.tar.gz -C /usr/bin
RUN rm ./bbb_toolchain.tar.gz

# Install all Kubos Python dependencies
RUN git clone https://github.com/kubos/kubos --depth 1
RUN pip3 install -r kubos/requirements.txt
RUN rm -r kubos

# Setup rust stuff
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH "$PATH:/root/.cargo/bin"
RUN rustup toolchain uninstall stable-x86_64-unknown-linux-gnu
RUN rustup default 1.32.0
RUN rustup target install arm-unknown-linux-gnueabihf
RUN rustup target install armv5te-unknown-linux-gnueabi
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN /root/.cargo/bin/cargo install --git https://github.com/kubos/cargo-kubos
COPY cargo_config /root/.cargo/config

# Install NOS3 dependencies
RUN apt-get --no-install-recommends install -y software-properties-common
RUN add-apt-repository 'deb http://cz.archive.ubuntu.com/ubuntu xenial main universe'
RUN apt-get install -y libboost-system1.58.0 libboost-program-options1.58.0 libxerces-c3.1
RUN wget https://github.com/nasa/nos3/raw/master/support/packages/ubuntu/itc-common-cxx11-Release_1.9.1_amd64.deb
RUN apt-get install -y ./itc-common-cxx11-Release_1.9.1_amd64.deb
RUN rm itc-common-cxx11-Release_1.9.1_amd64.deb

RUN wget https://github.com/nasa/nos3/raw/master/support/packages/ubuntu/nos-engine-cxx11-Release_1.4.0_amd64.deb
RUN apt-get install -y ./nos-engine-cxx11-Release_1.4.0_amd64.deb
RUN rm nos-engine-cxx11-Release_1.4.0_amd64.deb

RUN apt-get -y clean

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin:/usr/bin/bbb_toolchain/usr/bin"
