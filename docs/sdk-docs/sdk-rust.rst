Using Rust with the Kubos SDK
=============================

The Kubos SDK Vagrant box comes with support for Rust, Cargo, and several
helper tools to integrate C-based Kubos libraries with Rust projects.

.. note::

   All of the following instructions are assumed to be run inside of the
   Kubos SDK Vagrant environment.

New Project
-----------

A new Rust project can be created by running either of the following commands:

`Executable project`::

  cargo new --bin bin_name


`Library project`::

  cargo new lib_name

Cargo will create the project folder and a basic folder structure.

Compiling
---------

To compile the project use the normal Cargo build command::

    $ cargo build --target [target]
    
The resulting binary will be located in `{project directory}/target/{target}/debug/{project name}`.

This binary can then be transferred to the target OBC for execution.

You may also omit the ``--target`` parameter in order to build the project to run directly in your
Vagrant image. Use ``cargo run`` to trigger execution in this case.

.. _rust-targets:

Cross-compilation
-----------------

The Kubos SDK provides Rust cross-compilation targets for each of the supported OBCs.

The target name varies depending which command is used to compile the project.

+------------------+-------------------------------+------------------------------+
| OBC              | ``cargo build`` target        | ``cargo kubos`` target       |
+------------------+-------------------------------+------------------------------+
| Beaglebone Black | arm-unknown-linux-gnueabihf   | kubos-linux-beaglebone-gcc   |
+------------------+-------------------------------+------------------------------+
| ISIS-OBC         | armv5te-unknown-linux-gnueabi | kubos-linux-isis-gcc         |
+------------------+-------------------------------+------------------------------+
| Pumpkin MBM2     | arm-unknown-linux-gnueabihf   | kubos-linux-pumpkin-mbm2-gcc |
+------------------+-------------------------------+------------------------------+

Some Rust crates require that the C compiler be explicitly declared when cross-compiling.

+------------------+-----------------------------------------------+
| OBC              | Compiler Path                                 |
+------------------+-----------------------------------------------+
| Beaglebone Black | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  |
+------------------+-----------------------------------------------+
| ISIS-OBC         | /usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc |
+------------------+-----------------------------------------------+
| Pumpkin MBM2     | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  |
+------------------+-----------------------------------------------+

To simplify development when cross-compiling, use the ``cargo kubos`` command to automatically setup
the build environment. The ``cargo kubos`` command takes a required cargo sub-command (i.e. ``build``,
``test``), and a target. For example, to build a project for the ISIS iOBC::

    $ cargo kubos -c build -t kubos-linux-isis-gcc
    
Cross compiling can also be done manually by specifying the C compiler path in the ``CC``
environment variable like so::

    $ CC={path} cargo build --target {target}
    
For example, the equivalent command as above using ``cargo build``::

    $ CC=/usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc cargo build --target armv5te-unknown-linux-gnueabi
    
Some crates also depend on pkg-config, which requires that an additional environment variable,
``PKG_CONFIG_ALLOW_CROSS``, be set in order to enable cross-compiling::

    $ PKG_CONFIG_ALLOW_CROSS=1 CC=/usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc cargo build --target armv5te-unknown-linux-gnueabi

.. _rust-transfer:

Flashing
--------

.. note::

   The addition of Rust to the Kubos SDK is pretty recent and SDK tooling is
   currently undergoing revision to make the flashing process smoother!

Rust binaries can be transferred to the target OBC :ref:`via a supported file transfer
method <file-transfer>`.

Binaries may be transferred to any location on the target board, however, they should be copied
to `/home/system/usr/bin` if you would like them to be automatically accessible via the system PATH.

Running on Target
-----------------

Once transferred, the binary can be started with ``./binary-name`` if you log in to the board
and navigate to the specific directory in which the file is located, or without the ``./`` characters
from any location if the file was transferred to a system PATH directory.

Formatting
----------

The ``rustfmt`` tool can be used to automatically edit your source code to match the
current Rust standards.

To format your code:

- Install ``rustfmt``::

    $ rustup component add rustfmt-preview
    
- Navigate to your project folder
- Run the formatting tool via Cargo::

    $ cargo fmt
    
Important Notes
~~~~~~~~~~~~~~~

- Kubos is currently using the ``0.4.2-stable`` version of ``rustfmt``.
- Using ``cargo install rustfmt`` to install ``rustfmt`` will result in the deprecated version being installed,
  which has slightly different formatting rules. Please use the ``rustup`` installation method instead.

