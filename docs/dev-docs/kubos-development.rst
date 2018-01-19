Developing Kubos Modules
========================

The top level `Kubos <https://github.com/kubos/kubos>`__ project
contains all of the Kubos source modules and targets.

Getting Started
---------------

If you want to make changes to the Kubos code, perhaps for debugging
purposes or to support a new peripheral, you'll first need to clone the
kubos repo and then pass the folder through to your VM:

:doc:`Install the latest version of the Kubos SDK <../installation-docs/sdk-installing>`

Clone the Kubos repo to your host machine.

::

    $ git clone https://github.com/kubos/kubos
        

Update your Vagrantfile to pass the repo folder through to your VM. The
destination folder will be created if it doesn't already exist.

::

    config.vm.synced_folder "C:\\Users\\Catherine\\git\\kubos", "/home/vagrant/shared"

Reload your vagrant image to pick up the new synced folder.

::

    $ vagrant reload

Log in to your vagrant image

::

    $ vagrant ssh       

**Note:** It is possible to do development on the kubos repo from within
the vagrant image, but it is our recommended workflow to have the repo
on your host machine and pass it through. This way if the image becomes
corrupted, or if you want to pass the modified code through to another
VM, it's still available.

Kubos Development Environment
-----------------------------

The kubos repository is a collection of
`Yotta <http://yottadocs.mbed.com/>`__ modules and targets which are
loaded inside the Kubos Vagrant box. They can also be built locally
using the ``kubos link`` and ``kubos link-target`` commands.

See the :doc:`KubOS Linux quick start guide <../os-docs/first-linux-project>` 
for instructions on setting up and building Kubos SDK projects.

Linking in a Local Module
~~~~~~~~~~~~~~~~~~~~~~~~~

Once you've made changes to your local kubos repo, you'll want to link
them into your project.

**Note:** If you create a new high-level component, like telemetry or
hal, you'll need to create a module.json file so that the module can be
linked in successfully.

Let's say that you've updated the Kubos telemetry module to add
debugging lines to see how the flow of communication works between
processes. This would be your process to link and build the changes:

::

    $ cd /home/vagrant/shared/telemetry
    $ kubos link
    $ cd /home/vagrant/my-project
    $ kubos link telemetry
    $ kubos build

After running the ``kubos link`` command from the module directory and
``kubos link <module name>`` from the project directory, ``kubos build``
will pick up the module and pull it into the build process.

**Note:** The module name is taken from the "name" definition in the
module.json file, not from the folder name. For example, to link in the
CSP module, you would do ``kubos link csp``, not ``kubos link libcsp``.

Linking in a Local Target
~~~~~~~~~~~~~~~~~~~~~~~~~

If you want to add or update a Kubos target, you'll follow a similar
process. For example:

::

    $ cd /home/vagrant/shared/targets/target-stm32f407-disco-gcc
    $ kubos link-target
    $ cd /home/vagrant/my-project
    $ kubos link-target stm32f407-disco-gcc
    $ kubos build

**Note:** The target name is taken from the "name" definition in the
target.json file, not from the folder name.

Unlinking Modules and Targets
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

If you'd like to unlink your local changes and revert to using the
official Kubos version, use the ``kubos unlink`` and
``kubos unlink-target`` commands from within your project

::

    $ cd /home/vagrant/my-project
    $ kubos unlink csp
    $ kubos unlink-target kubos-linux-beaglebon-gcc

Listing Linked Resources
~~~~~~~~~~~~~~~~~~~~~~~~

To see what the dependencies of your project are and which folders are
currently being used to build, use the ``kubos ls`` command.

Any modules which have be linked from an outside resource will show that
file path. Any modules which are using the native Kubos code will have a
'/home/vagrant/.kubos' path.

::

    vagrant@vagrant:~/my-project$ kubos ls
    my-project 0.1.0
      ┗━ imtq-api 1.0.0 yotta_modules/imtq-api -> /home/vagrant/iobc/imtq-api
      ┣━ adcs-api 0.1.0 yotta_modules/adcs-api -> /home/vagrant/.kubos/kubos/adcs/adcs-api
      ┣━ kubos-hal 0.1.2 yotta_modules/kubos-hal -> /home/vagrant/.kubos/kubos/hal/kubos-hal
      ┃ ┣━ csp 1.5.1 yotta_modules/csp -> /home/vagrant/.kubos/kubos/libcsp
      ┃ ┃ ┗━ tinycbor 0.5.0 yotta_modules/tinycbor -> /home/vagrant/.kubos/kubos/tinycbor
      ┃ ┗━ kubos-hal-linux 0.1.0 yotta_modules/kubos-hal-linux -> /home/vagrant/.kubos/kubos/hal/kubos-hal-linux
      ┃   ┗━ kubos-hal-iobc 0.1.0 yotta_modules/kubos-hal-iobc -> /home/vagrant/.kubos/kubos/hal/kubos-hal-iobc
      ┗━ ccan-json 1.0.0 yotta_modules/ccan-json -> /home/vagrant/.kubos/kubos/ccan/json


Similarly, to see the dependencies of your target and any linked
resources, use the ``kubos target`` command.

::

    vagrant@vagrant:~/my-project$ kubos target
    kubos-linux-beaglebone-gcc 0.1.1 -> /home/vagrant/.kubos/kubos/targets/target-kubos-linux-beaglebone-gcc
    kubos-linux-gcc 0.1.1 -> /home/vagrant/.kubos/kubos/targets/target-kubos-linux-gcc
    kubos-gcc 0.1.1 -> /home/vagrant/.kubos/kubos/targets/target-kubos-gcc

