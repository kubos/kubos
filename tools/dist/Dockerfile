# kubos/kubos-dev:0.0.6

FROM phusion/baseimage:0.9.22

MAINTAINER kyle@kubos.co

RUN add-apt-repository -y ppa:team-gcc-arm-embedded/ppa
RUN add-apt-repository -y ppa:george-edison55/cmake-3.x
RUN add-apt-repository -y ppa:git-core/ppa

RUN apt-get update -y

RUN apt-get upgrade -y python2.7
RUN apt-get install -y build-essential libssl-dev libffi-dev libhidapi-hidraw0 clang
RUN apt-get install -y python-setuptools build-essential ninja-build python-dev libffi-dev libssl-dev
RUN apt-get install -y gcc-arm-embedded
RUN apt-get install -y git
RUN apt-get install -y cmake
RUN apt-get install -y gcc-msp430 gdb-msp430 msp430-libc
RUN apt-get install -y libdbus-1-dev dbus
RUN apt-get install -y unzip wget

# Legacy BBB toolchain
RUN apt-get install -y crossbuild-essential-armhf gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf

#do the pip setup and installation things
RUN easy_install pip
RUN pip install --upgrade pip

#KubOS Linux setup
RUN echo "Installing KubOS Linux Toolchain"

RUN apt-get install -y minicom
RUN apt-get install -y libc6-i386 lib32stdc++6 lib32z1

RUN wget https://s3.amazonaws.com/kubos-provisioning/iobc_toolchain.tar.gz
RUN tar -xf ./iobc_toolchain.tar.gz -C /usr/bin
RUN rm ./iobc_toolchain.tar.gz

RUN wget https://s3.amazonaws.com/kubos-provisioning/bbb_toolchain.tar.gz
RUN tar -xf ./bbb_toolchain.tar.gz -C /usr/bin
RUN rm ./bbb_toolchain.tar.gz

RUN pip install pysocks
RUN pip install mock
RUN pip install --upgrade setuptools
RUN pip install git+https://github.com/kubos/kubos-cli
RUN pip install cryptography==1.9

RUN mkdir -p /usr/local/lib/yotta_modules
RUN mkdir -p /usr/local/lib/yotta_targets
RUN mkdir -p /home/vagrant/.kubos

ENV PATH "$PATH:/usr/bin/iobc_toolchain/usr/bin:/usr/bin/bbb_toolchain/usr/bin"

# D-Bus setup/init stufff
# Create a D-bus init script and put the conf in place
RUN mkdir -p /etc/my_init.d
ADD dbus_environment.sh /etc/my_init.d/dbus_environment.sh
RUN chmod +x /etc/my_init.d/dbus_environment.sh
ADD kubos.conf /etc/dbus-1/kubos.conf

# Export neccesary env variables
ENV DBUS_SESSION_BUS_ADDRESS "unix:path=/tmp/kubos"
ENV DBUS_STARTER_BUS_TYPE "session"
