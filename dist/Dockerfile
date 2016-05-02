FROM phusion/baseimage:0.9.18
MAINTAINER marshall@kubos.co

RUN mkdir /build

RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa
RUN add-apt-repository ppa:george-edison55/cmake-3.x
RUN apt-get update
RUN apt-get install -y python-setuptools build-essential ninja-build python-dev libffi-dev libssl-dev gcc-arm-embedded git
RUN apt-get install -y cmake=3.2.2-2~ubuntu14.04.1~ppa1
RUN easy_install pip
RUN pip install yotta

RUN mkdir -p /usr/local/lib/yotta_modules
RUN chmod 755 /usr/local/lib/yotta_modules

RUN apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

VOLUME /build
WORKDIR /build
