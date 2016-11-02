# Installing Kubos-SDK

## Prerequisites

### Install VirtualBox

Vagrant requires a virtualization "provider". Currently the only provider that Kubos officially supports is VirtualBox.

See the VirtualBox [downloads](https://www.virtualbox.org/wiki/Downloads) for more information on installing VirtualBox.

### Install Vagrant

If you don't already have Vagrant installed see the Vagrant [installation documentation.](https://www.vagrantup.com/docs/installation://www.vagrantup.com/docs/installation/)

Make sure vagrant is installed properly:

        $ vagrant --version


### Create your kubos Vagrant Box:

        $ vagrant init kubostech/kubos-sdk # or whatever init method we have decided to go with now

This will create a vagrant file in your current Directory. This is important because to interact with this box in the future you will need to
cd back to this same directory you have initialized the box in.

It is strongly recommended that you create your project in a directory on your host that is shared with your box, rather than directly inside your box. If the
directory is located on your host, if your box is ever destroyed or re-built your project files will be completely intact.

By default the directory that you run `vagrant init kubostech/kubos-sdk` in will be mounted into the box at the path `/vagrant`

To mount a specific directory from your host, open the Vagrantfile and look for the following two lines:

        #To mount a specific directory into your box uncomment the next line and change the following paths to match your host directory and a desired mount point in the box.
        #config.vm.synced_folder "/path/on/host", "/path/in/vagrant/box"

Un-comment the second line and change the paths to match your host directory and a desired mount point in the box. Note - These must be absolute paths.

For more information on mounting volumes see the following [guide](https://www.vagrantup.com/docs/synced-folders/basic_usage.html)

This initialization command may take a few minutes to run - It is provisioning a virtual machine with all of the required build, flashing and debugging
utilities you will need to work with a Kubos project. Addiitonall it is installing the Kubos-CLI.

        $ vagrant ssh

This will start an ssh session in the vagrant box with the kubos-cli and all of the required dependencies installed.

That's it! From here see more on [creating your first project](docs/first-project.md)
