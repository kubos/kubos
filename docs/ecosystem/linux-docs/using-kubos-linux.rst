Using Kubos Linux
=================

Overview
--------

This document is intended as a general guide for creating,
loading, and using KubOS projects and other files within the user space
of Kubos Linux.

Reference Documents
-------------------

General Documentation
~~~~~~~~~~~~~~~~~~~~~

-  :doc:`../../sdk-docs/sdk-installing`
-  :doc:`../../tutorials/first-mission-app`

Board-Specific Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

-  :doc:`../../obc-docs/bbb/index`
-  :doc:`../../obc-docs/iobc/index`
-  :doc:`../../obc-docs/mbm2/index`


Using Peripherals
-----------------

Each board has a variety of different ports available for interacting with
peripheral devices. Currently, users should interact with these devices
using the standard Linux functions. A Kubos HAL will be added in the
future.

Please refer to the appropriate :doc:`Working with {board} <../../obc-docs/index>` document for more
information about the specific peripheral availability.

.. _user-accounts:

User Accounts
-------------

In general, it is preferred to use a non-root user account to interact
with a Linux system. A default user account 'kubos' is included with
Kubos Linux. Other user accounts can be created using the standard Linux
commands (``adduser``, ``useradd``, etc).

All user accounts should have a home directory in the format
'/home/{username}'.

The ``passwd`` command can be used to change the password of existing user
accounts.

Kubos Linux File System
-----------------------

There are a few key directories residing within the Kubos Linux user
space.

/home
~~~~~

All user-created files should reside under the /home directory. This
directory maps to a separate partition from the root file system. As a
result, all files here will remain unchanged if the system goes through
a kernel upgrade or downgrade.

The home directories of all user accounts, except root, should live
under this directory.

.. warning::

    Any files not residing under the /home directory will be destroyed
    during an upgrade/downgrade
    
/home/system/logs
^^^^^^^^^^^^^^^^^

All log files generated with :doc:`rsyslog <logging>` reside in this directory.
A symlink has been set up to have `/var/log` route to this location.

/home/system/usr/bin
^^^^^^^^^^^^^^^^^^^^

This directory is included in the system's PATH, so applications placed
here can be called directly from anywhere, without needing to know the
full file path.

/home/system/etc/init.d
^^^^^^^^^^^^^^^^^^^^^^^

All user-application initialization scripts live under this directory.
The naming format is 'S{run-level}{application}'.

Resetting the Boot Environment
------------------------------

.. note::

    This is a case which normal users should never encounter, but becomes more likely when initially testing custom Kubos Linux builds.
    Due to the blocking nature of the behavior, this information has been included in this doc in order to make it more prominent.

If the system goes through the :doc:`full recovery process <kubos-linux-recovery>` and the bootcount is still exceeded,
it will present the U-Boot CLI instead of attempting to boot into Kubos Linux again.

If this occurs, follow the :ref:`instructions for resetting the boot environment <env-reset>`.
