Kubos SDK Cheatsheet
====================

Creating a Project
------------------

Run the ``kubos init`` command followed by the name of your project to
bootstrap your Kubos project. This will create a new directory under
your current working directory with your project's name and add the
source files for a basic Kubos project (kubos-rt-example).

::

        $ kubos init project-name # creates a KubOS RT project

        $ kubos init -l linux-project-name # creates a KubOS Linux project

**Note:** Inside of the build system there are several reserved words,
which cannot be used as the name of the project. These are ``test``,
``source``, ``include``, ``yotta_modules`` and ``yotta_targets``.

The contents of your project directory should look something like this:

::

        $ ls
        CONTRIBUTING LICENSE.txt module.json README source yotta_modules yotta_targets

Here is a quick rundown of the files that were generated:

+-------------------+-------------------------------------------------------------------------------------------+
| File/folder       | Description                                                                               |
+===================+===========================================================================================+
| `project-name`    | This folder is where header files live                                                    |
+-------------------+-------------------------------------------------------------------------------------------+
| `source`          | This folder is where source files live                                                    |
+-------------------+-------------------------------------------------------------------------------------------+
| `test`            | This folder is where test source files live                                               |
+-------------------+-------------------------------------------------------------------------------------------+
| `module.json`     | This file is yotta's module description file                                              |
+-------------------+-------------------------------------------------------------------------------------------+
| `yotta_modules`   | This directory holds the symlinks for the project's module dependencies                   |
+-------------------+-------------------------------------------------------------------------------------------+
| `yotta_targets`   | This directory holds the symlinks for the available Kubos targets                         |
+-------------------+-------------------------------------------------------------------------------------------+
| `CONTRIBUTING.md` | The doc outlining the process of contributing to a Kubos project                          |
+-------------------+-------------------------------------------------------------------------------------------+
| `LICENSE.txt`     | The software license associated with the example project                                  |
+-------------------+-------------------------------------------------------------------------------------------+
| `README.md`       | The readme for the example project that outlines some of the basic details of the example |
+-------------------+-------------------------------------------------------------------------------------------+

Kubos uses the yotta build/module system, which is where this file
structure comes from. You can read more about yotta
`here <http://yottadocs.mbed.com/>`__.

.. _selecting-a-target:

Selecting a Target
------------------

Kubos needs to know which target you intend to build for so it can
select the proper cross compiler. Kubos currently supports several
different targets:

+------------+------------------------------+---------------------------------------------------+--------------+
| MCU Family | Kubos Target                 | Description                                       | Supported OS |
+============+==============================+===================================================+==============+
| STM32F4    | stm32f407-disco-gcc          | STM32F407 Discovery Board                         | KubOS RT     |
+------------+------------------------------+---------------------------------------------------+--------------+
|            | pyboard-gcc                  | STM32F405 PyBoard                                 | KubOS RT     |
+------------+------------------------------+---------------------------------------------------+--------------+
|            | na-satbus-3c0-gcc            | STM32F405 NanoAvionics SatBus 3c0 OBC             | KubOS RT     |
+------------+------------------------------+---------------------------------------------------+--------------+
| MSP430     | msp430f5529-gcc              | MSP430F5529 Launchpad                             | KubOS RT     |
+------------+------------------------------+---------------------------------------------------+--------------+
| ISIS       | kubos-linux-isis-gcc         | ISIS-OBC                                          | KubOS Linux  |
+------------+------------------------------+---------------------------------------------------+--------------+
| Pumpkin    | kubos-linux-pumpkin-mbm2-gcc | Pumpkin Motherboard Module 2                      | KubOS Linux  |
+------------+------------------------------+---------------------------------------------------+--------------+  
| Beaglebone | kubos-linux-beaglebone-gcc   | Beaglebone Black, Rev. C                          | KubOS Linux  |
+------------+------------------------------+---------------------------------------------------+--------------+
|            | legacy-linux-beaglebone-gcc  | Beaglebone Black running a non-Kubos Linux distro | Debian       |
+------------+------------------------------+---------------------------------------------------+--------------+
| (Vagrant)  | x86-linux-native             | Native target for the Kubos Vagrant image         | KubOS Linux  |
+------------+------------------------------+---------------------------------------------------+--------------+

To select a target, use the ``kubos target`` command with the appropriate value from the
"Kubos Target" column. 

For example

::

        $ kubos target stm32f407-disco-gcc

To see all of the available targets run:

::

        $ kubos target --list

Building a Project
------------------

To build a KubOS project, all we need to do is run the ``kubos build``
command. The Kubos CLI will read the module.json file, determine what
libraries are needed and build them.

Basic build command:

::

        $ kubos build

Build with verbose output:

::

        $ kubos build -- -v

**Note:** The Kubos CLI commands have their own specific arguments that
can be used. There are also global arguments (like ``--verbose`` or
``-v``) a double hyphen ``--`` separates the command specific arguments
from the global arguments

Clean command:

::

        $ kubos clean

To build a project from scratch run ``kubos clean`` to remove all
remaining files generated for previous builds followed by
``kubos build``.

Linking Local Modules and Targets
---------------------------------

The Kubos SDK comes with all of the latest Kubos modules and targets
pre-packaged and pre-linked. If a module or target needs to be modified
locally, the CLI comes with the ability to link that local module into
the build process.

Modules and Targets
^^^^^^^^^^^^^^^^^^^

Modules are groups of source code that implement a feature or unit of
functionality. Kubos operating systems are split into a number of
modules. An example of a Kubos module is the `Kubos
HAL <https://github.com/kubostech/kubos/tree/master/hal/kubos-hal>`__

Targets are groups of configuration files that allow toolchains to build
and cross-compile modules for specific hardware targets. One example of
a Kubos target is the `STM32F407 Discovery
Target <https://github.com/kubostech/kubos/tree/master/targets/target-stm32f407-disco-gcc>`__

Linking Modules:
^^^^^^^^^^^^^^^^

Links are made in two steps - first globally then locally.

By linking a module globally you are making it available to link into
any of your projects. By linking the module locally you are including
the linked module in your build.

-  To link a module globally:

   ::

       $ cd .../<module-directory>/
       $ kubos link

-  To link a module that is already globally linked into a project:

   ::

       $ cd .../<project-directory>/
       $ kubos link <module name>

The next time your project is built it will use your local development
module, rather than the packaged version.

**Note:** To verify where all of your targets are being loaded from
``kubos list`` will show you which modules are linked and which are
local to your project

Linking Targets:
^^^^^^^^^^^^^^^^

Custom or modified targets are linked in a very similar way to modules.

Links are made in two steps - first globally then locally.

By linking a target globally you are making it available to link into
any of your projects. By linking the target locally you are now able to
use the linked target in your build.

-  To link a target globally:

   ::

       $ cd .../<target-directory>/
       $ kubos link-target

-  To link a target that is already globally linked into a project:

   ::

       $ cd .../<project-directory>/
       $ kubos link-target <target name>

-  You may now use the standard target command to select the newly
   linked target:

   ::

       $ cd ../<project-directory>/
       $ kubos target <target name>

The next time your project is built it will use your local development
target, rather than the packaged version.

**Note:** Running ``kubos target`` will show you whether you are using a
local or a linked copy of a target

Flashing your Project
---------------------

Flashing your project using the kubos tool is a relatively
straightforward process:

1. Ensure that your board is plugged into your computer. Running the
   following command will list all of the available devices in your
   Kubos SDK box.

   ::

       $ lsusb

2. Run the flash command

   ::

       $ kubos flash

*Note: If your current user does not have read/write permission to your
hardware device you may need to run this command as root*

::

        $ sudo kubos flash

Debugging your Project
----------------------

A gdb server must be started to allow your gdb instance to connect and
debug directly on your hardware device. After building your project with
``kubos build`` the kubos-cli can start a gdb server and gdb instance
for you.

Start a gdb server and instance: **Note:** This may need to run as root
depending on your USB device permissions

::

        $ kubos debug

If the debug command is successful you will be prompted with a gdb
instance attached to your device and ready to debug!

**Note:** The ``kubos debug`` command is not yet implemented for KubOS
Linux projects.
