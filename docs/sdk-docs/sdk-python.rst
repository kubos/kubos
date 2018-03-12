Using Python with the Kubos SDK
===============================

The Kubos SDK Vagrant box comes with a Python interperter and all the modules
needed to develop a basic hardware service. Services can be tested locally
in the Vagrant box up until the point that they need hardware interaction.

Flashing
--------

.. note::

   Python is currently only supported on the BeagleboneBlack and MBM2

There are currently two ways to add Python programs to a system running Kubos Linux:

- Add a new Python package to a custom Kubos Linux build

    - Fork and clone `kubos-linux-build <https://github.com/kubos/kubos-linux-build>`__
      and add additional Python packages to the `package/python` directory
    - You can see our current system packages and how they are structured 
      `here <https://github.com/kubos/kubos-linux-build/tree/master/package/python>`__.
    - See the :ref:`SysAdmin docs <sysadmin>` for more information on
      building Kubos Linux.

- Add individual Python files on the fly by flashing them to an attached hardware target

    - Navigate to an existing example Kubos module like ``kubos-linux-example``.
    - Run ``kubos linux -a``.
    - Run ``kubos -t [target] build`` using the same target you cross-compiled with.
    - Run ``kubos flash /absolute/path/to/python/file``. You must use the absolute
      path to the Python file you'd like to flash. Relative paths will not work.

Running on Target
-----------------

The following steps will allow you to run Python files which have been flashed
to a Linux target:

0. Make sure the target hardware is attached to your computer via a serial cable.
1. Run ``minicom kubos`` from inside of the Vagrant box.
2. Enter the username ``kubos`` and the password ``Kubos123``.
3. Navigate to the folder ``/home/system/usr/local/bin``.
4. This folder is the default destination for flashed files. Your
   Python files should be here. You can now run them with ``python file.py``.
