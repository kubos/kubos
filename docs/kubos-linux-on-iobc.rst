KubOS Linux on the ISIS iOBC
============================

Overview
--------

The goal of this document is to create a KubOS Linux installation for the iOBC
that can then run the satellite services (telemetry, payload communication,
etc) needed for the ISIS customers' missions.

The :doc:`Working with the iOBC <working-with-the-iobc>` doc can then be used to
create and load a user application on top of the new KubOS Linux install.

**Note:** Ideally, the user should never have to mess with the kernel
themselves. It should be pre-loaded onto the iOBC.

Software Components
-------------------

ISIS Bootloader
~~~~~~~~~~~~~~~

The ISIS bootloader lives in the very beginning of the NOR flash. It should come
pre-loaded on the board and should not need to be modified. It initializes the
memory hardware and then copies U-Boot into the SDRAM and starts its execution.

If for some reason this bootloader needs to be reloaded, the relevant
instructions can be found in section 8.1 of the *ISIS-OBC Quickstart Guide*.

U-Boot
~~~~~~

`Wiki <https://en.wikipedia.org/wiki/Das_U-Boot>`__

`Site Page <http://www.denx.de/wiki/U-Boot>`__

`Kubos U-Boot Repo <https://github.com/kubostech/uboot>`__

U-Boot, at the most basic level, is responsible for loading the kernel from the
SD card into the SDRAM. However, it also provides a basic OS and CLI which can
be used to configure and debug the kernel before it's loaded.

Additionally, we've made some changes to allow us to use it as a kernel upgrade
and recovery system. At boot time it will check for available upgrade packages
or a corrupted Linux kernel and will then upgrade or rollback the kernel and
rootfs as necessary.

Kernel
~~~~~~

Linux
^^^^^

`Version Overview <https://kernelnewbies.org/Linux_4.4>`__

We're using Linux 4.4. This is the current long-term support version (LTS) and
will be supported until early 2018.

Glibc
^^^^^

`Overview <https://www.gnu.org/software/libc/>`__

We use the standard GNU C library to build our toolchains. We are currently
building using v2.23.

BusyBox
^^^^^^^

`Overview <https://busybox.net/about.html>`__

BusyBox provides many of the common Linux console and shell commands, but in a
smaller package.

BuildRoot
~~~~~~~~~

`Overview <https://buildroot.uclibc.org/>`__

The current development tool for building all of the components required for
running embedded Linux. Using this allows us to pass in a basic configuration
file and then have all of the required packages and options brought in and
compiled automatically. This reduces the amount of time to configure KubOS
Linux for a new board.

SAM-BA
~~~~~~

`Product Page <http://www.atmel.com/tools/atmelsam-bain-systemprogrammer.aspx>`__

The software tool used to flash the kernel and components onto the iOBC.

Installation Process
--------------------

.. _build-os:

Build the OS Files
~~~~~~~~~~~~~~~~~~

**WARNING:** The OS files cannot be built using a synced folder in a Vagrant box (or regular VM).
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
the iOBC.

**Note:** You will need to build with ``sudo`` if you are using the default iOBC
configuration, since it points the output toolchain to "/usr/bin/iobc_toolchain",
which is a protected directory.

::

    $ sudo make BR2_EXTERNAL=../kubos-linux-build at91sam9g20isis_defconfig

Build everything

::

    $ make

The full build process will take a while. Running on a Linux VM, it took about
an hour. Running in native Linux, it took about ten minutes. Once this build
process has completed once, you can run other BuildRoot commands to rebuild
only certain sections and it will go much more quickly (<5 min).

BuildRoot documentation can be found
`**here** <https://buildroot.org/docs.html>`__

The generated files will be located in buildroot-2016.11/output/images. They are:

-  uboot.bin - The U-Boot binary
-  zImage - The compressed Linux kernel file
-  {board}.dtb - The Device Tree Binary that Linux uses to configure itself
   for your board
-  rootfs.tar - The root file system. Contains BusyBox and other libraries

Changing the Output Toolchain Directory
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you would like to build your toolchain in somewhere other than the
"/usr/bin/iobc_toolchain" directory, update the ``BR2_HOST_DIR`` variable in the
"configs/at91sam9g20isis_defconfig" file.

If you would like BuildRoot to just build the toolchain locally, you may remove
the ``BR2_HOST_DIR`` variable entirely. The toolchain will then be built under the
main "buildroot-2016.11" directory in a new "output/host" folder.

.. _install-sd:

Install the SD Card Files
~~~~~~~~~~~~~~~~~~~~~~~~~

Due to their size, the kernel and rootfs files live on the SD card.

Pre-Requisites
^^^^^^^^^^^^^^

In order to write the files to the SD card your build system needs be able to a)
see the SD card device and b) read/write to multiple partitions.

If you're running Mac OS or Windows, you'll need to pass the SD card through to
your Vagrant box.

-  `Mac OS X Instructions <https://www.geekytidbits.com/mount-sd-card-virtualbox-from-mac-osx/>`__
-  `Windows Instructions <http://rizwanansari.net/access-sd-card-on-linux-from-windows-using-virtualbox/>`__

If you're running Linux, you can either pass through the SD card to your Vagrant
box via the VirtualBox Manager, or run the whole build process natively.

Get the Device Name
^^^^^^^^^^^^^^^^^^^

To start, find the name of your SD card in your system:

::

    $ sudo fdisk -l

You should see a device that looks like this:

::

    Disk /dev/sdb: 3.8 GiB, 4025483264 bytes, 7862272 sectors
    Units: sectors of 1 * 512 = 512 bytes
    Sector size (logical/physical): 512 bytes / 512 bytes
    I/O size (minimum/optimal): 512 bytes / 512 bytes
    Disklabel type: dos
    Disk identifier: 0xf39e6ab1

In this example '/dev/sdb' is the name of the SD card. You might also see
'/dev/mmcblk0'. You'll need to use this name in all future commands.

Run the Formatting/Flashing Script
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Navigate to the 'kubos-linux-build/tools' directory.

Run the ``format-sd.sh`` script. You might need to run as root to get
permissions for certain steps.

The script has optional parameters: 

- ``-d {device}`` - Specify the name of the SD card device. The default is
  '/dev/sdb' 
- ``-s {size}`` - Size, in MB, of the SD card. The default is 4000 (4GB). 
- ``-w`` - Specify that the SD card should be wiped before formatting. Useful
  if there was any data previously on the card. **Note** Wiping a 4GB SD card
  takes about 10 minutes. 
- ``-p`` - Specify that existing kpack-base.itb and kernel files should be
  copied into the appropriate partitions 
- ``-pp`` - Specify that the kpack-base.itb and kernel files should be built
  and then copied to their partitions 
- ``-ppp`` - Specify that the SD card should not be formatted. Only build and
  copy the kpack and kernel files. 
- ``-b {branch}`` - Specify the branch name of U-Boot that has been built. The
  default is 'master'. This option should not need to be used outside of
  development.

So if I wanted to wipe my SD card and then build and load the new kernel files,
I would run:

::

    $ ./format-sd.sh -wpp

Once the script has finished successfully, the SD card is ready to be inserted
into the iOBC's SD Card 0 slot.

Manual Format/Flash Process
^^^^^^^^^^^^^^^^^^^^^^^^^^^

If for some reason you'd like to format the SD card and load the bare minimum
files onto it manually, follow this process.

**Partition the SD Card**

First, you'll need to set up the partitions on the SD card (for readability,
we'll be using device name '/dev/sdb'. Be sure to replace with the device name
of your SD card):

Create a partition table

::

    $ sudo parted /dev/sdb mklabel msdos y

Create the partitions

::

    $ sudo parted /dev/sdb mkpart primary linux-swap 1M 513M
    $ sudo parted /dev/sdb mkpart extended 513M 4000M
    $ sudo parted /dev/sdb mkpart logical fat16 513M 534M
    $ sudo parted /dev/sdb mkpart logical ext4 534M 555M
    $ sudo parted /dev/sdb mkpart logical ext4 555M 606M
    $ sudo parted /dev/sdb mkpart logical ext4 606M 4000M

Configure the partitions (ex. /dev/sdb1)

::

    $ sudo mkswap /dev/sdb1
    $ sudo mkfs.fat /dev/sdb5
    $ sudo mkfs.ext4 /dev/sdb6
    $ sudo mkfs.ext4 /dev/sdb7
    $ sudo mkfs.ext4 /dev/sdb8

**Create the Kernel File**

The BuildRoot build process creates the zImage file, which is a self-extracting
kernel image. In order to help detect corruption, we package that into an
\*.itb file, which includes a checksum value that can be validated during boot time.

Navigate to your 'kubos-linux-build' folder and open the 'tools' directory.

Run the ``kubos-kernel.sh`` script.

The script has optional parameters (which are unlikely to be needed): 

- ``-i {input-file}`` - Specify the name of the
  \*.its file to use. This file describes the files that will be packaged and their usage configuration options. The default is 'kubos-kernel.its', which should also be located in the 'tools' directory. 
-  ``-b {branch}`` - Specify the branch name of U-Boot that has been built.
   The default is 'master'. This option should not need to be used outside of
   development. 

The script will create the 'kubos-kernel.itb' file.

**Copy the files**

Next, you'll need to copy the kernel file into the boot partition and the rootfs
into the rootfs partition

From your project folder:

Create mount folders

::

    $ mkdir boot
    $ mkdir rootfs

Mount the partitions (replace '/dev/sdb' with the name of your SD card device)

::

    $ sudo mount /dev/sdb5 boot
    $ sudo mount /dev/sdb6 rootfs

Copy the kubos-kernel.itb file into partition 5. It will need to be renamed to
'kernel'.

::

    $ sudo cp buildroot-2016.11/output/images/kubos-kernel.itb boot/kernel

Untar the rootfs into partition 6

::

    $ sudo tar -xvf buildroot-2016.11/output/images/rootfs.tar -C rootfs

Unmount the partitions

::

    $ sudo umount /dev/sdb5
    $ sudo umount /dev/sdb6

Remove the SD card and insert it into iOBC SD card slot 0.

Install the NOR Flash Files
~~~~~~~~~~~~~~~~~~~~~~~~~~~

Pre-Requisites
^^^^^^^^^^^^^^

1. Obtain a SEGGER SAM-ICE programmer/debugger 
2. Install programming drivers from https://www.segger.com/jlink-software.html 
3. Install FTDI USB-to-serial drivers from http://www.ftdichip.com/Drivers/VCP.htm
4. Install SAM-BA (and PuTTY, if you don't already have it) from the
   ISIS-OBC SDK installer. (Refer to Section 3.3 of the ISIS-OBC Quick Start
   Guide)
5. Setup the iOBC board for serial connection and programming. (Refer to
   Chapter 4 of the ISIS-OBC Quick Start Guide)
6. Connect the programming and serial connection cables to your
   computer.
7. Power the board.

Note: Make sure the red jumper on the programming board is in place; it bypasses
the watchdog. If you don't, the board will continually reboot and you won't be
able to flash anything.

Boot into U-Boot (Optional)
^^^^^^^^^^^^^^^^^^^^^^^^^^^

(Skip this section if you've never put Linux on your board before)

If you already have Linux running on your board, you'll need to boot into the
U-Boot console rather than the Linux console in order to be able to flash the
board.

You'll need to establish a serial connection with the board in order to connect
to the console. Set up a serial connection to the board at a baudrate of 115200.

.. figure:: images/iOBC/putty_connection.png
   :alt: PuTTY Connection

   PuTTY Connection

Once the serial connection is open, boot (or reboot) the board. Hold down any
key while the board is starting up. This will exit out of the auto-boot and
bring up the CLI.

.. figure:: images/iOBC/uboot_console.png
   :alt: U-Boot Console

   U-Boot Console

Flash the Files
^^^^^^^^^^^^^^^

Start up SAM-BA. You'll want to select the at91sam9g20-ISISOBC option from the
'Select your board' drop-down.

.. figure:: images/iOBC/samba_connection_select.png
   :alt: SAM-BA Connection Selection

   SAM-BA Connection Selection

Execute the 'Enable NorFlash' script. This will prep the board to enable
flashing.

.. figure:: images/iOBC/samba_enable_norflash.png
   :alt: SAM-BA Enable NorFlash

   SAM-BA Enable NorFlash

Select the uboot.bin file in the 'Send File Name' field.

Make sure that 'Address' is set to 0xA000.

Click 'Send File'

.. figure:: images/iOBC/samba_send_uboot.png
   :alt: SAM-BA Send U-Boot

   SAM-BA Send U-Boot

Select the at91sam9g20isis.dtb file in the 'Send File Name' field (you'll need
to view all file types in order to see the .dtb file)

Set 'Address' to 0x80000.

Click 'Send File'

.. figure:: images/iOBC/samba_send_dtb.png
   :alt: SAM-BA Send DTB

   SAM-BA Send DTB

Reboot the System
^^^^^^^^^^^^^^^^^

After new files have been loaded, the board will need to be powered off and back
on again in order to go through the normal boot process.

Status LEDs
-----------

There are four LEDs present on the iOBC which give some indication of what state
the board is in:

-  Three LEDS (solid) - The system is currently running U-Boot
-  Two LEDs (blinking) - The system is currently running KubOS Linux

Connect to the System
---------------------

You should now be able to set up a serial connection to your board and interact
with the KubOS Linux environment.

You'll need to establish a serial connection with the board in order to connect
to the console. Set up a serial connection to the board at a baudrate of 115200.

.. figure:: images/iOBC/putty_connection.png
   :alt: PuTTY Connection

   PuTTY Connection

You should see the console boot into Linux like this:

.. figure:: images/iOBC/linux_console.png
   :alt: Linux Console

   Linux Console

Upgrade Process
---------------

If you already have KubOS Linux installed on your system, but would like to
upgrade to the latest version, check out the :ref:`upgrade-installation` section. 
Alternatively, if you would like to rollback to a previously installed version, 
refer to the :ref:`upgrade-rollback` section.

Recovery Process
----------------

Should your KubOS Linux kernel become corrupted (as indicated by failing to
successfully boot into Linux several times), the system will automatically try
to recover during the next boot.

It will go through the following steps, if each is present (system will reboot
after attempting each step):

1. Reload the current version of KubOS Linux from the kpack\*.itb file
   in the upgrade partition
2. Reload the previous version of KubOS Linux from the kpack\*.itb file
   in the upgrade partition
3. Reload the base version of KubOS Linux from the kpack-base.itb file
   in the upgrade partition
4. Boot into the alternate OS

If none of these steps work, then the system will boot into the U-Boot CLI. From
here, some basic troubleshooting and debugging abilities should be available.

More information about the recovery process and architecture can be found in the
:doc:`KubOS Linux Recovery doc <kubos-linux-recovery>`
