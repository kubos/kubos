# kubos/kubos-dev:0.1.6

FROM phusion/baseimage:0.9.22

MAINTAINER kyle@kubos.co

RUN add-apt-repository -y ppa:george-edison55/cmake-3.x

RUN apt-get update -y

RUN apt-get upgrade -y python2.7
RUN apt-get install -y build-essential libssl-dev libffi-dev libhidapi-hidraw0 clang
RUN apt-get install -y python-setuptools build-essential ninja-build python-dev libffi-dev libssl-dev
RUN apt-get install -y git
RUN apt-get install -y cmake
RUN apt-get install -y unzip wget
RUN apt-get install -y sqlite3 libsqlite3-dev

#do the pip setup and installation things
RUN easy_install pip
# Need to install pip<v10 due to this issue: https://github.com/ARMmbed/yotta/issues/835
# Forcibly controlling version until this is resolved
RUN pip install pip==9.0.3

#Kubos Linux setup
RUN echo "Installing Kubos Linux Toolchain"

RUN apt-get install -y minicom
RUN apt-get install -y libc6-i386 lib32stdc++6 lib32z1

RUN wget https://s3.amazonaws.com/kubos-provisioning/iobc_toolchain.tar.gz
RUN tar -xf ./iobc_toolchain.tar.gz -C /usr/bin
RUN rm ./iobc_toolchain.tar.gz

RUN wget https://s3.amazonaws.com/kubos-provisioning/bbb_toolchain.tar.gz
RUN tar -xf ./bbb_toolchain.tar.gz -C /usr/bin
RUN rm ./bbb_toolchain.tar.gz

RUN pip install --upgrade setuptools
RUN pip install -r https://raw.githubusercontent.com/kubos/kubos-cli/master/requirements.txt
RUN pip install git+https://github.com/kubos/kubos-cli

RUN mkdir -p /usr/local/lib/yotta_modules
RUN mkdir -p /usr/local/lib/yotta_targets
RUN mkdir -p /home/vagrant/.kubos

# Setup rust stuff
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH "$PATH:/root/.cargo/bin"
RUN rustup target install arm-unknown-linux-gnueabihf
RUN rustup target install armv5te-unknown-linux-gnueabi
RUN /root/.cargo/bin/cargo install --git https://github.com/kubos/cargo-kubos
COPY cargo_config /root/.cargo/config

#Tools to generate docs
RUN apt-get install -y doxygen graphviz plantuml
RUN pip install Sphinx==1.5.6
RUN pip install breathe
RUN pip install sphinx-rtd-theme==0.2.4
RUN pip install sphinxcontrib-plantuml sphinxcontrib-versioning
RUN pip install sphinx-jsondomain

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin:/usr/bin/bbb_toolchain/usr/bin"
