# Installing Kubos-SDK

## What is the Kubos-SDK?

The kubos-sdk is a term used to describe the all of the components used that make up a KubOS operating system and software project as well as the tools used to build this project.
The kubos-sdk components are made up of:

 * KubOS source modules - the individual components of the operating systems, hardware abstraction layers, and APIs
 * kubos-cli - The command line tool used to create, configure, build and debug KubOS projects
 * Other relevant components?


## How Does The  Work?

The kubos-sdk is distributed through a vagrant box. A vagrant box is a command-line based virtual machine. This virual machine contains all of the KubOS source code, compiler toolchains,
debugging utilities and miscelaneous tools the kubos-cli. The box, when started is already pre-configured with all of the required tools you will need. This minimizes the set-up process
so you can work on your project rather than setting up tooling.

Vagrant is a nice interface that abstracts the virtualization provider into a simple to use interface. Vagrant supports a variety of providers (VirtualBox, VmWare, Parallels, etc.) but 
right now the kubos-sdk only supports VirtualBox.


## Prerequisites

### Install VirtualBox

Vagrant requires a virtualization "provider". Currently the only provider that Kubos officially supports is VirtualBox.

See the VirtualBox [downloads](https://www.virtualbox.org/wiki/Downloads) for more information on installing VirtualBox.

Passing USB devices into a virtual machine also requires the VirtualBox Extension Pack which is also available from VirtualBox's [downloads](https://www.virtualbox.org/wiki/Downloads)

### Install Vagrant

If you don't already have Vagrant installed see the Vagrant [installation documentation.](https://www.vagrantup.com/docs/installation)

Make sure vagrant is installed properly:

        $ vagrant --version

## Set-Up

### Create your kubos Vagrant Box:

To create an instance of the kubos-sdk box follow these steps:

Clone kubostech/kubos-vagrant repo:

       $ git clone https://github.com/kubostech/kubos-vagrant

The kubostech/kubos-vagrant repo contains the Vagrant file and all necessary files to provision your the kubos-sdk box.

Ignore the last bit of this section unless this gets published to the hashicorp atlast. I (kyle) have no idea if we're doing that now or not.
Vagrant init

       $ vagrant init kubostech/kubos-sdk # or whatever init method we have decided to go with now

This will create a vagrant file in your current Directory. This is important because to interact with this box in the future you will need to
cd back to this same directory you have initialized the box in.


### Mounting a host directory:

It is strongly recommended that you create your project in a directory on your host that is shared with your box, rather than directly inside your box. If the
directory is located on your host, if your box is ever destroyed or re-built your project files will be completely intact.

By default the directory that you run `vagrant init kubostech/kubos-sdk` in will be mounted into the box at the path `/vagrant`

To mount a specific directory from your host, open the Vagrantfile that was in the kubos-vagrant directory you clone in the above step and look for the following two lines:

        #To mount a specific directory into your box uncomment the next line and change the following paths to match your host directory and a desired mount point in the box.
        #config.vm.synced_folder "/path/on/host", "/path/in/vagrant/box"

Un-comment the second line and change the paths to match your host directory and a desired mount point in the box. Note - These must be absolute paths.

After a volume is mounted the directory and all of its contents in directory `/path/on/host` on the host will be available in the container at the `/path/in/vagrant/box` directory in the kubos-sdk box.
This allows you to use the text editor of your choosing to edit the project files from your host machine at the host directory path.

Note: If you make changes to the vagrant file after the kubos-sdk box has already been started you will need to run `vagrant reload` in order for these changes to take effect in the kubos-sdk box.

#####For more information on mounting volumes see the following [guide](https://www.vagrantup.com/docs/synced-folders/basic_usage.html)


### Start the vagrant box:

To start the box after modifying your Vagrant file run:

        $ vagrant up

This initialization command may take a few minutes to run - It is provisioning a virtual machine with all of the required build, flashing and debugging
utilities you will need to work with a Kubos project. Additionally it is installing the Kubos-CLI.

After the box has started you need to "ssh" into the machine to work with your projects.

        $ vagrant ssh

This will start an ssh session in the vagrant box with the kubos-cli and all of the required dependencies installed.

That's it! From here see more on [creating your first project](docs/first-project.md)

