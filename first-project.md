# Getting started with KubOS-SDK

This is intended to be a quick guide to creating a new project on top of the Kubos framework.

## Prerequisites

[Install the KubOS-CLI](docs/cli-installing.md)

Create an instance of the Kubos Vagrant box

        $ vagrant init kubostech/kubos-sdk

Start the box

        $ vagrant up

SSH into your box

        $ vagrant ssh

## Creating your project

The simplest way to create a new Kubos project is by using the kubos sdk. The `kubos init` command takes a project name and creates the project files & folder.

It is strongly recommended that you create your project in a directory on your host that is shared with your box, rather than directly inside your box. If the
directory is located on your host, if your box is ever destroyed or re-built your project files will be completely intact.

For more information on mounting volumes see the following [guide](https://www.vagrantup.com/docs/synced-folders/basic_usage.html)

        $ kubos init myproject

The init command creates a new directory with the kubos-rt-example included so you can get started right away.

Note - Inside of the build system there are several reserved words, a project cannot be named any of these words. The most common of these are `test`, `source` and `include`.

We have also created several different example Kubos projects which can be used as starting points.

 - [Example showing basic freertos tasks and csp](https://github.com/kubostech/kubos-rt-example)
 - [Example showing the i2c HAL and sensors](https://github.com/kubostech/kubos-i2c-example)
 - [Example showing the spi HAL and sensors](https://github.com/kubostech/kubos-spi-example)
 - [Example showing the sensor interface](https://github.com/kubostech/kubos-sensor-example)
 - [Example showing csp over uart](https://github.com/kubostech/kubos-csp-example)

If you would prefer to use one of our other examples as a starting point all you need to do is run:

        $ git clone https://github.com/kubos-rt-example myproject

## Choosing a target

Once you have created a project you will need to select a target. The target defines which hardware your project will run on and how the peripherals are configured.

You can see a list of available projects by running the following command:

        $ kubos target --list

For this example we will set the msp430f5529 target:

        $ kubos target msp430f5529-gcc

## Building and flashing

Now that the target is set you can begin building. This command will build the current project:

        $ kubos build

You should see the `Build Succeeded` message! You are now ready to load your software on some hardware. Connect your hardware to your computer and run the following flash command:

         $ kubos flash

Note - You may need to run this command with `sudo` if you run into a permissions error.

        $ sudo kubos flash

Congratulations! You have just created a basic KubOS project, built it and (hopefully) flashed it onto some hardware.
