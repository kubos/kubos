KubOS Ecosystem
===============

TODO: Title

TODO: TOC formatting

.. figure:: ../images/architecture_stack.png
    :align: center
    
Applications
------------

TODO: Expand?

- What do apps do?
- What apps do I need?
- First time user example app walkthrough
- App API walkthroughs
- How to write apps in other languages


Applications are user-level programs which can either be run as one-off executions or continuous
processes.

Mission applications include anything that makes decisions or governs autonomy on the satellite,
as well as any other applications designed to address mission-specific concerns.

More details about developing applications and how they can be used can be found in the
:doc:`mission development <../mission-dev/index>` section.

.. toctree::
    :maxdepth: 1
    
    apps/app-guide
    apps/python-app-api
    apps/rust-app-api
    
.. _service-docs:

Services
--------

Kubos services are defined as any persistent process that is used to interact with the satellite.
Services rarely make decisions, but will allow the user to accomplish typical flight software tasks
such as telemetry storage, file management, shell access, and hardware interaction.

.. toctree::
    :maxdepth: 1

    Core Services <services/core-services>
    Hardware Services <services/hardware-services>
    Payload Services <services/payload-services>
    Service Configuration <services/service-config>
    Service Development <services/service-dev>
    services/service-outline-guide
    GraphQL <services/graphql>
    
.. _linux-docs:
    
Kubos Linux
-----------

Kubos Linux is a custom Linux distribution designed with embedded devices in mind.

It focuses on including only drivers that are useful for space applications (eg.
I2C and SPI, rather than display drivers) and multi-layer system validation and
recovery logic.

Kubos Linux projects are built into binaries which will run as Linux user space
applications.

Installation Docs
~~~~~~~~~~~~~~~~~

.. toctree::
    :maxdepth: 1
    
    Installing Kubos Linux on Beaglebone Black <../obc-docs/bbb/installing-linux-bbb>
    Installing Kubos Linux on ISIS-OBC <../obc-docs/iobc/installing-linux-iobc>
    Installing Kubos Linux on Pumpkin MBM2 <../obc-docs/mbm2/installing-linux-mbm2>
    
General Guides
~~~~~~~~~~~~~~

.. toctree::
    :maxdepth: 1
    
    Using Kubos Linux <linux-docs/using-kubos-linux>
    Logging <linux-docs/logging>
    Process Monitoring <linux-docs/monitoring>
    
.. _system-guides:
    
System Guides
~~~~~~~~~~~~~

.. toctree::
    :maxdepth: 1
    
    Working with the Beaglebone Black <../obc-docs/bbb/working-with-the-bbb>
    Working with the iOBC <../obc-docs/iobc/working-with-the-iobc>
    Working with the Pumpkin MBM2 <../obc-docs/mbm2/working-with-the-mbm2>
    
.. _sysadmin:
    
SysAdmin Docs
~~~~~~~~~~~~~

.. toctree::
    :maxdepth: 1
    
    Kubos Linux Overview <linux-docs/kubos-linux-overview>
    Kubos Linux Upgrades <linux-docs/kubos-linux-upgrade>
    Kubos Linux Recovery <linux-docs/kubos-linux-recovery>
    Building Kubos Linux for the Beaglebone Black <linux-docs/kubos-linux-on-bbb>
    Building Kubos Linux for the ISIS-OBC <linux-docs/kubos-linux-on-iobc>
    Building Kubos Linux for Pumpkin MBM2 <linux-docs/kubos-linux-on-mbm2>
    Configuring KubOS <linux-docs/configuring-kubos>
    