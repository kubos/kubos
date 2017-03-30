# Installing Kubos SDK

## What is the Kubos SDK? {#what-is-kubos-sdk}

The Kubos SDK is a term used to describe the all of the components used that make up a KubOS operating system and the tools used to build this project.
The Kubos SDK components are made up of:

 * Kubos source modules - the individual components of the operating systems, hardware abstraction layers, and APIs
 * Kubos CLI - The command line tool used to create, configure, build and debug KubOS projects


## How Does The SDK Work? {#how-sdk-works}

The Kubos SDK is distributed through a vagrant box. A vagrant box (referred to simply as a "box") is a command-line based virtual machine. This virtual machine contains all of the Kubos source code, compiler toolchains,
debugging utilities and miscellaneous tools the Kubos CLI. The box, when started, is already pre-configured with all of the required tools for the CLI you will need. This minimizes the set-up process
so you can work on your project rather than setting up tooling.

[Vagrant](https://www.vagrantup.com/docs/) is a nice interface that abstracts the virtualization provider into a simple to use interface. Vagrant supports a variety of providers (VirtualBox, VmWare, Parallels, etc.) but
right now the Kubos SDK only supports the VirtualBox provider.


## Prerequisites

### Install VirtualBox {#install-virtualbox}

Vagrant requires a virtualization "provider". Currently the only provider that Kubos officially supports is VirtualBox.

See the VirtualBox [downloads](https://www.virtualbox.org/wiki/Downloads) for more information on installing VirtualBox.

Passing USB devices into a virtual machine also requires the VirtualBox Extension Pack which is also available from VirtualBox's [downloads](https://www.virtualbox.org/wiki/Downloads)

### Install Vagrant {#install-vagrant}

If you don't already have Vagrant installed see the Vagrant [installation documentation.](https://www.vagrantup.com/docs/installation)

Make sure vagrant installation is set up properly:

        $ vagrant --version

## Set-Up

### Create your Kubos SDK Vagrant Box: {#create-sdk-box}

To create an instance of the SDK box follow these steps:

       $ vagrant init kubostech/kubos-dev
       $ vagrant up

This will create a Vagrantfile in your current directory. Vagrantfiles are important as they contain the configuration details for specific boxes.
Vagrantfiles are dependent on your working directory. To interact with this box in the future you will need to `cd` back to this same directory you have initialized the box in.


### Mounting a host directory: {#mount-directory}

It is strongly recommended that you create your project in a directory on your host that is shared with your box. By keeping your project on your host it will protect them in the event your box is destroyed or re-built.

To mount a specific directory from your host, open the Vagrantfile located in the directory you ran `vagrant init` inside of in the above step and look for the following lines:

        # Share an additional folder to the guest VM. The first argument is
        # the path on the host to the actual folder. The second argument is
        # the path on the guest to mount the folder. And the optional third
        # argument is a set of non-required options.
        # config.vm.synced_folder "../data", "/vagrant_data"

Uncomment the last line in this block and change the paths to match your host directory and a desired mount point in the box. 

Note: The path in the box must be an absolute path

After a volume is mounted into the box all of the data from the host path will be available at the path specified for the box. In the above example the host path (`../data`) would be exposed at `/vagrant_data` inside of the box.
This allows you to use the text editor of your choosing to edit the project files from your host machine at the host directory path.

Note: If you make changes to the Vagrantfile after the box has been started you will need to run `vagrant reload` for these changes to take effect in the box.

#####For more information on mounting volumes see the following [guide](https://www.vagrantup.com/docs/synced-folders/basic_usage.html)

### Start the vagrant box: {#start-sdk-box}

To start the box after modifying your Vagrantfile run:

        $ vagrant up

After the box has started you need to "ssh" into the machine to work with your projects.

        $ vagrant ssh

This will start an ssh session in the vagrant box with the Kubos CLI and all of the required dependencies installed.

That's it! From here see more on [creating your first project](docs/first-project.md)

After a little bit of usage you may want to look at [how to upgrade the Kubos SDK](docs/sdk-upgrading.md)
