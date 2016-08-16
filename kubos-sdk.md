# KubOS-SDK

## Prerequisites

### Install docker

If you don't already have docker installed see the docker downloads for [Mac OS X](https://www.docker.com/products/docker-toolbox) or the installation docs for [Linux](https://docs.docker.com/engine/installation/)

		$ docker --version

The Kubos-SDK has been tested on Docker version 1.11.1.

### Install pip

#### Linux
##### Ubuntu/Debian

		$ sudo apt-get install python-pip

##### Fedora

		$ sudo yum upgrade python-setuptools
		$ sudo yum install python-pip python-wheel


Other Linux Dristibutions see the  [pip installation guide](http://python-packaging-user-guide.readthedocs.io/en/latest/install_requirements_linux/)

##### Mac OS X

Using easy_install:

		$ sudo easy_install pip

Using homebrew:

		$ brew install pip

## Installing Dependencies

#### Linux
##### Ubuntu/Debian

        $ sudo apt-get install build-essential libxml2-dev libxslt1-dev zlib1g-dev  wget python-dev  libffi-dev libssl-dev python-setuptools

##### Fedora

        $ sudo yum install gcc redhat-rpm-config python-devel libffi-devel openssl-devel python-setuptools

##### OpenSuse

        $ sudo zypper install gcc python-dev libffi-dev openssl-devel python-setuptools

## Installing KubOS-SDK

The KubOS-SDK is distributed using the python package system pip. You can install using this command:

		$ pip install kubos-sdk

KubOS-SDK is currently only supported in 64-bit OSX and Linux environments.

Pull the latest Kubos-SDK docker container:

		$ kubos update

## Upgrading KubOS-SDK (v0.0.2+)

The KubOS-SDK can be upgraded using this pip command:

		$ sudo pip install --upgrade kubos-sdk

Be sure to pull the latest Kubos-SDK docker container afterwards:

		$ kubos update

## <a name="upgrading"></a>Upgrading KubOS-SDK (from v0.1 - initial release)

The KubOS-SDK can be upgraded from version 0.1 using this command:

		$ sudo pip install -I kubos-sdk

Be sure to pull the latest Kubos-SDK docker container afterwards:

		$ kubos update

## Creating a new project

Run the `kubos init` command followed by the name of your project to bootstrap your KubOS project. This will create a new directory with your project's name and add the basic files.

		$ kubos init project-name

The contents of your project directory should look something like this:

		$ ls
		module.json  project-name  source  test

Here is a quick rundown of the files that were generated:

 * project-name - This folder is where header files live
 * source - This folder is where source files live
 * test - This folder is where test source files live
 * module.json - This file is yotta's module description file.

KubOS uses the yotta build/module system, which is where this file structure comes from. You can read more about yotta [here](http://yottadocs.mbed.com/).


## Building your project

Building a KubOS project is also a two step process:

#### 1. Select your target

Yotta needs to know which target you intend to build for so it can select the proper cross compiler. KubOS currently supports two different targets:

 * STM32F407 Discovery Board
 * STM32F405 PyBoard
 * NanoAvionics SatBus 3C0 OBC
 * MSP430F5529 Launchpad

The respective commands to select those targets are as follows.

		$ kubos target stm32f407-disco-gcc

		$ kubos target pyboard-gcc

		$ kubos target na-satbus-3c0-gcc

		$ kubos target msp430f5529-gcc

#### 2. Build!

Once your target has been selected you can run the build command

		$ kubos build

If all goes well you should see this message:

		Build Succeeded

## Working with your project

Part of the Kubos-SDK is a set of tools designed to enhance your development process. We are still in the process of developing this toolset, so be sure to check back with each release!

#### Linking External Modules

Kubos link allows you to symlink local modules to be used as dependencies. By default our sdk container comes with all the module dependencies you need to build a KubOS project. For most users this will be enough. However in some cases linking in local modules is very necessary.

For instance, the kubos-rt-example depends on the libcsp module. If you want to make changes to libcsp you would clone both repositories, and make your changes to libcsp. To test these changes you would link libcsp into your clone of kubos-rt-example. This would allow you to build the example with your libcsp changes and test them locally.

##### Linking modules:

 * Links are made in two steps - first globally then locally.

 * By linking a module globally you are making it available to link into any of your projects. By linking the module locally you are including the linked module in your build.

 * To link a module globally:

		$ cd .../<module-directory>/
		$ kubos link

 * To link a module that is already globally linked into a project:

		$ cd .../<project-directory>/
		$ kubos link <module name>

 * To link a module directly into a project in one step:

		$ cd .../<project-directory>/
		$ kubos link /path/to/module/

 * By doing this in one step kubos automatically links the module globally and then links it into your local project for you.

The next time your project is built it will use your local development module, rather than the packaged version.


## Flashing your project

At this point you've created a new project, written some fancy code (or borrowed our [example app](https://github.com/openkosmosorg/kubos-rt-example)) and built it. Now you want to run it!

Flashing your project using the kubos tool is a relatively straightforward process:

1. Ensure that your board is plugged into your computer

2. Run the flash command

		$ kubos flash

*Note: If your current user does not have read/write permission to your hardware device you may need to run this command as root*

		$ sudo kubos flash

#### Debug your project

A gdb server must be started to allow your gdb instance to connect and debug directly on your hardware device.
After building your project with `kubos build` kubos can manage a gdb server and gdb instance for you.

Start a gdb server and instance for you:
Note: this may need to run as root depending on your usb device permissions

		$ kubos debug

Additionally you can interact directly with the gdb server:

		$ kubos server <start, stop, restart, status>
