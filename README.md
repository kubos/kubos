KubOS Source Distribution
=========

The top level [Kubos](https://github.com/kubostech/kubos) project acts as a workspace for all KubOS RT related
projects to help simplify development.

Git repositories are managed with Android's ```repo``` utility, and once the workspace
is initialized, you can use repo to sync the project.

## Getting started

1. [Install the latest version of Kubos SDK](docs/kubos-sdk.md)
2. Clone the Kubos repo

        $ git clone https://github.com/kubostech/kubos

2. Run the development environment bootstrap script to pull down all of the KubOS
   repositories:

        $ cd kubos
        $ ./bootstrap.sh

## KubOS development environment

KubOS is a collection of Yotta modules and targets which come pre-packaged with the KubOS-SDK. They can also be built locally using the `kubos link` and `kubos link-target`
commands.

### Building an example application

Several different example applications can be found in the examples folder. Any of these can be easily built using the sdk.

        $ cd examples/kubos-rt-example
        $ kubos target msp430f5529-gcc
        $ kubos build

### Linking in a local folder

Made some modifications to an existing module? Want to link in a new library? The kubos tool can help with that as well.

        $ cd examples/kubos-rt-example
        $ kubos target msp430f5529-gcc
        $ kubos link /home/kubos/super-awesome-space-library
        $ kubos build

After running the `kubos link` command with the absolute path of a module, `kubos build` will pick up the module and pull it into the build process.

## Synchronizing repositories

Want to get the latest source? This command will sync the local source tree with the kubos source in github.

        $ ./repo sync
