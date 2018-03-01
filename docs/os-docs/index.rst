Kubos Linux
===========

Kubos Linux is a custom Linux distribution designed with embedded devices in mind.

It focuses on including only drivers that are useful for space applications (ex. 
I2C and SPI, rather than display drivers) and multi-layer system validation and 
recovery logic.

Kubos Linux projects are built into binaries which will run as Linux user space 
applications.

Installation Docs
-----------------

.. toctree::
    :maxdepth: 1
    
    Installing Kubos Linux on Beaglebone Black <../installation-docs/installing-linux-bbb>
    Installing Kubos Linux on ISIS-OBC <../installation-docs/installing-linux-iobc>
    Installing Kubos Linux on Pumpkin MBM2 <../installation-docs/installing-linux-mbm2>
    
General Guides
--------------

.. toctree::
    :maxdepth: 1
    
    First Linux Project <first-linux-project>
    Using Kubos Linux <using-kubos-linux>
    
System Guides
-------------
    
.. toctree::
    :maxdepth: 1
    
    Working with the Beaglebone Black <working-with-the-bbb>
    Working with the iOBC <working-with-the-iobc>
    Working with the Pumpkin MBM2 <working-with-the-mbm2>
    
.. _sysadmin:
    
SysAdmin Docs
-------------
    
.. toctree::
    :maxdepth: 1
    
    Kubos Linux Overview <kubos-linux-overview>
    Kubos Linux Upgrades <kubos-linux-upgrade>
    Kubos Linux Recovery <kubos-linux-recovery>
    Building Kubos Linux for the Beaglebone Black <kubos-linux-on-bbb>
    Building Kubos Linux for the ISIS-OBC <kubos-linux-on-iobc>
    Building Kubos Linux for Pumpkin MBM2 <kubos-linux-on-mbm2>