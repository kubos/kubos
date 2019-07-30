Using Rust with the Kubos SDK
=============================

The Kubos SDK comes with pre-built support for `Rust <https://www.rust-lang.org/>`__ and
`Cargo <https://doc.rust-lang.org/cargo/>`__.
Additionally, it includes tooling to assist with cross-compiling for a target OBC and to build
projects which use both Rust and C.

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
    
Making Rust Binaries Smaller
----------------------------

By default, Rust binaries can be quite large.
:ref:`Check out our Rust optimization tips <rust-opt>` for suggestions on how to make your compiled
Rust projects smaller.

.. _rust-transfer:

Transferring to Target
----------------------

Rust binaries can be transferred to the target OBC :ref:`via a supported file transfer
method <file-transfer>`.

Binaries may be transferred to any location on the target board, however, they should be copied
to `/home/system/usr/bin` if you would like them to be automatically accessible via the system PATH.

Running on Target
-----------------

Once transferred, the binary can be started with ``/path/to/binary-name``, or by simply specifying
the binary name if the file was transferred to a system PATH directory.
