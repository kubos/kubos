Rust
====

The Kubos SDK Vagrant box comes with support for rust, cargo and several
helper tools to integrate C-based Kubos libraries with Rust projects.

.. note::

   All of the following instructions are assumed to be run inside of the
   Kubos SDK Vagrant environment.

New Project
-----------

A new rust project can be created by running either of the following commands:

`Executable project`::

  cargo new --bin bin_name


`Library project`::

  cargo new lib_name

Cargo will create the project folder and a basic folder structure.

Compiling
---------

Compiling a rust project is done from within the project's folder. The command is::

  cargo kubos -c build

Running this command as-is will compile the current rust project with the
native linux compiler.

Running
-------

Running a local rust project must also be done from within the project's folder.
The command to run an executable project is::

  cargo kubos -c run

.. note::

   This command will only work for executable projects, not library projects.

Cross-compiling
---------------

The Kubos SDK supports cross-compiling Rust projects through the ``cargo kubos`` tool.
The syntax for cross-compiling is::

  cargo kubos -c build -t target

The following targets are currently supported:

 - ``kubos-linux-beaglebone-gcc`` - Beaglebone Black
 - ``kubos-linux-pumpkin-mbm2-gcc`` - Pumpkin MBM2
 - ``kubos-linux-isis-gcc`` - ISIS iOBC

Flashing
--------

.. note::

   The addition of Rust to the Kubos SDK is pretty recent and SDK tooling is
   currently undergoing revision to make the flashing process smoother!

Flashing Rust projects is currently done using the ``kubos-cli`` tool. It is a bit
of a process laid out in the following steps:

0. Make sure the target hardware is attached to your computer via serial cable.
1. Cross-compile the rust project for the desired target.
2. Navigate to an existing example kubos module like ``kubos-linux-example``.
3. Run ``kubos linux -a``.
4. Run ``kubos -t target build`` using the same target you cross-compiled with.
5. Run ``kubos flash $(find `pwd`/rel/path/to/project -name project_name -type f)``.
   It is important here that you put the relative path to your rust project
   after the ```pwd```. Another option is ``kubos flash /absolute/path/to/rust/binary``.
6. Assuming all went well you will now see ``kubos flash`` sending your compiled
   binary over to the target.

.. note::

   The current Kubos Rust SDK only supports cross-compiling and flashing to the
   BeagleboneBlack and Pumpkin MBM2. Support for the ISIS iOBC coming soon!


Running on Target
-----------------

The following steps will allow you to run Rust binaries which have been flashed
to a Linux target:

1. Run ``minicom kubos`` from inside of the Vagrant box.
2. Enter the username ``kubos`` and the password ``Kubos123``.
3. Navigate to the folder ``/home/system/usr/local/bin``.
4. This folder is the default destination for flashed files. Your binaries should
   be here. You can now run them with ``./binary``.
