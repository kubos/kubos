KubOS Linux
===========

KubOS Linux is a custom Linux distribution designed with embedded devices in mind.

It focuses on including only drivers that are useful for space applications (ex. 
I2C and SPI, rather than display drivers) and multi-layer system validation and 
recovery logic.

KubOS Linux projects are built into binaries which will run as Linux user space 
applications.

Installation Docs
-----------------

.. toctree::
    :maxdepth: 1
    
    Installing KubOS Linux on Beaglebone Black <../installation-docs/installing-linux-bbb>
    Installing KubOS Linux on ISIS-OBC <../installation-docs/installing-linux-iobc>
    Installing KubOS Linux on Pumpkin MBM2 <../installation-docs/installing-linux-mbm2>
    
General Guide
-------------

.. toctree::
    :maxdepth: 1
    
    Using KubOS Linux <using-kubos-linux>
    
System Guides
-------------
    
.. toctree::
    :maxdepth: 1
    
    Working with the Beaglebone Black <working-with-the-bbb>
    Working with the iOBC <working-with-the-iobc>
    Working with the Pumpkin MBM2 <working-with-the-mbm2>
    
SysAdmin Docs
-------------
    
.. toctree::
    :maxdepth: 1
    
    KubOS Linux Overview <kubos-linux-overview>
    KubOS Linux Upgrades <kubos-linux-upgrade>
    KubOS Linux Recovery <kubos-linux-recovery>
    Building KubOS Linux for the Beaglebone Black <kubos-linux-on-bbb>
    Building KubOS Linux for the ISIS-OBC <kubos-linux-on-iobc>
    Building KubOS Linux for Pumpkin MBM2 <kubos-linux-on-mbm2>