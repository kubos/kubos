KubOS Linux on the Beaglebone Black
===================================

Overview
--------

This supplementary document covers specific features and components of KubOS Linux for the Beaglebone Black.

The :doc:`kubos-linux-overview` doc covers the major components of KubOS Linux.

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

KubOS Linux Build Process
-------------------------

If for some reason you want or need to modify and rebuild the KubOS Linux components, follow
the steps in this section.

.. _build-os-bbb:

Build the OS Files
~~~~~~~~~~~~~~~~~~

.. warning::

    The OS files cannot be built using a `synced folder <https://www.vagrantup.com/docs/synced-folders/>`__ in a Vagrant box (or regular VM).
    VirtualBox does not support hard links in shared folders, which are crucial in order to complete
    the build.

Create new folder

::

    $ mkdir kubos-linux

Enter the new folder

::

    $ cd kubos-linux

Download BuildRoot-2016.11 (more current versions of BuildRoot may work as well,
but all testing has been done against 2016.11)

::

    $ wget https://buildroot.uclibc.org/downloads/buildroot-2016.11.tar.gz && tar xvzf buildroot-2016.11.tar.gz && rm buildroot-2016.11.tar.gz

Pull the kubos-linux-build repo

::

    $ git clone http://github.com/kubostech/kubos-linux-build

Move into the buildroot directory

::

    $ cd buildroot-2016.11

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

The generated files will be located in buildroot-2016.11/output/images.
The relevant files are:

-  uboot.bin - The U-Boot binary
-  kernel - The compressed Linux kernel file
-  beaglebone-black.dtb - The Device Tree Binary that Linux uses to configure itself
   for the Beaglebone Black board
-  rootfs.tar - The root file system. Contains BusyBox and other libraries
-  kubos-linux.img - The complete KubOS Linux SD card image

Changing the Output Toolchain Directory (optional)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you would like to build your toolchain in somewhere other than the
"/usr/bin/bbb_toolchain" directory, update the ``BR2_HOST_DIR`` variable in the
"configs/bbb_defconfig" file.

If you would like BuildRoot to just build the toolchain locally, you may remove
the ``BR2_HOST_DIR`` variable entirely. The toolchain will then be built under the
main "buildroot-2016.11" directory in a new "output/host" folder.

Create auxilliary SD Card Image
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

By default, the build process will create a bootable SD card image. This will be flashed
onto the eMMC. In order to create a full KubOS Linux setup, you'll want to also create
an auxiliary image for the microSD card containing the upgrade partition and an additional
user data partition.

Follow the :ref:`upgrade-creation` instructions in order to create a Kubos Package file
(kpack-\*.itb) to be used for recovery.

Then, from the `kubos-linux-build/tools` folder, run the ``format-aux.img`` script. 
This will create a new SD card image, `aux-sd.img`, with two partitions:
- An upgrade partition containing `kpack-base.itb`
- A user data partition

There are two parameters which may be specified:

-  -s : Sets the size of the aux-sd.img file, specified in MB. The default is 3800 (3.8GB)
-  -i : Specifies the name and location of the kpack-\*.itb file to use as kpack-base.itb

For example:

::

    $ ./format-aux.sh -i ../kpack-2017.07.21.itb

Reset the Global Links
~~~~~~~~~~~~~~~~~~~~~~

If you run a full build, the links to all the Kubos SDK modules will be changed to
point at modules within the buildroot directory. As a result, you will be unable
to build any future Kubos SDK projects as a non-privileged user.

To fix this, run these commands:

::

    $ cd $HOME/.kubos/kubos/tools
    $ ./kubos_link.py
    
Depending on the state of your Kubos SDK project, you might also need to change the
module links locally:

::

    $ cd {project folder}
    $ kubos link -a

Using KubOS Linux
-----------------

For information on how to create and run applications on your new KubOS Linux system, see the
:doc:`working-with-the-bbb` guide.
