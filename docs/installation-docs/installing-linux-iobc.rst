Installing KubOS Linux on an ISIS-OBC
=====================================

Overview
--------

This document covers the steps required to install KubOS Linux onto an iOBC.

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

-  :doc:`../linux-docs/kubos-linux-on-iobc` - Steps to build KubOS Linux
-  :doc:`../linux-docs/first-linux-project` - Basic tutorial for creating your first KubOS
   Linux SDK project
-  :doc:`../sdk-docs/sdk-cheatsheet` - Overview of the common Kubos SDK commands
-  :doc:`../linux-docs/using-kubos-linux` - General guide for interacting with KubOS Linux
-  :doc:`../linux-docs/working-with-the-iobc` - Guide for interacting with iOBC-specific features

Components
----------

The KubOS Linux installation process is composed of two high-level steps:

  - Flashing the SD card
  - Flashing the on-board NOR flash
    
To perform a default installation, three files are needed:

  - A KubOS Linux SD card image
  - u-boot.bin
  - at91sam9g20isis.dtb
  
All of these files can be obtained from `our KubOS Linux Releases page on GitHub <https://github.com/kubostech/kubos-linux-build/releases>`__

Download the latest `KubOS_Linux.zip` file and then unzip the files for the iOBC. They're located in the `KubOS_Linux/{version}/iOBC` folder.

.. _install-sd:

Install the SD Card Files
-------------------------

All users should install the SD card files using a distributed KubOS Linux image, unless they have
created a custom KubOS Linux build. In that case, the SD card files can be installed by either 
flashing a complete KubOS Linux image onto an SD card or :ref:`by alternate means <alt-sd-setup>`.

Pre-Requisites
~~~~~~~~~~~~~~

1. Obtain an SD card that is at least 4GB.

.. note:: 

    The KubOS Linux SD images are created for a 4GB SD card. The image can be applied to a larger SD card, but the
    resulting system will still only have 4GB of space available to it.

 
2. Install `Etcher <https://etcher.io/>`__. Other software to flash SD cards does exist,
   but Etcher is the Kubos software of choice.

3. Obtain a KubOS Linux image


Flash the SD Card
~~~~~~~~~~~~~~~~~

Using `Etcher <https://etcher.io/>`__:

  - Select the KubOS Linux image to flash
  - Make sure the SD card device is correct (may be auto-detected if there is only one SD card present
    in your system.)
  - Click the "Flash!" button to start the flashing process
  
.. figure:: ../images/iOBC/etcher.png
   :alt: Etcher Setup

   Etcher Setup
  
It should take roughly 10 minutes for a 4GB image to be loaded onto an SD card.

Once the program has finished successfully, the SD card is ready to be inserted
into the iOBC's SD Card 0 slot.

Install the NOR Flash Files
---------------------------

The NOR flash files will be loaded onto the iOBC using the Atmel SAM-BA software.

This can be done by using the provided command line script or :ref:`using the SAM-BA GUI <alt-nor-setup>`.

The SD card does not need to be inserted into the iOBC in order for this step to work.

.. warning::

    **The SAM-BA software currently only supports using the SAM-ICE JTAG with host machines
    running Windows. This means that you must use a Windows OS in order to initially flash
    the iOBC.**
    
    Once KubOS Linux has been installed, the device tree, which is located in the NOR flash,
    can be updated using the standard :ref:`upgrade-installation` process with a `kpack-nor-*.itb`
    file.

Pre-Requisites
~~~~~~~~~~~~~~

1. Obtain an `Atmel SAM-ICE programmer/debugger <http://www.atmel.com/tools/atmelsam-ice.aspx>`__.
2. Install programming drivers from https://www.segger.com/jlink-software.html.
3. Install FTDI USB-to-serial drivers from http://www.ftdichip.com/Drivers/VCP.htm
4. Install SAM-BA from the ISIS-OBC SDK installer. 
   (Refer to Section 3.3 of the `ISIS-OBC Quick Start Guide`)
   
   **Note:** You must use the ISIS version of SAM-BA, rather than the default
   Atmel installation. It includes several configuration files that are required
   to connect to the iOBC.
5. Setup the iOBC board for serial connection and programming. (Refer to
   Chapter 4 of the `ISIS-OBC Quick Start Guide`)
6. Connect the programming and serial connection cables to your
   computer.

.. warning::

    Make sure the red jumper on the programming board is in place; it bypasses
    the watchdog. If you don't, the board will continually reboot and you won't be
    able to flash anything.
    
7. Turn on the board.

8. Copy the `kubos-nor-flash.tcl` script from the `tools/at91sam9g20isis` folder in
   the `kubos-linux-build <https://github.com/kubostech/kubos-linux-build>`__ repo
   into the SAM-BA application folder.
9. Change line 44 in `{path to SAM-BA}/tcl_lib/boards.tcl` from this:

   ``"at91sam9g20-ISISOBC"    "at91sam9g20-ISISOBC/at91sam9g20-ISISOBC.tcl"``
   
   to this:
   
   ``"at91sam9g20-isisobc"    "at91sam9g20-ISISOBC/at91sam9g20-ISISOBC.tcl"``
   
   (the SAM-BA application converts everything to lower case, which will lead to 
   a "board not found" error if you don't change this file)


Boot into U-Boot
~~~~~~~~~~~~~~~~

**(Skip this section if you've never put Linux on your board before)**

If you already have Linux running on your board, you'll need to boot into the
U-Boot console rather than the Linux console in order to be able to flash the
board.

You'll need to establish a serial connection with the board in order to connect
to the console. 

You can do this via a Kubos Vagrant image with the ``minicom kubos`` command
after booting the board.

The default login information for an iOBC is kubos/Kubos123.

Issue the ``reboot`` command in order to restart the system.

Hold down any key while the board is restarting. This will exit out of the auto-boot and
bring up the CLI.

.. figure:: ../images/iOBC/uboot_console.png
   :alt: U-Boot Console

   U-Boot Console
   
The board is now ready to be flashed.

Flash the Files
~~~~~~~~~~~~~~~

The flashing script can be called from the standard command prompt using this command:

::

    $ {path to SAM-BA}/sam-ba.exe \jlink\ARM0 at91sam9g20-ISISOBC
          {path to SAM-BA}/kubos-nor-flash.tcl {input arguments} [> {logfile}]
    
Where the input arguments are as follows:

  - uboot={uboot file} - Path to U-Boot binary
  - dtb={dtb file} - Path to Device Tree binary
  - altos={alt file} - Path to alternate OS binary
  
Multiple input arguments can be specified and should be space-separated.
  
The optional logfile parameter is highly recommended, as the SAM-BA application will not
give any other response to this command. The log file will contain all of the output as the 
script connects to the board and transfers the files.

Example command:

::

    $ C:/ISIS/applications/samba/sam-ba.exe /jlink/ARM0 at91sam9g20-ISISOBC 
          C:/ISIS/applications/samba/kubos-nor-flash.tcl uboot=new-u-boot.bin dtb=new-dtb.dtb 
          > logfile.log
 
If you'd like to confirm that the command ran successfully, open the log file. You should see
this message for each file you attempted to flash:

    ``Sent file & Memory area content (address: [...], size: [...] bytes) match exactly !``

Reboot the System
-----------------

If you have not already done so, insert the SD card into the iOBC's first SD card
slot while the board is **not powered**.

After new files have been loaded, the board will need to be powered off and back
on again in order to go through the normal boot process.

Using KubOS Linux
-----------------

For information on how to create and run applications on your new KubOS Linux system, see the
:doc:`../linux-docs/working-with-the-iobc` guide.


Non-Default Installation Process
--------------------------------

There are alternate ways to install KubOS Linux onto the board, in case you want to create a custom
installation, or are having issues with the default installation work flow.

.. _alt-sd-setup:

Alternate SD Card Setup
~~~~~~~~~~~~~~~~~~~~~~~

If you do not have a KubOS Linux image, you can load the required files onto an SD card:

  - by using our flashing script
  
    or
  
  - manually

Pre-Requisites
^^^^^^^^^^^^^^

Since you are not using a KubOS Linux image, you will need to go through the :ref:`OS build process <build-os>`
locally in order to create the kernel and rootfs files.

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

Method 1: Run the Formatting/Flashing Script
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A script is available to format the SD card and then load the latest
KubOS Linux kernel and root filesystem.

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

Method 2: Manual Format/Flash Process
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If for some reason you'd like to format the SD card and load the bare minimum
files onto it manually, follow this process.

Partition the SD Card
#####################

First, you'll need to set up the partitions on the SD card (for readability,
we'll be using device name '/dev/sdb'. Be sure to replace with the device name
of your SD card):

Create a partition table

::

    $ sudo parted /dev/sdb mklabel msdos y

Create the partitions

::

    $ sudo parted /dev/sdb mkpart primary ext4 4M 3856M
    $ sudo parted /dev/sdb mkpart extended 3856M 3996M
    $ sudo parted /dev/sdb mkpart logical fat16 3856M 3876M
    $ sudo parted /dev/sdb mkpart logical ext4 3876M 3896M
    $ sudo parted /dev/sdb mkpart logical ext4 3896M 3996M

Configure the partitions (ex. /dev/sdb1)

::

    $ sudo mkfs.ext4 /dev/sdb1
    $ sudo mkfs.fat /dev/sdb5
    $ sudo mkfs.ext4 /dev/sdb6
    $ sudo mkfs.ext4 /dev/sdb7

Create the Kernel File
######################

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

Copy the files
##############

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

    $ sudo cp buildroot-2017.02.8/output/images/kubos-kernel.itb boot/kernel

Untar the rootfs into partition 6

::

    $ sudo tar -xvf buildroot-2017.02.8/output/images/rootfs.tar -C rootfs

Unmount the partitions

::

    $ sudo umount /dev/sdb5
    $ sudo umount /dev/sdb6

Remove the SD card and insert it into iOBC SD card slot 0.

.. _alt-nor-setup:

Alternate NOR Flash Setup
~~~~~~~~~~~~~~~~~~~~~~~~~

You must still use the Atmel SAM-BA software on a Windows machine in order to flash the required files
into the iOBC NOR flash. However, you can use the SAM-BA software directly to do so, rather than using
the previously provided script.

Pre-Requisites
^^^^^^^^^^^^^^

1. Obtain an `Atmel SAM-ICE programmer/debugger <http://www.atmel.com/tools/atmelsam-ice.aspx>`__.
2. Install programming drivers from https://www.segger.com/jlink-software.html.
3. Install FTDI USB-to-serial drivers from http://www.ftdichip.com/Drivers/VCP.htm
4. Install SAM-BA from the ISIS-OBC SDK installer. 
   (Refer to Section 3.3 of the `ISIS-OBC Quick Start Guide`)
   
   **Note:** You must use the ISIS version of SAM-BA, rather than the default
   Atmel installation. It includes several configuration files that are required
   to connect to the iOBC.
5. Setup the iOBC board for serial connection and programming. (Refer to
   Chapter 4 of the `ISIS-OBC Quick Start Guide`)
6. Connect the programming and serial connection cables to your
   computer.

.. warning::

    Make sure the red jumper on the programming board is in place; it bypasses
    the watchdog. If you don't, the board will continually reboot and you won't be
    able to flash anything.

7. Turn on the board.
    
8. Obtain the NOR flash files either from Kubos, or from your own :ref:`local build <build-os>`:
    
    - u-boot.bin
    - at91sam9g20isis.dtb    
    
Boot into U-Boot
^^^^^^^^^^^^^^^^

**(Skip this section if you've never put Linux on your board before)**

If you already have Linux running on your board, you'll need to boot into the
U-Boot console rather than the Linux console in order to be able to flash the
board.

You'll need to establish a serial connection with the board in order to connect
to the console. 

You can do this via a Kubos Vagrant image with the ``minicom kubos`` command
after booting the board.

The default login information for an iOBC is kubos/Kubos123.

Issue the ``reboot`` command in order to restart the system.

Hold down any key while the board is restarting. This will exit out of the auto-boot and
bring up the CLI.

.. figure:: ../images/iOBC/uboot_console.png
   :alt: U-Boot Console

   U-Boot Console

The board is now ready to be flashed.    
    
Installation
^^^^^^^^^^^^

Start SAM-BA
############

Start up SAM-BA. You'll want to select the at91sam9g20-ISISOBC option from the
'Select your board' drop-down.

.. figure:: ../images/iOBC/samba_connection_select.png
   :alt: SAM-BA Connection Selection

   SAM-BA Connection Selection

Enable Flashing
###############

Execute the 'Enable NorFlash' script. This will prep the board to enable
flashing.

.. figure:: ../images/iOBC/samba_enable_norflash.png
   :alt: SAM-BA Enable NorFlash

   SAM-BA Enable NorFlash

Flash U-Boot
############

Select the uboot.bin file in the 'Send File Name' field.

Make sure that 'Address' is set to 0xA000.

Click 'Send File'

.. figure:: ../images/iOBC/samba_send_uboot.png
   :alt: SAM-BA Send U-Boot

   SAM-BA Send U-Boot
   
Click 'Compare sent file with memory' after the file transfer has completed to confirm
that all data was sent successfully.

Flash Device Tree
#################

Select the at91sam9g20isis.dtb file in the 'Send File Name' field (you'll need
to view all file types in order to see the .dtb file)

Set 'Address' to 0x70000.

Click 'Send File'

.. figure:: ../images/iOBC/samba_send_dtb.png
   :alt: SAM-BA Send DTB

   SAM-BA Send DTB
   
Click 'Compare sent file with memory' after the file transfer has completed to confirm
that all data was sent successfully.

Reboot the System
~~~~~~~~~~~~~~~~~

After new files have been loaded, the board will need to be powered off and back
on again in order to go through the normal boot process.
