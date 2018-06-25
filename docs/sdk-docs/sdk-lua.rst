Using Lua with the Kubos SDK
============================

The Kubos SDK Vagrant box contains limited support for running Lua based
projects. This functionality is primarily used when running the communications,
shell and file services or clients.

.. note::

    All of the following instructions are assumed to be run inside of the
    Kubos SDK Vagrant environment.


Running Existing Projects
-------------------------

To run an existing lua project first navigate to the project`s folder::

    $ cd /home/vagrant/kubos/services/shell-service

Next use the `lit` tool to install any lua dependencies::

    $ lit install

Finally the project can be run using the `luvi-regular` binary::

    $ luvi-regular .

Any necessary command line arguments for the project may be passed like so::

    $ luvi-regular . -- arg1 arg2
