Installing KubOS Linux on a Pumpkin Motherboard Module 2
========================================================

The KubOS Linux installation process is composed of two high-level steps:

  - Flashing the eMMC
  - Flashing the microSD card
    
To perform a full default installation, two files are needed:

  - A KubOS Linux SD card image
  - An aux_sd image
  
All of these files can be obtained from `our KubOS Linux Releases page on GitHub <https://github.com/kubostech/kubos-linux-build/releases>`__

Download the latest `KubOS_Linux.zip` file and then unzip the files for the Pumpkin MBM2. They're located in the `KubOS_Linux/{version}/Pumpin-MBM2` folder.

Pre-Requisites
--------------

1. Obtain an SD card that is at least 4GB.

.. note:: 

    The KubOS Linux SD images are created for a 4GB SD card. The image can be applied to a larger SD card, but the
    resulting system will still only have 4GB of space available to it.

 
2. Install `Etcher <https://etcher.io/>`__. Other software to flash SD cards does exist,
   but Etcher is the Kubos software of choice.

3. Obtain the SD card images

Install the eMMC Image
----------------------

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
into the Pumpkin MBM2's microSD slot.

Boot into U-Boot
~~~~~~~~~~~~~~~~

.. note:: These instructions should work whether you're currently running KubOS Linux
    or some other Linux distribution.

We now want to overwrite the eMMC, so we'll need to use U-Boot in order to boot
KubOS Linux from the SD card.

You'll need to establish a serial connection with the board in order to connect
to the console. 

Hold down any key while the board is booting. This will exit out of the auto-boot and
bring up the CLI.

::

    U-Boot 2016.09 (Jul 17 2017 - 11:43:29 -0500)

    I2C:   ready
    DRAM:  512 MiB
    MMC:   OMAP SD/MMC: 0, OMAP SD/MMC: 1
    Net:   cpsw, usb_ether
    Hit any key to stop autoboot:  0 
    U-Boot>
   
Copy/paste these commands:

::
    
    setenv bootargs console=ttyS0,115200 root=/dev/mmcblk0p2 ext4 rootwait; fatload mmc 0:1 ${fdtaddr} /pumpkin-mbm2.dtb; fatload mmc 0:1 ${loadaddr} /kernel; bootm ${loadaddr} - ${fdtaddr}
    
This will cause the board to load KubOS Linux off of the microSD card, allowing us to flash
the eMMC.

Flash the eMMC
~~~~~~~~~~~~~~

To flash the eMMC, log into the board and then run these commands:

::

    $ umount /home/microsd
    $ umount /home
    $ dd if=/dev/mmcblk0 of=/dev/mmcblk1
    
The four status LEDs on the board should start flashing in a random pattern. This indicates
that the eMMC is currently being flashed. 

The process should take roughly ten minutes, after which the LEDs should return to normal, 
with one LED blinking to indicate a successfully running KubOS Linux system.

After this has completed, shutdown and de-power the system.

Install the Auxiliary Image
---------------------------

Re-Flash the SD Card
~~~~~~~~~~~~~~~~~~~~

Now flash the micro SD card with the auxiliary SD card image. This image contains the
KubOS Linux upgrade partition and the second user data partition.

Once the flash process has completed, put the card back into the microSD slot.

.. warning::

    If you do not have a microSD card in the board, the system will not boot.

The installation process is now complete.

Using KubOS Linux
-----------------

For information on how to create and run applications on your new KubOS Linux system, see the
:doc:`working-with-the-mbm2` guide.
