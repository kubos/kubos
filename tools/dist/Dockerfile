FROM phusion/baseimage:0.9.18

MAINTAINER kyle@kubos.co

RUN add-apt-repository -y ppa:team-gcc-arm-embedded/ppa
RUN add-apt-repository -y ppa:george-edison55/cmake-3.x
RUN add-apt-repository -y ppa:git-core/ppa
RUN add-apt-repository -y ppa:fkrull/deadsnakes-python2.7

RUN apt-get update -y

RUN apt-get upgrade -y python2.7
RUN apt-get install -y build-essential libssl-dev libffi-dev libhidapi-hidraw0 clang
RUN apt-get install -y python-setuptools build-essential ninja-build python-dev libffi-dev libssl-dev
RUN apt-get install -y gcc-arm-embedded
RUN apt-get install -y git
RUN apt-get install -y cmake
RUN apt-get install -y gcc-msp430 gdb-msp430 msp430-libc
RUN apt-get install -y libdbus-1-dev
RUN apt-get install -y unzip wget

#do the pip setup and installation things
RUN easy_install pip
RUN pip install --upgrade pip

#KubOS Linux setup
RUN echo "Installing KubOS Linux Toolchain"

RUN apt-get install -y minicom
RUN apt-get install -y libc6-i386 lib32stdc++6 lib32z1

RUN wget http://portal.kubos.co/bin/iobc_toolchain.tar.gz
RUN tar -xf ./iobc_toolchain.tar.gz -C /usr/bin
RUN rm ./iobc_toolchain.tar.gz

RUN pip install pysocks
RUN pip install mock
RUN pip install --upgrade setuptools
RUN pip install git+https://github.com/kubostech/kubos-cli

RUN mkdir -p /usr/local/lib/yotta_modules
RUN mkdir -p /usr/local/lib/yotta_targets
RUN mkdir -p /home/vagrant/.kubos

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin"
