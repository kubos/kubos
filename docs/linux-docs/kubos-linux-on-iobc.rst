Building KubOS Linux for the ISIS-OBC
=====================================

Overview
--------

This supplementary document covers specific features and components of KubOS Linux for the ISIS-OBC.

The :doc:`kubos-linux-overview` doc covers the major components of KubOS Linux.

Additionally, this document covers the steps required in order to build KubOS Linux.

Reference Documents
-------------------

iOBC Documentation
~~~~~~~~~~~~~~~~~~

The :title:`ISIS-OBC Quickstart Guide` should have been packaged with the iOBC
and is a useful document for learning what each of the hardware
components are, how to connect them, and what drivers need to be
installed to support them.

Kubos Documentation
~~~~~~~~~~~~~~~~~~~

-  :doc:`../installation-docs/installing-linux-iobc` - Steps to install KubOS Linux on an iOBC
-  :doc:`using-kubos-linux` - General guide for interacting with KubOS Linux
-  :doc:`working-with-the-iobc` - Guide for interacting with iOBC-specific features

Software Components
-------------------

ISIS Bootloader
~~~~~~~~~~~~~~~

The ISIS bootloader lives in the very beginning of the NOR flash. It should come
pre-loaded on the board and should not need to be modified. It initializes the
memory hardware and then copies U-Boot into the SDRAM and starts its execution.

If for some reason this bootloader needs to be reloaded, the relevant
instructions can be found in section 8.1 of the *ISIS-OBC Quickstart Guide*.

SAM-BA
~~~~~~

`Product Page <http://www.atmel.com/tools/atmelsam-bain-systemprogrammer.aspx>`__

The software tool used to program the iOBC's NOR flash storage area.

.. note:: 

    The ISIS-OBC SDK includes the SAM-BA application. You should install this version,
    rather than the default Atmel version, since it is packaged with several iOBC configuration
    files which are required to successfully connect to the board.

KubOS Linux Build Process
-------------------------

If for some reason you want or need to modify and rebuild the KubOS Linux components, follow
the steps in this section.

.. _build-os:

Build the OS Files
~~~~~~~~~~~~~~~~~~

.. warning::

    The OS files cannot be built using a `synced folder <https://www.vagrantup.com/docs/synced-folders/>`__ in a Vagrant box (or regular VM).
    VirtualBox does not support hard links in shared folders, which are crucial in order to complete
    the build.
    
:doc:`SSH into a Kubos SDK box <../installation-docs/sdk-installing>`

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
the iOBC.

.. note::

    You will need to build with ``sudo`` if you are using the default iOBC
    configuration, since it points the output toolchain to "/usr/bin/iobc_toolchain",
    which is a protected directory.

::

    $ sudo make BR2_EXTERNAL=../kubos-linux-build at91sam9g20isis_defconfig

Build everything

::

    $ sudo make

The full build process will take a while. Running on a Linux VM, it took about
an hour. Running in native Linux, it took about ten minutes. Once this build
process has completed once, you can run other BuildRoot commands to rebuild
only certain sections and it will go much more quickly (<5 min).

BuildRoot documentation can be found
`**here** <https://buildroot.org/docs.html>`__

The generated files will be located in buildroot-2016.11/output/images. They are:

-  uboot.bin - The U-Boot binary
-  zImage - The compressed Linux kernel file
-  at91sam9g20isis.dtb - The Device Tree Binary that Linux uses to configure itself
   for the iOBC
-  rootfs.tar - The root file system. Contains BusyBox and other libraries

Changing the Output Toolchain Directory (optional)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you would like to build your toolchain in somewhere other than the
"/usr/bin/iobc_toolchain" directory, update the ``BR2_HOST_DIR`` variable in the
"configs/at91sam9g20isis_defconfig" file.

If you would like BuildRoot to just build the toolchain locally, you may remove
the ``BR2_HOST_DIR`` variable entirely. The toolchain will then be built under the
main "buildroot-2016.11" directory in a new "output/host" folder.

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

Create an SD Card Image
~~~~~~~~~~~~~~~~~~~~~~~

.. note::

    The image script will use 4GB of system RAM during execution. By default,
    the Kubos Vagrant box only provides 2GB. As a result, you'll either need to `increase
    the amount of RAM provided to your box 
    <https://askubuntu.com/questions/510134/how-to-increase-vm-hdd-and-ram-sizes>`__,
    or run the script natively.

A script is available to create an SD card image with the latest
KubOS Linux kernel and root filesystem.

Navigate to the 'kubos-linux-build/tools' directory.

Run the ``format-image.sh`` script. You might need to run as root to get
permissions for certain steps.

The script has optional parameters: 

- ``-d {device}`` - Sets the SD card device name to flash the newly created image to
  (does not flash by default)
- ``-i {name}`` - Specifies the output file name of the image file to be created.
  (default: "kubos-linux.img")
- ``-p`` - Specify that existing kpack-base.itb and kernel files should be
  copied into the appropriate partitions 
- ``-pp`` - Specify that the kpack-base.itb and kernel files should be built
  and then copied to their partitions 
- ``-ppp`` - Only build and copy the kpack and kernel files. Skip all other steps. 
- ``-s {size}`` - Size, in MB, of the SD card. The default is 3800 (~4GB). 
- ``-b {branch}`` - Specify the branch name of U-Boot that has been built. The
  default is 'master'. This option should not need to be used outside of
  development.

So if I wanted to create a custom-named image with brand new kernel files,
I would run:

::

    $ ./format-image.sh -i kubos-linux-v1.0.img -pp

Create an Upgrade Package
~~~~~~~~~~~~~~~~~~~~~~~~~

If you would like to distribute your changes as a Kubos upgrade package instead,
please refer to the :ref:`upgrade-creation` instructions.

