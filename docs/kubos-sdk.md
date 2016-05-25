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
                            

## Installing KubOS-SDK

The KubOS-SDK is distributed using the python package system pip. You can install using this command:

            $ pip install kubos-sdk

KubOS-SDK is currently only supported in 64-bit OSX and Linux environments.

Pull the latest Kubos-SDK docker container:
            
            $ kubos update

## Creating a new project

Creating a new KubOS project is a two step process:

#### 1. Create your project directory

        $ mkdir project-name

#### 2. Initialize the project

From inside of the project directory run the `kubos init` commmand

        $ cd project-name
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
 * MSP430F5529 Launchpad

The respective commands to select those targets are as follows.

        $ kubos target stm32f407-disco-gcc@openkosmosorg/target-stm32f407-disco-gcc

        $ kubos target msp430f5529-gcc@openkosmosorg/target-msp430f5529-gcc

#### 2. Build!

Once your target has been selected you can run the build command

        $ kubos build

If all goes well you should see this message:

        Build Succeeded


## Flashing your project

At this point you've created a new project, written some fancy code (or borrowed our [example app](https://github.com/openkosmosorg/kubos-rt-example)) and built it. Now you want to run it!

Flashing your project using the kubos tool is a relatively straightforward process:

1. Ensure that your board is plugged into your computer

2. Run the flash command

        $ kubos flash

*Note: If your current user does not have read/write permission to your hardware device you may need to run this command as root*

        $ sudo kubos flash
