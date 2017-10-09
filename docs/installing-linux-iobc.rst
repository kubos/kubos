Installing KubOS Linux on an ISIS-OBC
=====================================

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
  
.. figure:: images/iOBC/etcher.png
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

.. figure:: images/iOBC/uboot_console.png
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
:doc:`working-with-the-iobc` guide.
