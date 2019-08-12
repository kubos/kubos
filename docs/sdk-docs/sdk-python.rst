Using Python with the Kubos SDK
===============================

The Kubos SDK Vagrant box comes with a Python interperter and all the modules needed to develop a
basic hardware service or mission application.
Services can be tested locally in the Vagrant box up until the point that they need hardware
interaction.

.. note::

    The SDK comes with support for both Python2.7 and Python3.7.
    However, only Python3.7 is available on OBCs running Kubos Linux.

Transferring to Target
----------------------

.. note::

   The ISIS-OBC does not currently support Python due to size constraints

There are currently two ways to add Python scripts and libraries to a system running Kubos Linux:

- Add a new Python package to a custom Kubos Linux build

    - Fork and clone `kubos-linux-build <https://github.com/kubos/kubos-linux-build>`__
      and add additional Python packages to the `package/python` directory
    - You can see our current system packages and how they are structured
      `here <https://github.com/kubos/kubos-linux-build/tree/master/package/python>`__.
    - See the :ref:`custom Kubos Linux docs <custom-klb>` for more information on
      building Kubos Linux.

- Add individual Python files on the fly by transferring them to an attached hardware target

Python project files can be transferred to the target OBC :ref:`via a supported file transfer
method <file-transfer>`.

Binaries may be transferred to any location on the target board, however, they should be copied
to `/home/system/usr/bin` if you would like them to be automatically accessible via the system PATH.

Running on Target
-----------------

The following steps will allow you to run Python3.7 files which have been flashed
to a Linux target:

0. Make sure you can :doc:`communicate with your OBC <../obc-docs/comms-setup>`.
1. Transfer your python script using a supported :ref:`file transfer method <file-transfer>`.
2. Navigate to the destination folder of the transfer.
3. Your Python files should be here. You can now run them with ``python file.py``.
