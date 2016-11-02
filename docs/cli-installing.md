# Installing Kubos-SDK

## Prerequisites

### Install Vagrant

If you don't already have Vagrant installed see the Vagrant [getting started guide](https://www.vagrantup.com/docs/getting-started/)

Make sure vagrant is installed properly:

        $ vagrant --version


### Create your kubos Vagrant Box:

        $ vagrant init kubostech/kubos-sdk # or whatever init method we have decided to go with now

    This will create a vagrant file in your current Directory. This is important because to interact with this box in the future you will need to
    cd back to this same directory you have initialized the box in.

    This initialization command may take a few minutes to run - It is provisioning a virtual machine with all of the required build, flashing and debugging
    utilities you will need to work with a Kubos project. Addiitonall it is installing the Kubos-CLI.

        $ vagrant ssh

    This will start an ssh session in the vagrant box with the kubos-cli and all of the required dependencies installed.

    That's it! From here see more on [creating your first project](docs/first-project.md)
