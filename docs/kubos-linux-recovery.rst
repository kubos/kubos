KubOS Linux Recovery Architecture
=================================

Ideally, a space mission using KubOS Linux will run successfully until the end of its natural lifespan. However, space is unpredictable and systems can become corrupted. As a result, we've implemented a basic mechanism to help detect and recover from these random errors.

Overview
--------

Each time the system attempts to boot, an internal counter is increased. If the KubOS Linux kernel successfully boots, it will reset this internal counter.

If the system has failed to boot twice already, then the custom Kubos recovery code is attempted. If the Kubos recovery steps fail, then the system attempts to boot into an alternate operating system instead. If that fails, then the system will stop attempting to load an OS and will instead just present the U-Boot CLI.

Environment Variables
---------------------

There are several U-Boot environment variables that are used to track the state of a KubOS Linux system: 

* `bootcmd` - The usual set of commands that are used to boot into KubOS Linux. 
* `altbootcmd` - The alternate set of commands that are used if the system cannot successfully boot into KubOS Linux. They will be set up to attempt to boot into an alternate OS. 
* `recovery_available` - Indicates that recovery actions are available and should be taken, if necessary. 
* `bootlimit` - The number of bad boots allowed before the system attempts to use altbootcmd instead of bootcmd to boot. 
* `bootcount` - The number of boots that have been attempted. 
* `kubos_curr_version` - The name of the kpack \*.itb file that the current KubOS Linux kernel and rootfs were loaded from. 
* `kubos_prev_version` - The name of the kpack \*.itb file that was previously used to load the KubOS Linux kernel and rootfs.
* `kubos_curr_tried` - Indicates that reloading the current version has been attempted.

The default values for these variables can be found in the configuration header file for each board located in the `U-Boot repo <https://github.com/kubostech/uboot>`__ under the 'include/configs' directory.

Boot Processing Diagram
-----------------------

.. figure:: images/kubos_linux_recovery.png
   :alt: Boot Processing Diagram

   Boot Processing Diagram

Kubos Recovery
--------------

The Kubos recovery process has three main components: 

* Attempt to reload the current version of KubOS Linux 
* Attempt to load the previous version of KubOS Linux 
* Attempt to load the base version of KubOS Linux

The boot count will not be increased again until this full recovery process is determined to have failed.

All of the files required for this process live in the board's 'upgrade' partition. The base version of KubOS Linux (kpack-base.itb) should be pre-loaded into the partition. The current and previous versions are loaded into the partition as part of the system upgrade process. These versions follow a natural process.

Brand new KubOS Linux system:

::

    kubos_curr_version = kpack-base.itb
    kubos_prev_version = kpack-base.itb

After the first system upgrade:

::

    kubos_curr_version = kpack-upgrade1.itb
    kubos_prev_version = kpack-base.itb

After the second system upgrade:

::

    kubos_curr_version = kpack-upgrade2.itb
    kubos_prev_version = kpack-upgrade1.itb

Rolling back to a previous version of KubOS Linux uses the same mechanism as :doc:`upgrading to a new version <kubos-linux-upgrade>`. A kpack\*.itb file is broken into its components and then the kernel image is written to the boot partition and the rootfs image is written to the rootfs partition.

**Note** This process will wipe out everything that was previously in the rootfs partition. As a result, all user files should be stored in the userspace partition, which is mapped to the '/home' directory. This userspace partition should not be affected by the Kubos recovery process.

Manual Recovery
~~~~~~~~~~~~~~~

If for some reason your KubOS Linux system boots after an upgrade but has introduced some non-critical issue (like an incompatibility with a user application), you can manually rollback to a previously installed version. Previous packages are not deleted once they have been loaded. As a result, you can simply specify which package you would like to boot into and then restart your system. List the contents of the '/upgrade' directory to see what files are available.

From the KubOS Linux shell:

::

    $ ls /upgrade
        kpack-2017.02.20.itb    kpack-2017.03.21.itb    kpack-2017.04.11.itb
    $ fw_printenv kubos_updatefile kpack-{desired version}.itb
    $ reboot

Alternate Boot
--------------

If the system has failed to boot more times than the 'bootlimit' value allows, the system will attempt to boot using the 'altbootcmd' environment variable. 
This variable contains all of the commands required to sanitize the current boot environment and then, if it has been set up, boot into an alternate operating system. 
Due to the low-portability of any commands that deal with memory, the exact format will change between boards (and potentially between customers), but should follow this rough format:

-  Set the 'recovery\_available' variable to 0 (if we succesfully boot into an alternate OS, it should reset this back to 1. If we fail to boot into the alternate OS, then we should not keep attempting.)
-  Clear the 'bootcmd' variable. If 'recovery\_available' is 0 and 'bootcmd' is NULL, then the system won't attempt to boot into anything and will instead just go to the U-Boot CLI. The hope is that from here some manual troubleshooting and recovery can occur.
-  Save the U-Boot envars. The ``saveenv`` command saves any local environment variables changes to persistent storage.
-  If an alternate OS has been setup on the board:

  -  Copy the alternate OS from persistent storage into SDRAM.
  -  Run the alternate OS from SDRAM.

-  Otherwise, continue to the U-Boot CLI

This is the default alternate boot value:

::

    altbootcmd=setenv recovery_available 0; setenv bootcmd; saveenv

Generic Alternate OS Setup
~~~~~~~~~~~~~~~~~~~~~~~~~~

The basic process for creating an alternate OS and loading it onto a board
should be:  

* Build an application that is capable of running on the board. Pay attention to the SDRAM address that the application is configured to run from. Frequently, this is a static address (likely the very beginning of SDRAM), so the application must end up running from this location. 
* Load it into the appropriate persistent storage (NOR/NAND flash, SD card, etc) 
* Update the altbootcmd variable with the address to copy the application from, the address to copy the application to, and the length of the application. 
  Then add a command to trigger the boot process. This can be done from the U-Boot CLI with the ``setenv`` and ``saveenv`` commands, or from KubOS Linux with the ``fw_setenv`` command.

The updated altbootcmd might look something like this:

::

    altbootcmd=setenv recovery_available 0; setenv bootcmd; saveenv; cp.b 0x10080000 0x20000000 0x70000; go 0x20000000

This command will go through the default alternate boot commands and then:

  - Copy 0x7000 bytes from address 0x10080000 (a permanent storage location) to address 0x20000000 (the beginning of SDRAM)
  - Use the ``go`` command to attempt to boot from address 0x20000000 (``go`` is used for generic executables)
  
U-Boot CLI
----------

`U-Boot CLI Documentation <http://www.denx.de/wiki/DULG/UBootCommandLineInterface>`__

The U-Boot CLI provides a few commands which may be helpful for manually diagnosing and recovering from system problems. It has a very limited functionality, but should be better than nothing.

If you want to avoid booting into an operating system for any reason and instead want to interact with the U-Boot CLI, you can abort the boot by creating a serial connection and then holding down any key while powering the board. This action will not increase the boot count.
