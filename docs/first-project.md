# Getting started with Kubos SDK

This is intended to be a quick guide to creating a new project on top of the Kubos framework.

## Prerequisites

[Install the Kubos CLI](docs/cli-installing.md)

Create an instance of the Kubos Vagrant box

        $ vagrant init kubostech/kubos-dev

Start the box

        $ vagrant up

SSH into your box

        $ vagrant ssh

## Creating your project

It is strongly recommended that you create your project in a directory on your host that is shared with your box, rather than directly inside your box. If the
directory is located on your host, if your box is ever destroyed or re-built your project files will be completely intact.

To mount a specific directory from your host, open the Vagrantfile that was in the kubos-vagrant directory you clone in the above step and look for the following lines:

        # Share an additional folder to the guest VM. The first argument is
        # the path on the host to the actual folder. The second argument is
        # the path on the guest to mount the folder. And the optional third
        # argument is a set of non-required options.
        # config.vm.synced_folder "../data", "/vagrant_data"

Uncomment the last line in this block and change the paths to match your host directory and a desired mount point in the box.

Note - The path in the box must be an absolute path. In the kubos-dev vagrant box the home directory is `/home/vagrant`

After a volume is mounted into the box all of the data from the host path will be available at the path specified for the box. In the above example the host path (`../data`) would be exposed at `/vagrant_data` inside of the box.
This allows you to use the text editor of your choosing to edit the project files from your host machine at the host directory path.

For more information on mounting volumes see the following [guide](https://www.vagrantup.com/docs/synced-folders/basic_usage.html)

The simplest way to create a new KubOS RT project is by using the Kubos CLI. The `kubos init` command takes a project name and creates the project files & folder.

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

If you created your project from a clone there's some additional setup needed to satisfy all of the projects depedencies for Kubos source modules.

Running the following commands will clone a project and link all of the Kubos modules needed to build it:

        $ git clone <url of the project you want>
        $ cd <project name>
        $ kubos link --all

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
