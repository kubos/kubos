# Upgrading Your Kubos Development Environment

## Upgrading Kubos CLI

Inside of a vagrant box lives the Kubos CLI. Upon new KubOS releases there may be update for the Kubos CLI.

First SSH into your kubos-dev box:

       $ cd <The path containing your kubos-dev Vagrant file>
       $ vagrant ssh

The Kubos CLI can be upgraded using this pip command:

        $ sudo pip install --upgrade kubos-cli

## Upgrading the KubOS Source Modules

To update the KubOS source modules your project will be built with run the following command:

        $ kubos update

To check which version of cli, use `kubos version`

To list all of the available versions of the KubOS source modules run:

        $ kubos versions

To activate and use a new version of the KubOS source run:

        $ kubos use <version number>

The `use` command will checkout and replace the existing KubOS source modules.

After running the `use` command, modules from the new version will be linked.
If the new release holds new modules or has removed existing modules it may be necessary to re-link all modules to get rid of any module conflicts between new and old versions.

To relink all of the kubos source modules simply run:

        $ kubos link --all
