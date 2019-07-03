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

+------------------+-------------------------------+
| OBC              | ``cargo build``               |
+------------------+-------------------------------+
| Beaglebone Black | arm-unknown-linux-gnueabihf   |
+------------------+-------------------------------+
| ISIS-OBC         | armv5te-unknown-linux-gnueabi |
+------------------+-------------------------------+
| Pumpkin MBM2     | arm-unknown-linux-gnueabihf   |
+------------------+-------------------------------+

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

This can be done by specifying the C compiler path in the ``CC`` envar like so::

    $ CC={path} cargo build --target {target}
    
For example::

    $ CC=/usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc cargo build --target armv5te-unknown-linux-gnueabi
    
Some crates depend on pkg-config, which requires that an additional flag, ``PKG_CONFIG_ALLOW_CROSS``,
be set in order to enable cross-compiling::

    $ PKG_CONFIG_ALLOW_CROSS=1 CC=/usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc cargo build --target armv5te-unknown-linux-gnueabi

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
