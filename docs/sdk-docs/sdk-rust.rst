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

Compiling Projects Which Have C Dependencies
--------------------------------------------

A special Kubos SDK tool has been created for build interoperability between
Rust projects and C projects: `cargo kubos`. This tool allows Rust projects
to correctly compile against existing C libraries and also provides
cross-compiler compatibility for existing Kubos hardware targets.

Compiling a Rust project is done from within the project's folder. The command is::

  cargo kubos -c build -t [target]

You may also omit the ``-t`` parameter in order to build the project to run directly in your
Vagrant image. Use ``cargo kubos -c run`` to trigger execution in this case.

.. _rust-targets:

Cross-compilation Targets
-------------------------

The Kubos SDK provides Rust cross-compilation targets for each of the supported OBCs.

The target name varies depending which command is used to compile the project.

+------------------+-------------------------------+------------------------------+
| OBC              | ``cargo build``               | ``cargo kubos -c build``     |
+------------------+-------------------------------+------------------------------+
| Beaglebone Black | arm-unknown-linux-gnueabihf   | kubos-linux-beaglebone-gcc   |
+------------------+-------------------------------+------------------------------+
| ISIS-OBC         | armv5te-unknown-linux-gnueabi | kubos-linux-isis-gcc         |
+------------------+-------------------------------+------------------------------+
| Pumpkin MBM2     | arm-unknown-linux-gnueabihf   | kubos-linux-pumpkin-mbm2-gcc |
+------------------+-------------------------------+------------------------------+

.. _rust-transfer:

Flashing
--------

.. note::

   The addition of Rust to the Kubos SDK is pretty recent and SDK tooling is
   currently undergoing revision to make the flashing process smoother!

Via Ethernet
~~~~~~~~~~~~

Rust project binaries can be transferred to the target OBC :ref:`via ethernet <ethernet>` for
targets which have ethernet enabled.

Binaries may be transferred to any location on the target board, however, they should be copied
to `/home/system/usr/bin` if you would like them to be automatically accessible via the system PATH.

Via Serial
~~~~~~~~~~

Flashing Rust projects over the debug serial connection is done using the :doc:`Kubos CLI <sdk-reference>`.
It is a bit of a process laid out in the following steps:

0. Make sure the target hardware is attached to your computer via a serial cable.
1. Cross-compile the Rust project for the desired target.
2. Navigate to an existing example kubos module like ``kubos-linux-example``.
3. Run ``kubos linux -a``.
4. Run ``kubos -t [target] build`` using the same target you cross-compiled with.
5. Run ``kubos flash $(find `pwd`/rel/path/to/project -name project_name -type f)``.
   It is important here that you put the relative path to your rust project
   after the ```pwd```. Another option is ``kubos flash /absolute/path/to/rust/binary``.
6. Assuming all went well you will now see ``kubos flash`` sending your compiled
   binary over to the target.

If you would like the transferred binary to be accessible from any location in the system,
it will then need to be manually transferred to a location the system PATH:

1. Run ``minicom kubos`` from inside of the Vagrant box.
2. Enter the username ``kubos`` and the password ``Kubos123``.
3. Navigate to the folder ``/home/system/usr/local/bin``.
4. Run ``mv {binary-name} ../../bin``.

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

