Developing Kubos Modules
========================

The top level `Kubos <https://github.com/kubostech/kubos>`__ project
contains all of the Kubos source modules and targets.

Getting Started
---------------

If you want to make changes to the Kubos code, perhaps for debugging
purposes or to support a new peripheral, you'll first need to clone the
kubos repo and then pass the folder through to your VM:

:doc:`Install the latest version of the Kubos SDK <sdk-installing>`

Clone the Kubos repo to your host machine.

::

    $ git clone https://github.com/kubostech/kubos
        

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

See the :doc:`KubOS RT quick start guide <first-rt-project>` or the
:doc:`KubOS Linux quick start guide <first-linux-project>` for instructions
on setting up and building Kubos SDK projects.

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
    $ kubos unlink telemetry
    $ kubos unlink stm32f407-disco-gcc

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
    ┗━ kubos-rt 0.1.0 yotta_modules/kubos-rt -> /home/vagrant/sharedOS/kubos-rt
    ┣━ freertos 9.0.4 yotta_modules/freertos -> /home/vagrant/.kubos/kubos/freertos/os
    ┃   ┣━ cmsis-core 1.2.4 yotta_modules/cmsis-core -> /home/vagrant/.kubos/kubos/cmsis/cmsis-core
    ┃   ┃   ┗━ cmsis-core-st 1.0.5 yotta_modules/cmsis-core-st -> /home/vagrant/.kubos/kubos/cmsis/cmsis-core-st
    ┃   ┃          ┗━ cmsis-core-stm32f4 1.2.4 yotta_modules/cmsis-core-stm32f4 -> /home/vagrant/.kubos/kubos/cmsis/cmsis-core-stm32f4
    ┃   ┃              ┣━ stm32cubef4 1.2.4 yotta_modules/stm32cubef4 -> /home/vagrant/.kubos/kubos/hal/stm32cubef4
    ┃   ┃              ┃   ┗━ stm32cubef4-stm32f407vg 0.0.3 yotta_modules/stm32cubef4-stm32f407vg -> /home/vagrant/.kubos/kubos/hal/stm32cubef4-stm32f407vg
    ┃   ┃              ┗━ cmsis-core-stm32f407xg 0.0.4 yotta_modules/cmsis-core-stm32f407xg -> /home/vagrant/.kubos/kubos/cmsis/cmsis-core-stm32f407xg
    ┃   ┗━ freertos-config-stm32f4 0.0.3 yotta_modules/freertos-config-stm32f4 -> /home/vagrant/.kubos/kubos/freertos/config-stm32f4
    ┣━ csp 1.5.1 yotta_modules/csp -> /home/vagrant/sharedOS/libcsp
    ┣━ kubos-hal 0.1.2 yotta_modules/kubos-hal -> /home/vagrant/.kubos/kubos/hal/kubos-hal
    ┃   ┗━ kubos-hal-stm32f4 0.1.2 yotta_modules/kubos-hal-stm32f4 -> /home/vagrant/.kubos/kubos/hal/kubos-hal-stm32f4
    ┗━ kubos-core 0.1.2 yotta_modules/kubos-core -> /home/vagrant/.kubos/kubos/kubos-core

Similarly, to see the dependencies of your target and any linked
resources, use the ``kubos target`` command.

::

    vagrant@vagrant:~/my-project$ kubos target
    stm32f407-disco-gcc 0.1.0 -> /home/vagrant/sharedOS/targets/target-stm32f407-disco-gcc
    kubos-arm-none-eabi-gcc 0.1.1 -> /home/vagrant/.kubos/kubos/targets/target-kubos-arm-none-eabi-gcc
    kubos-rt-gcc 0.1.0 -> /home/vagrant/.kubos/kubos/targets/target-kubos-rt-gcc
    kubos-gcc 0.1.1 -> /home/vagrant/.kubos/kubos/targets/target-kubos-gcc
