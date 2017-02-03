# Getting started with Kubos SDK

This is intended to be a quick guide to creating a new KubOS RT project on top of the Kubos framework.

## Prerequisites

[Install the Kubos SDK](docs/cli-installing.md)

Create an instance of the Kubos Vagrant box

        $ vagrant init kubostech/kubos-dev

Start the box

        $ vagrant up

SSH into your box

        $ vagrant ssh

## Creating your project

The simplest way to create a new Kubos project is by using the Kubos CLI. The `kubos init` command takes a project name and creates the project files and folders.

**Note:** Inside of the build system there are several reserved words, a project cannot be named any of these words. The most common of these are `test`, `source` and `include`.

        $ kubos init myproject

The init command creates a new directory with the kubos-rt-example included so you can get started right away.


We have also created several different example Kubos projects which can be used as starting points.

 - [Example showing basic freertos tasks and csp](https://github.com/kubostech/kubos-rt-example)
 - [Example showing the i2c HAL and sensors](https://github.com/kubostech/kubos-i2c-example)
 - [Example showing the spi HAL and sensors](https://github.com/kubostech/kubos-spi-example)
 - [Example showing the sensor interface](https://github.com/kubostech/kubos-sensor-example)
 - [Example showing csp over uart](https://github.com/kubostech/kubos-csp-example)
 - [Example KubOS Linux project](https://github.com/kubostech/kubos-linux-example)

If you would prefer to use one of our other examples as a starting point all you need to do is run:

        $ git clone https://github.com/kubos-spi-example myproject
        
It is unnecessary to run the `kubos init` command in this case.

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

Congratulations! You have just created a basic Kubos project, built it and (hopefully) flashed it onto some hardware.
