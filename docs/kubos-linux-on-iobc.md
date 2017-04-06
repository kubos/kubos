# KubOS Linux on the ISIS iOBC

[TOC]

# KubOS Linux on the ISIS iOBC {#kubos-linux-on-the-iobs}

## Overview {#overview}

The goal of this document is to create a KubOS Linux installation for the iOBC that can then run the satellite services (telemetry, payload communication, etc) 
needed for the ISIS customers' missions.

The [User Applications on the ISIS iOBC](docs/user-app-on-iobc.md) doc can then be used to create and load a user application on top of the new KubOS Linux install.

**Note:** Ideally, the user should never have to mess with the kernel themselves.  It should be pre-loaded onto the iOBC.

## Software Components {#software-components}

### ISIS Bootloader

The ISIS bootloader lives in the very beginning of the NOR flash. It should come pre-loaded on the board and should not need to be modified. It 
initializes the memory hardware and then copies U-Boot into the SDRAM and starts its execution.

### U-Boot

[Wiki](https://en.wikipedia.org/wiki/Das_U-Boot)

[Site Page](http://www.denx.de/wiki/U-Boot)

[Kubos U-Boot Repo](https://github.com/kubostech/uboot)

U-Boot, at the most basic level, is responsible for loading the kernel from the SD card into the SDRAM. However, it also provides a basic OS and CLI
which can be used to configure and debug the kernel before it's loaded.

Additionally, we've made some changes to allow us to use it as a kernel upgrade and recovery system. At boot time it will check for available upgrade packages or a corrupted Linux kernel and will then upgrade or rollback the kernel and rootfs as necessary.

### Kernel

#### Linux

[Version Overview](https://kernelnewbies.org/Linux_4.4)

We're using Linux 4.4. This is the current long-term support version (LTS) and will be supported until early 2018.

#### Glibc

[Overview](https://www.gnu.org/software/libc/)

We use the standard GNU C library to build our toolchains. We are currently building using v2.23.

#### BusyBox

[Overview](https://busybox.net/about.html)

BusyBox provides many of the common Linux console and shell commands, but in a smaller package.

### BuildRoot

[Overview](https://buildroot.uclibc.org/)

The current development tool for building all of the components required for running embedded Linux.  Using this allows us to 
pass in a basic configuration file and then have all of the required packages and options brought in and compiled automatically.
This reduces the amount of time to configure KubOS Linux for a new board.

### SAM-BA

[Product Page](http://www.atmel.com/tools/atmelsam-bain-systemprogrammer.aspx)

The software tool used to flash the kernel and components onto the iOBC.

## Installation Process {#installation-process}

### Build the OS Files {#build-the-os-files}

Create new folder

    $ mkdir kubos-linux

Enter the new folder

    $ cd kubos-linux
  
Download BuildRoot-2016.11 (more current versions of BuildRoot may work as well, but all testing has been done against 2016.11)

    $ wget https://buildroot.uclibc.org/downloads/buildroot-2016.11.tar.gz && tar xvzf buildroot-2016.11.tar.gz && rm buildroot-2016.11.tar.gz
  
Pull the kubos-linux-build repo

    $ git clone http://github.com/kubostech/kubos-linux-build
  
Move into the buildroot directory

    $ cd buildroot-2016.11
  
Point BuildRoot to the external kubos-linux-build folder and tell it to build the iOBC

    $ make BR2_EXTERNAL=../kubos-linux-build at91sam9g20isis_defconfig
  
Build everything

    $ make
  
The full build process will take a while.  Running on a Linux VM, it took about an hour.  Running in native Linux, it took about
ten minutes.  Once this build process has completed once, you can run other BuildRoot commands to rebuild only certain sections
and it will go much more quickly (\<5 min).

BuildRoot documentation can be found [**here**](https://buildroot.org/docs.html)

The generated files will be located in buildroot-2016.11/output/images.  They are:

- uboot.bin   - The U-Boot binary
- zImage      - The compressed Linux kernel file
- {board}.dtb - The Device Tree Binary that Linux uses to configure itself for your board
- rootfs.tar  - The root file system.  Contains BusyBox and other libraries

### Install the SD Card Files {#install-the-sd-card-files}

Due to their size, the kernel and rootfs files live on the SD card.

#### Pre-Requisites

In order to write the files to the SD card your build system needs be able to a) see the SD card device and b) read/write to multiple partitions.

If you're running Mac OS or Windows, you'll need to pass the SD card through to your Vagrant box.

* [Mac OS X Instructions](https://www.geekytidbits.com/mount-sd-card-virtualbox-from-mac-osx/)
* [Windows Instructions](http://rizwanansari.net/access-sd-card-on-linux-from-windows-using-virtualbox/)

If you're running Linux, you can either pass through the SD card to your Vagrant box via the VirtualBox Manager, or run the whole build process
natively.

#### Get the Device Name

To start, find the name of your SD card in your system:

    $ sudo fdisk -l
    
You should see a device that looks like this:

    Disk /dev/sdb: 3.8 GiB, 4025483264 bytes, 7862272 sectors
    Units: sectors of 1 * 512 = 512 bytes
    Sector size (logical/physical): 512 bytes / 512 bytes
    I/O size (minimum/optimal): 512 bytes / 512 bytes
    Disklabel type: dos
    Disk identifier: 0xf39e6ab1
    
In this example '/dev/sdb' is the name of the SD card.  You might also see '/dev/mmcblk0'.  You'll need to use this name in all future commands.

#### Run the Formatting/Flashing Script

Navigate to your 'kubos-linux-build' folder and open the 'tools' directory.

Run the `format-sd.sh` script. You might need to run as root to get permissions for certain steps.

The script has optional parameters:
* `-d {device}` - Specify the name of the SD card device. The default is '/dev/sdb'
* `-s {size}` - Size, in MB, of the SD card. The default is 4000 (4GB).
* `-w` - Specify that the SD card should be wiped before formatting. Useful if there was any data previously on the card. ** Note ** Wiping a 4GB SD card takes about 10 minutes.
* `-p` - Specify that existing kpack-base.itb and kernel files should be copied into the appropriate partitions
* `-pp` - Specify that the kpack-base.itb and kernel files should be built and then copied to their partitions
* `-ppp` - Specify that the SD card should not be formatted. Only build and copy the kpack and kernel files.
* `-b {branch}` - Specify the branch name of U-Boot that has been built. The default is 'master'. This option should not need to be used outside of development.

So if I wanted to wipe my SD card and then build and load the new kernel files, I would run:

    $ ./format-sd.sh -wpp
    
Once the script has finished successfully, the SD card is ready to be inserted into the iOBC's SD Card 0 slot.

#### Manual Format/Flash Process

If for some reason you'd like to format the SD card and load the bare minimum files onto it manually, follow this process.

##### Partition the SD Card

First, you'll need to set up the partitions on the SD card ({name} is the name of the disk device. Ex. /dev/sdb):

Create a partition table

    $ sudo parted {name} mklabel msdos y
    
Create the partitions 

    $ sudo parted {name} mkpart primary linux-swap 1M 513M
    $ sudo parted {name} mkpart extended 513M 4000M
    $ sudo parted {name} mkpart logical fat16 513M 534M
    $ sudo parted {name} mkpart logical ext4 534M 555M
    $ sudo parted {name} mkpart logical ext4 555M 606M
    $ sudo parted {name} mkpart logical ext4 606M 4000M
    
Configure the partitions (ex. /dev/sdb1) 

    $ sudo mkswap {name}{partition1}
    $ sudo mkfs.fat {name}{partition5}
    $ sudo mkfs.ext4 {name}{partition6}
    $ sudo mkfs.ext4 {name}{partition7}
    $ sudo mkfs.ext4 {name}{partition8}
    
##### Create the Kernel File

The BuildRoot build process creates the zImage file, which is a self-extracting kernel image. In order to help detect corruption, we package that into an *.itb file, which includes a checksum value that can be validated during boot time.

Navigate to your 'kubos-linux-build' folder and open the 'tools' directory.

Run the `kubos-kernel.sh` script.

The script has optional parameters (which are unlikely to be needed):
* `-i {input-file}` - Specify the name of the *.its file to use. This file describes the files that will be packaged and their usage configuration options. The default is `kubos-kernel.its`, which should also be located in the 'tools' directory.
* `-b {branch}` - Specify the branch name of U-Boot that has been built. The default is 'master'. This option should not need to be used outside of development.

The script will create the 'kubos-kernel.itb' file.

##### Copy the files

Next, you'll need to copy the kernel file into the boot partition and the rootfs into the rootfs partition

From your project folder:

Create mount folders

    $ mkdir boot
    $ mkdir rootfs
    
Mount the partitions

    $ sudo mount {name}{partition5} boot
    $ sudo mount {name}{partition6} rootfs
    
Copy the kubos-kernel.itb file into partition 5. It will need to be renamed to 'kernel'.

    $ sudo cp buildroot-2016.11/output/images/kubos-kernel.itb boot/kernel
    
Untar the rootfs into partition 6

    $ sudo tar -xvf buildroot-2016.11/output/images/rootfs.tar -C rootfs
    
Unmount the partitions

    $ sudo umount {name}{partition5}
    $ sudo umount {name}{partition6}
    
Remove the SD card and insert it into iOBC SD card slot 0.

### Install the NOR Flash Files {#install-the-nor-flash-files}

#### Pre-Requisites

1. Obtain a SEGGER SAM-ICE programmer/debugger
2. Install programming drivers from [https://www.segger.com/jlink-software.html](https://www.segger.com/jlink-software.html)
3. Install FTDI USB-to-serial drivers from [http://www.ftdichip.com/Drivers/VCP.htm](http://www.ftdichip.com/Drivers/VCP.htm)
4. Install SAM-BA (and PuTTY, if you don't already have it) from the ISIS-OBC SDK installer. (Refer to Section 3.3 of the ISIS-OBC Quick Start Guide)
4. Setup the iOBC board for serial connection and programming. (Refer to Chapter 4 of the ISIS-OBC Quick Start Guide)
5. Connect the programming and serial connection cables to your computer.
6. Power the board.

Note:  Make sure the red jumper on the programming board is in place; it bypasses the watchdog.  If you don't, the board will
continually reboot and you won't be able to flash anything.

#### Boot into U-Boot (Optional)

(Skip this section if you've never put Linux on your board before)

If you already have Linux running on your board, you'll need to boot into the U-Boot console rather than the Linux console in order to be able to
flash the board.

You'll need to establish a serial connection with the board in order to connect to the console.  Set up a serial connection to the board at a 
baudrate of 115200.

![PuTTY Connection](images/iOBC/putty_connection.png)

Once the serial connection is open, boot (or reboot) the board.  Hold down any key while the board is starting up.  This will exit out of the 
auto-boot and bring up the CLI.

![U-Boot Console](images/iOBC/uboot_console.png)

#### Flash the Files

Start up SAM-BA.  You'll want to select the at91sam9g20-ISISOBC option from the 'Select your board' drop-down.

![SAM-BA Connection Selection](images/iOBC/samba_connection_select.png)

Execute the 'Enable NorFlash' script.  This will prep the board to enable flashing.

![SAM-BA Enable NorFlash](images/iOBC/samba_enable_norflash.png)

Select the uboot.bin file in the 'Send File Name' field.

Make sure that 'Address' is set to 0xA000.

Click 'Send File'

![SAM-BA Send U-Boot](images/iOBC/samba_send_uboot.png)

Select the at91sam9g20isis.dtb file in the 'Send File Name' field (you'll need to view all file types in order to see the .dtb file)

Set 'Address' to 0x80000.

Click 'Send File'

![SAM-BA Send DTB](images/iOBC/samba_send_dtb.png)


### Boot the System {#boot-the-system}

You should now be able to set up a serial connection to your board and boot it into Linux.

You'll need to establish a serial connection with the board in order to connect to the console.  Set up a serial connection to the board at a 
baudrate of 115200.

![PuTTY Connection](images/iOBC/putty_connection.png)

You should see the console boot into Linux like this:

![Linux Console](images/iOBC/linux_console.png)


## Upgrade Process {#upgrade-process}

If you already have KubOS Linux installed on your system, but would like to upgrade to the latest version, check out the 'Upgrade Installation' section of the [KubOS Linux Upgrade doc](docs/kubos-linux-upgrade.md). Alternatively, if you would like to rollback to a previously installed version, refer to the 'Upgrade Rollback' section of the same document.

## Recovery Process {#recovery-process}

Should your KubOS Linux kernel become corrupted (as indicated by failing to successfully boot into Linux several times), the system will automatically try to recover during the next boot.  

It will go through the following steps, if each is present:

1. Reload the current version of KubOS Linux from the kpack*.itb file in the upgrade partition
2. Reload the previous version of KubOS Linux from the kpack*.itb file in the upgrade partition
3. Reload the base version of KubOS Linux from the kpack-base.itb file in the upgrade partition
4. Boot into the alternate OS

If none of these steps work, then the system will boot into the U-Boot CLI. From here, some basic troubleshooting and debugging abilities should be available.

More information about the recovery process and architecture can be found in the [KubOS Linux Recovery doc](docs/kubos-linux-recovery.md)
