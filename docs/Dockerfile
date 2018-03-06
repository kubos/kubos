# kubos/kubos-docs:0.1.0

FROM phusion/baseimage:0.9.22

MAINTAINER ryan@kubos.co

RUN apt-get update -y

RUN apt-get upgrade -y python2.7
RUN apt-get install -y python-setuptools build-essential ninja-build python-dev git

#do the pip setup and installation things
RUN easy_install pip
RUN pip install --upgrade pip

#Tools to generate docs
RUN apt-get install -y doxygen graphviz plantuml
RUN pip install Sphinx==1.5.6
RUN pip install breathe
RUN pip install sphinx-rtd-theme==0.2.4
RUN pip install sphinxcontrib-plantuml sphinxcontrib-versioning
RUN pip install sphinx-jsondomain
