# Upgrading Your Kubos Development Environment

## Upgrading Kubos CLI

Inside of a vagrant box lives the Kubos CLI. Upon new KubOS releases there may be update for the Kubos CLI.

First SSH into your kubos-dev box:

       $ cd <The path containing your kubos-dev Vagrant file>
       $ vagrant ssh

The Kubos CLI can be upgraded using this pip command:

        $ sudo pip install --upgrade kubos-cli

## Upgrading the KubOS Source Modules

To upgrade the KubOS source modules your project will be built with run the following command:

        $ kubos update

To list the current CLI versions and the currently active version of the KubOS Source run the following command:

        $ kubos version

To list all of the avaialble versions of the KubOS source modules run:

        $ kubos versions

To activate and use a new version of the KubOS source run:

        $ kubos use <version number>

