# Developing KubOS Modules

The top level [Kubos](https://github.com/kubostech/kubos) project contains all of the kubos source modules and targets.

## Getting started - Modifying an existing Kubos module

1. [Install the latest version of Kubos-CLI](docs/sdk-installing.md)
2. Clone the Kubos repo

        $ git clone https://github.com/kubostech/kubos

## Kubos development environment

Kubos is a collection of Yotta modules and targets which are loaded inside the kubos-sdk Vagrant box. They can also be built locally using the `kubos link` and `kubos link-target`
commands.

### Building an example application

Several different example applications can be found in the Kubos Example repos. Any of these can be easily built using the CLI.

        $ kubos init <project_name> #will initialize a new project with the [kubos-rt-example project](https://github.com/kubostech/kubos-rt-example)
        $ kubos target msp430f5529-gcc
        $ kubos build

### Linking in a local module

Made some modifications to an existing module? Want to link in a new library? The kubos-cli can help with that as well.

        $ cd /home/kubos/super-awesome-space-library
        $ sudo kubos link
        $ cd /home/kubos/example
        $ kubos link super-awesome-space-library
        $ kubos build

After running the `kubos link` command from the module directory and `kubos link <module name>` from the project directory, `kubos build` will pick up the module and pull it into the build process.
