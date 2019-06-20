Compiling Code for a Target OBC
===============================

In order for programs written using a compiled language (Rust, C, etc) to work on a target OBC,
it will need to be compiled using the appropriate cross-compile toolchain.

This document covers the process of installing the cross-compile toolchain and compiling a program
for a target OBC using a local development environment.

The Kubos SDK comes with the cross-compile toolchains pre-installed, as well as some additional
helper tooling.
If you are using the SDK as your development environment, please refer to :doc:`those docs <../sdk-docs/index>`
for more specific information.

Download
--------

Below are the direct download links for the latest cross-compiling toolchains.
If you need a prior version of a toolchain, please contact a Kubos team member for support.

- `Beaglebone Black / Pumpkin MBM2 <https://s3.amazonaws.com/kubos-world-readable-assets/bbb_toolchain.tar.gz>`__
- `ISIS OBC <https://s3.amazonaws.com/kubos-world-readable-assets/iobc_toolchain.tar.gz>`__

Installation
------------

Uncompress the toolchain into your location of choice.

The remainder of this document will refer to the location as `/usr/bin/{obc}_toolchain/`, since this
is the location we use internally.

Rust
~~~~

If you are programming in Rust, you will now want to `configure Cargo <https://doc.rust-lang.org/cargo/reference/config.html>`__
to use the new toolchain.

Navigate to your ``.cargo`` directory and edit the ``config`` file (or create one if it doesn't
already exist)

Add a new ``[target.*]`` section with a ``linker`` variable which points to the toolchain's ``gcc``
file::

    [target.$triple]
    linker = "/usr/bin/{obc}_toolchain/usr/bin/{arch}-gcc"
    
Please refer to the Kubos SDK's `Cargo config file <https://github.com/kubos/kubos-vagrant/blob/master/kubos-dev/bin/cargo_config>`__
for reference.
You should be able to directly copy the lines pertaining to your target OBC.

Using the Toolchains
--------------------

Rust
~~~~

The new toolchains can be used by specifying the target triple you defined in the ``--target``
argument when running ``cargo build``.

For example::

    $ cargo build --target arm-unknown-linux-gnueabihf
    
More specific information about how to set up and use a Rust project can be found in the
:doc:`../sdk-docs/sdk-rust` doc.

C
^

To use the toolchains with C, define the ``CC`` and ``CXX`` envars with the path to the appropriate
binary prior to compiling your code.

For example::

    $ export CC=/usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc
    $ export CXX=/usr/bin/bbb_toolchain/usr/bin/arm-linux-g++
    $ cmake .. && make

More specific information about how to set up and use a C project can be found in the
:doc:`../sdk-docs/sdk-c` doc.