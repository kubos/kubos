Porting KubOS to a New OBC
==========================

The KubOS middleware and applications are intended to be able to run on any OBC which is running
Kubos Linux. Unfortunately, while userland programs are completely portable between OBCs, the
underlying operating system and bootloader are not.

This doc is intended as a high-level overview of the work which needs to be done in order to get
KubOS up and running on a new OBC.

Note: The terms "board" and "OBC" are used interchangeably.

Resources
---------

Internal
~~~~~~~~

- :doc:`../ecosystem/linux-docs/kubos-linux-overview`
- :doc:`../deep-dive/klb/configuring-kubos`
- :doc:`Building KubOS <../deep-dive/klb/kubos-linux-on-bbb>`

External
~~~~~~~~

- `Buildroot <https://buildroot.org/downloads/manual/manual.html>`__
- `U-Boot <http://www.denx.de/wiki/U-Boot>`__
- `Device Trees <https://www.devicetree.org/>`__
- `Busybox <https://busybox.net/about.html>`__

System Requirements
-------------------

We recommend that any board being targeted for Kubos Linux have **at least** the following resources:

- 400MHz processor with MMU
- 64MB RAM (32MB is sufficient if Python is disabled)
- 400MB storage for OS and upgrade partition (100MB is sufficient if Python is disabled)

System Layout
-------------

Before any work can be done, the system's partition layout should be decided.

The system should be composed of at least the following partitions:

- Boot partition - Contains U-Boot, the kernel, and the device tree (may be broken into multiple
  partitions)
- U-Boot envar partition - The environment variables which U-Boot uses in order to load and boot
  Kubos Linux. We recommend that this be placed in its own partition in order to help prevent
  corruption.
- Root FS - The Kubos Linux root file system
- Upgrade partition - This will hold the Kubos :doc:`upgrade/rollback <../ecosystem/linux-docs/kubos-linux-upgrade>`
  files
- User data partition - This will hold all of the user data. It is placed in its own partition to
  prevent it from being affected by a system upgrade or rollback.

These partitions may be placed across multiple memory devices, if desired.
For example, the partition containing U-Boot could be placed in a more secure memory device (ex.
flash storage), while the root FS could be placed in a larger, but less secure location since it can
be easily recovered.

Buildroot
---------

Buildroot is responsible for building and assembling the OS components based on a given
configuration file.

Kubos' Buildroot configurations and files are kept in the `KLB repo <https://github.com/kubos/kubos-linux-build>`__
and follow Buildroot's recommended `external tree structure <https://buildroot.org/downloads/manual/manual.html#outside-br-custom>`__.

Clone the `KLB repo <https://github.com/kubos/kubos-linux-build>`__ repo to your local development
environment.
Almost all work done to port KubOS to a new OBC will be done here.

Board Config
~~~~~~~~~~~~

The high-level configuration file for the new OBC should be placed in `kubos-linux-build/configs`
and should have a name which ends in "\*_defconfig".

The `kubos-linux-build/configs/generic-kubos_defconfig` file gives a good starting place for your
configuration. The "{target}" string should be replaced with your OBC's name.

The Kubos :doc:`core services <../ecosystem/services/core-services>` are included by default, so do
not need to be manually specified in this file.

Board Directory
~~~~~~~~~~~~~~~

All other files needed for your OBC should be placed in a new directory under the
`kubos-linux-build/board` directory.

The normal convention is `board/{company}/{obc}`.

You may add your OBC under the `kubos` directory or you may create a new directory for your
organization.

Files in this directory include:

- U-Boot configuration
- Linux configuration
- Linux device tree
- Image generation
- Board-specific overlay
- Board-specific patches

These files will be covered in more detail in later sections of this doc.

Overlay
~~~~~~~

Board-specific overlay files should be given in a subdirectory, `board/{company}/{obc}/overlay`.

These files should be located under the same directories as the file target file system.

There are a few overlay files which should be common to all boards.

etc/network/interfaces
^^^^^^^^^^^^^^^^^^^^^^

This file defines the default ethernet connection (if one exists) as well as any other network
connections. For example, a :ref:`SLIP connection <slip>` might also be defined.

etc/fstab
^^^^^^^^^

This file defines all of the partitions which should be mounted at system boot.

etc/fw_env.config
^^^^^^^^^^^^^^^^^

This file defines the location of the U-Boot envar partition so that the envars can be accessed
from Linux with the ``fw_printenv`` and ``fw_setenv`` commands.

etc/inittab
^^^^^^^^^^^

This file is responsible for running some boot-time activities and kicking off all the init scripts.

Most importantly, it should be customized to run ``fsck`` on all system partitions and then to
mount all relevant partitions.

etc/monitrc
^^^^^^^^^^^

This file controls the settings of `Monit <https://mmonit.com/monit/documentation/monit.html>`__,
the system's :doc:`process monitoring <../ecosystem/linux-docs/monitoring>` tool.

This file is customized for each board primarily due to the ``SET HTTPD`` command, which varies
depending on whether or not the system has an ethernet connection available.

Image Creation
~~~~~~~~~~~~~~

Once the build process has been completed, a final OS image will likely need to be created so it
can be loaded onto your target memory device/s.

For the Beaglebone Black and Pumpkin MBM2 targets, this is controlled by the `post-image.sh` script,
which calls ``genimage`` in order to create the image.
It uses the `genimage.cfg` file in order to determine which partitions need to be created, what
size the partitions need to be, and which files need to be placed inside of them.

`genimage <https://github.com/pengutronix/genimage>`__ is our preferred tool used to create system
images, however it might not be compatible with all board layouts.

U-Boot
------

`U-Boot <http://www.denx.de/wiki/U-Boot>`__ is the bootloader which is used for all boards.
It is responsible for loading the operating system files into the appropriate storage locations and
then kicking off the OS boot process.

U-Boot configuration is a very manual process.
The easiest way to determine what settings need to be used is to find example boards which are as
close to your desired architecture as possible (frequently things like a processor's evaluation kit
board are available).

Kubos has created a fork of U-Boot at https://github.com/kubos/uboot.
When adding a new board, users may do one of three things:

- Create a pull request which adds support for their board to Kubos' U-Boot repo
- Create a patch which adds support and store it in their board's Buildroot directory
- Create a custom fork of U-Boot

Buildroot Config
~~~~~~~~~~~~~~~~

A good portion of the U-Boot configuration is done with a configuration file, located in the board's
Buildroot directory.
This file defines the high-level capabilities and the behavior of the U-Boot prompt.

The following options should be enabled in order to build the Kubos OS recovery and
upgrade system into the U-Boot binary:

- ``CONFIG_UPDATE_KUBOS``
- ``CONFIG_DFU``
- ``CONFIG_DFU_TFTP``
- The ``CONFIG_DFU_*`` options which match the memory device type/s you are using (ex. ``CONFIG_DFU_MMC``)

U-Boot Config
~~~~~~~~~~~~~

The remainder of a board's configuration is done within U-Boot itself.

You'll need to create a new configuration header file in `uboot/include/configs`.

This header file will define things like the location of the U-Boot envars, the default values for
those envars, and the location and properties of various system resources.

The file should have ``#include "kubos-common.h"`` in order to build in the resources needed for
OS upgrade and recovery.

U-Boot Board Package
~~~~~~~~~~~~~~~~~~~~

Next, you'll need to create a new directory under `uboot/board`.
Boards currently supported by Kubos are located under `uboot/board/kubos`.

Within this directory should be at least two files:

- Kconfig - Defines the new board-specific configuration options, including a pointer to the
  previously mentioned configuration header file (``SYS_BOARD``)
- Makefile - Defines the board-specific drivers which need to be compiled into U-Boot

Installing U-Boot
~~~~~~~~~~~~~~~~~

Special care should be taken when determining where the final U-Boot binary should be installed.

Many boards' initial bootloaders expect the starting executable (U-Boot, in this case) to be located
in a particular memory location.

Note: This same care is not required for installing the rest of the system since you'll be defining
the location of the other major components (kernel, root FS, etc) within U-Boot.

Linux
-----

Config
~~~~~~

The Buildroot configuration supports having multiple "fragment" files for Linux configuration
(``BR2_LINUX_KERNEL_CONFIG_FRAGMENT_FILES``).
We take advantage of that by specifying common Linux options within the
`kubos-linux-build/common/linux-kubos.config` file.

Board-specific options should be specified within a separate config file under the board's
Buildroot directory.
These options include things like model-specific peripheral drivers and processor-specific
definitions.

Device Tree
~~~~~~~~~~~

The board's device tree defines the particular hardware characteristics of the board.
This includes things like specifying the pins allocated to a SPI bus, and the address of
a particular bank of memory.

Device tree development is one of the major pain points when bringing up a new OBC.

We recommend the following debug tactics:

1. Turn your compiled device tree (\*.dtb) back into the source tree to make sure that it's getting
   assembled the way you want it to. ``dtc -I dtb -O dts {buildroot}/output/images/{board}.dtb``
2. Start up Linux with debug printing enabled (Note: this will generate a huge amount of data, so
   you're going to want to have it automatically saved off somewhere for you to review later):

   - Power up your board and hold down a key to go into the U-Boot console
   - Enter ``editenv bootargs``
   - Add ``debug`` to the end of the printed string and then press Enter
   - Enter ``run bootcmd``
   - This will start up Linux and spew out all kinds of stuff. Once it's done booting (probably a
     minute or so), you can review the startup data. You'll be looking for any kinds of issues
     assigning the desired pins to a particular device or loading the needed driver for a peripheral.

Busybox
-------

Currently, all OBCs supported by Kubos use a common Busybox configuration, located in
`kubos-linux-build/common/busybox-kubos.config`.
This config file specifies all the commands and utilities which are needed in order to run KubOS.

Additional config fragment files may be specified, if desired, with the
``BR2_PACKAGE_BUSYBOX_CONFIG_FRAGMENT_FILES`` option.
