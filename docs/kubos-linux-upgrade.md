# Upgrading KubOS Linux

[TOC]

# Upgrading KubOS Linux {#kubos-linux-upgrade}

## Overview {#overview}

KubOS Linux upgrades are distributed as kpack-{version}.itb files. ITB stands for 'Image Tree Blob' and allows Kubos to utilize the existing DFU utility currently available in U-Boot.

Within each file will be a new version of the kernel image and root filesystem.

To upgrade a board currently running KubOS Linux, an upgrade package will be loaded into the upgrade partition of the board. For now, this can be done through the Kubos SDK or by 
manually copying the package into the upgrade partition.

Once the board is rebooted, U-Boot will take the package and then install each component into the appropriate partition (kernel/rootfs). If installation completes successfully, then
the board will be rebooted and then boot into the new version of KubOS Linux.

Note: User files should not be impacted by upgrade installation as long as they remain under the /home directory. This directory maps to the userspace partition.

The overall flow looks like this:

                                     +--------------------------+
                                     |                          |
                                     |          Boot            <-------------------+
                                     |                          |                   |
                                     +--------------------------+                   |
                                     |                          |                +--+---+
                                     |         RootFS           <--------+       |zImage|
                                     |                          |        |       +--+---+
                                     +--------------------------+        |          |
                                     |                          |        |          |
                                     |        User Data         |   +----+-----+    |
                                     |                          |   |rootfs.img|    |
                                     +--------------------------+   +----+-----+    |
                                     |                          |        |          |
                                     |         Upgrade          |        |          |
                                     |                          |        |          |
    +--------------------------+     | +----------------------+ |        |          |
    |                          |     | |                      +----------+          |
    | External Source Location +------>| kpack-2017.03.03.itb | |                   |
    |                          |     | |                      +---------------------+
    +--------------------------+     | +----------------------+ |
                                     |                          |
                                     +--------------------------+


## Upgrade Installation {#upgrade-installation}

### Pre-requisites

The SD card should have been formatted with the correct partitions. If not, refer to the 'Installation Process->Partition the SD Card' section
of the [KubOS Linux on the ISIS iOBC](docs/kubos-linux-on-iobc.md) doc.

The host computer should be connected to the board and the iOBC should be on and running KubOS Linux. 

A Kubos SDK VM should be installed on your host computer and atleast one shared folder should be set up.  Installation instructions can be found [here](docs/sdk-installing.md).

### Installation

Acquire an upgrade package. For now, this will likely be sent via email from a Kubos engineer.  Once an official distribution process has been created
this document will be upgraded with the new procedure.

Load the package into a shared folder accessible by your Kubos SDK VM.

Create or navigate to a Kubos SDK project.  The content of the project does not matter; it will only be used to flash the package correctly onto the target.
    
    $ kubos init -l fakeproj
    $ cd fakeproj
    $ kubos build

Set the target to the iOBC.

    $ kubos target kubos-linux-isis-gcc
    
Build the project. This does not need to complete successfully. The build process just brings in some files and settings that are required in order to flash files to the board.

    $ kubos build

Use the kubos flash command to load the package onto your board. Note: You might need to update your config.json file with the appropriate login information to access your board.  See the 'Updating Credentials' section of [this document](docs/user-app-on-iobc.md)
    for more information. 
    
    $ kubos flash /home/vagrant/shared/kpack-{version}.itb
    
Wait for the transfer to complete. This can take roughly 30 minutes. The transfer rate via serial connection is slow because a) the upgrade packages are large and b) it runs some CRC functions during
transfer to ensure that the package does not become corrupted. You should see a progress bar indicating the time remaining for the transfer.

    TODO: Add a progress bar...And then show an example here.
    
Refer to the Troubleshooting section of the [User Applications on the ISIS iOBC](User Applications on the ISIS iOBC) document if anything goes wrong with the transfer.

Once the transfer has completed successfully, trigger a reboot of the iOBC. This can be done with the Linux `reboot` command. Once job scheduling has been implemented, you will be
able to schedule the desired reboot time. 

When the board boots into U-Boot, the new package will be detected and loaded. If the loading is successful, the board will reboot into the newly installed KubOS Linux. The U-Boot console messages
will look similar to this:

    Processing upgrade 'zImage@1' :crc32+ sha1+ 
    ###writing zImage
    1154936 bytes written
    Processing upgrade 'rootfs@1' :crc32+ sha1+ 
    ##########################Un-Protected 1 sectors
    Erasing Flash...
    . done
    Erased 1 sectors
    Writing to Flash... done
    Protected 1 sectors
    resetting ...
    reset_cpu

## Upgrade Creation {#upgrade-creation}

This section is for developers who have made changes to KubOS Linux and want to generate an upgrade package.

### Pre-requisite 

Build the new OS.
Refer to the 'Installation Process->Build the OS Files' section of the [KubOS Linux on the ISIS iOBC](docs/kubos-linux-on-iobc.md) doc.

Make sure /usr/bin/iobc_toolchain is in your PATH. If you're building from a Kubos SDK VM, it should have been automatically added.
Otherwise, you may need to manually add it. The U-Boot `mkimage` tool requires `dtc` which is built into the toolchain.

### Run the Packaging Script

From the kubos-linux-build folder, run the kubos-package.sh script. This will create the rootfs.img file and then use the kpack.its file
to bundle the rootfs.img and zImage files into an *.itb file. This is the file that will be distributed to customers when an upgrade is needed.

The automatically generated naming convention for the package is kpack-_yyyy_-_mm_-_dd_.itb

#### Custom Packages

If you'd like to customize the package, there are a few different options available through the script:

- -s : Sets the size of the rootfs.img file, specified in KB. The default is 13000 (13MB).
- -i : Sets the name and location of the input *.its file. Use if you want to create a custom package.  The default is _kpack.its_.
- -v : Sets the version information for the package. The output file will be kpack-{version}.itb.

For example:

    $ ./kubos-package.sh -s 15000 -i /home/test/custom.its -v 2.0

### Distribute the Package

There isn't currently a central storage location or procedure for upgrade packages, since no official packages have been created yet. This
section should be upgraded once something has been implemented. 

## Upgrade Rollback {#upgrade-rollback}

If for some reason you need to rollback to a previous version of KubOS Linux, you don't need to reflash the board with the correct upgrade package.
Previous packages are not deleted once they have been loaded. As a result, you can simply specify which package you would like to boot into and then 
restart your system.

From the KubOS Linux shell:

    $ fw_printenv kubos_updatefile kpack-{desired version}.itb
    $ reboot
