# Upgrading

## Upgrading from KubOS-SDK (v0.0.2+)

The KubOS-SDK can be upgraded using this pip command:

		$ sudo pip install --upgrade kubos-sdk

Be sure to pull the latest Kubos-SDK docker container afterwards:

		$ kubos update

## Upgrading links to Kubos-SDK v0.1.0

The link commands (link and link-target) store link data in two separate files. The Kubos-sdk version 0.1.0 addition of the link-target command forced us to use a new json format. If you have created links with a previous version of the kubos-sdk they are no longer compatible with version 0.1.0.

The two files are located in your home directory and in your project directory. The file in your home directory is named .kubos-link-global.json and in your project directory is .kubos-link.json

Currently removing both of these files and recreating the links is the best way to upgrade existing links to the newer format.

