Building Kubos Linux for the Beaglebone Black
=============================================

Overview
--------

This supplementary document covers specific features and components of Kubos Linux for the Beaglebone Black.

The :doc:`../../ecosystem/linux-docs/kubos-linux-overview` doc covers the major components of Kubos Linux.

Additionally, this document covers the steps required in order to build Kubos Linux.

Reference Documents
-------------------

Beaglebone Documentation
~~~~~~~~~~~~~~~~~~~~~~~~

- `Beaglebone Black Web Page <https://beagleboard.org/black>`__
- `Beaglebone Black Wiki <http://elinux.org/Beagleboard:BeagleBoneBlack>`__
- `Beaglebone Black Hardware Diagrams <http://beagleboard.org/Support/bone101/#hardware>`__
- `Beaglebone Black System Reference Manual Rev C <http://static6.arrow.com/aropdfconversion/8fff89aa85f5c451318cbdee2facd9c9fac36872/bbb_srm.pdf>`__

Kubos Documentation
~~~~~~~~~~~~~~~~~~~

-  :doc:`../../obc-docs/bbb/installing-linux-bbb` - Steps to install Kubos Linux
-  :doc:`../../ecosystem/linux-docs/using-kubos-linux` - General guide for interacting with Kubos Linux
-  :doc:`../../obc-docs/bbb/working-with-the-bbb` - Guide for interacting with BBB-specific features

Software Components
-------------------

ROM Bootloader
~~~~~~~~~~~~~~

The ROM bootloader lives in a small section of ROM space. It should come
pre-loaded on the board and should not need to be modified. It selects the
next bootloader depending on whether the boot mode button is being held.
If not held, it attempts to run the next boot step from eMMC storage;
otherwise, it attempts to boot from the microSD card.

U-Boot
~~~~~~
This board utilizes U-Boot's SPL feature. A small boot file called "MLO" is
run and that file then loads the main U-Boot image into SDRAM.

The main U-Boot image iterates through the `boot_targets` variable to attempt
to boot from an available MMC device. The partuuid of the first successful
device is passed off to Linux to be used to mount the root filesystem.

By default, the microSD card slot will be checked first, followed by the
eMMC. This behavior can be changed by setting the ``boot_dev`` value to
``1`` to indicate that the eMMC should be tried first.

Kubos Linux Build Process
-------------------------

If for some reason you want or need to modify and rebuild the Kubos Linux components, follow
the steps in this section.

.. note::

    Kubos Linux should be built from within an instance of the Kubos SDK or some other native Linux
    environment.

.. _build-os-bbb:

Build the OS Files
~~~~~~~~~~~~~~~~~~

.. warning::

    The OS files cannot be built using a `synced folder <https://www.vagrantup.com/docs/synced-folders/>`__ in a Vagrant box (or regular VM).
    VirtualBox does not support hard links in shared folders, which are crucial in order to complete
    the build.
    
:doc:`SSH into a Kubos SDK box <../../sdk-docs/sdk-installing>`

In order to build Kubos Linux, two components are needed:

- The `kubos-linux-build repo <https://github.com/kubos/kubos-linux-build>`__ - Contains the configurations, patches, and extra tools needed to build Kubos Linux
- `BuildRoot <https://buildroot.org/>`__ - The actual build system

These components should be setup as children of the same parent directory.
There are several commands and variables in the build process which use relative file paths to navigate between the components.

After the environment has been set up, all build commands will be run from the BuildRoot directory unless otherwise stated.

To set up a build environment and build Kubos Linux:

Create a new parent folder to contain the build environment

::

    $ mkdir kubos-linux

Enter the new folder

::

    $ cd kubos-linux

Download BuildRoot-2019.02.2 (more current versions of BuildRoot may work as well,
but all testing has been done against 2019.02.2)

.. note:: All Kubos documentation will refer to v2019.02.2, which is the latest version of the LTS release at the time of this writing.

::

    $ wget https://buildroot.uclibc.org/downloads/buildroot-2019.02.2.tar.gz && tar xvzf buildroot-2019.02.2.tar.gz && rm buildroot-2019.02.2.tar.gz

Pull the kubos-linux-build repo

::

    $ git clone http://github.com/kubos/kubos-linux-build

Move into the buildroot directory

::

    $ cd buildroot-2019.02.2

Point BuildRoot to the external kubos-linux-build folder and tell it to build
for the Beaglebone Black.

.. note::

    You will need to build with ``sudo`` if you are using the default 
    configuration, since it points the output toolchain to "/usr/bin/bbb_toolchain",
    which is a protected directory.

::

    $ sudo make BR2_EXTERNAL=../kubos-linux-build beaglebone-black_defconfig

Build everything

::

    $ sudo make

The full build process will take a while. Running on a Linux VM, it takes about
an hour. Running in native Linux, it took about ten minutes. Once this build
process has completed once, you can run other BuildRoot commands to rebuild
only certain sections and it will go much more quickly (<5 min).

BuildRoot documentation can be found
`**here** <https://buildroot.org/docs.html>`__

The generated files will be located in buildroot-2019.02.2/output/images.
The relevant files are:

-  uboot.bin - The U-Boot binary
-  kernel - The compressed Linux kernel file
-  beaglebone-black.dtb - The Device Tree Binary that Linux uses to configure itself
   for the Beaglebone Black board
-  rootfs.tar - The root file system. Contains BusyBox and other libraries
-  kubos-linux.tar.gz - A compressed file containing the complete Kubos Linux SD card
   image, ``kubos-linux.img``. It has a disk signature of 0x4B4C4E58 ("KLNX").
-  aux-sd.tar.gz - A compressed file containing the auxilliary SD card image which
   contains the upgrade partition and the ``kpack-base.itb`` file which is used for
   OS recovery. It has a disk signature of 0x41555820 ("AUX ").

The `kubos-linux.tar.gz` and `aux-sd.tar.gz` files are the two final files which will be used to
install Kubos Linux onto your target board.

Changing the Output Toolchain Directory (optional)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you would like to build your toolchain in somewhere other than the
"/usr/bin/bbb_toolchain" directory, update the ``BR2_HOST_DIR`` variable in the
"configs/bbb_defconfig" file.

If you would like BuildRoot to just build the toolchain locally, you may remove
the ``BR2_HOST_DIR`` variable entirely. The toolchain will then be built under the
main "buildroot-2019.02.2" directory in a new "output/host" folder.

Using Kubos Linux
-----------------

For information on how to create and run applications on your new Kubos Linux system, see the
:doc:`../../obc-docs/bbb/working-with-the-bbb` guide.

Configuring Kubos Linux
-----------------------

For information on how to customize your build of Kubos Linux, see the
:doc:`configuring-kubos` guide.

This guide covers things like including custom packages, enabling hardware services, and selecting
a non-default version of the KubOS source.