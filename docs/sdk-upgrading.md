# Upgrading Your Kubos Development Environment

New updates will be announced on the Kubos website. There will be instructions depending on the specifc release details of all the steps needed to upgrade for the newest release.

## Upgrading Kubos CLI

Inside of a vagrant box lives the Kubos CLI. Upon new Kubos releases there may be updates for the Kubos CLI.

First SSH into your kubos-dev box:

       $ cd <The path containing your kubos-dev Vagrantfile>
       $ vagrant ssh

The Kubos CLI can be upgraded using this pip command:

        $ sudo pip install --upgrade kubos-cli

## Upgrading the Kubos Source Modules

To update the KubOS source modules your project will be built with run the following command:

        $ kubos update

To check which version of the CLI you're using, use `kubos version`

To list all of the available versions of the KubOS s4ource modules run:

        $ kubos versions

To activate and use a new version of the KubOS source run:

        $ kubos use <version number>

The `use` command will checkout and replace the existing KubOS source modules.

After running the `use` command, modules from the new version will be linked.
If the new release holds new modules or has removed existing modules it may be necessary to re-link all modules to get rid of any module conflicts between new and old versions.

To relink all of the kubos source modules simply run:

        $ kubos link --all

## Downgrading the Kubos Source Modules

In the event you want to downgrade to an older version of the Kubos source modules simply use the `kubos use <version>` command with the older version number you want to downgrade to.
