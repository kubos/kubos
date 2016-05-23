KubOS Source Distribution
=========

This top level project acts as a workspace for all KubOS RT and KubOS Linux related
projects to help simplify development.

Git repositories are managed with Android's ```repo``` utility, and once the workspace
is initialized, you can use repo to sync the project.

## Getting started

1. [Install the latest stable version of yotta](http://yottadocs.mbed.com/#installing)
2. Run the development environment bootstrap script to pull down all of the KubOS
   repositories, and setup the yotta development environment:

        $ ./bootstrap.sh

## KubOS development environment

KubOS is a collection of Yotta modules and targets that can either be fetched
remotely, or built locally using the `yotta link` and `yotta link-target`
commands. Since KubOS has a large number of modules and targets, we use a tool
to automate the environment setup, `tools/yotta_link.py`. Here are some example
usages of the tool:

Add module and target symlinks to the system for use by applications:
        $ ./tools/yotta_link.py --sys

Uninstall system symlinks:

        $ ./tools/yotta_link.py --unlink --sys

Install symlinks into a KubOS application:

        $ ./tools/yotta_link.py --app <path-to-app>

Uninstall application symlinks:

        $ ./tools/yotta_link.py --unlink --app <path-to-app>

To install system symlinks and application symlinks for all example apps in the
KubOS distribution (this is also the default action when no arguments are used,
and is called from `bootstrap.sh`):

        $ ./tools/yotta_link.py --all

See the full help documentation by running `./tools/yotta_link.py --help`

## Synchronizing repositories

        $ ./repo sync
