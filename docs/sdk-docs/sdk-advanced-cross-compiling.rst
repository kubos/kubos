Advanced Rust Cross Compiling
=============================

In the future there may be the need to cross compile KubOS for targets
not currently supported. This document gives instructions for setting
up cross compiling support for those targets.

First check the `Rust Platform Support <https://forge.rust-lang.org/release/platform-support.html>`__
for your target. If it is not in this list...then contact us via Slack.

If it is in the list, then step one is to attempt to install the triple via ``rustup``::

    $ rustup target add target-triple-here

If this installs correctly then all you need is a working gcc toolchain and you should be
good to go! There is one piece of necessary ``cargo`` configuration needed. A cargo 
`config file <https://doc.rust-lang.org/cargo/reference/config.html>`__ named
``config`` in either ``$HOME/.cargo`` or ``$PROJECT/.cargo`` folder. Inside you'll need to set
the following options::

    [target.target-triple]
    linker = "$TOOLCHAIN/toolchain-name-gcc"
    ar = "$TOOLCHAIN/toolchain-name-ar"

If the desired target is under **Tier 2.5** or **Tier 3** then it likely
cannot be added directly via ``rustup`` and will required additional work.

First these dependencies need to be installed::

    $ cargo install xargo
    $ rustup default nightly-2020-01-29
    $ rustup component add rust-src

You will also need a functional gcc cross-compiling toolchain in
an accessible location that we'll call $TOOLCHAIN.

The environment variables ``CC`` and ``CXX`` need to be set prior
to beginning to cross compile::

    $ export CC=$TOOLCHAIN/bin/target-triple-gcc
    $ export CXX=$TOOLCHAIN/bin/target-triple-g++

Now run this command in your Rust project::

    $ xargo build --target target-triple-here

If you get an error about ``can't find crate for 'std'`` then:

Now create the following files in your Rust project:

``Xargo.toml``::

    [target.target-triple.std]
    stage = 0

``.cargo/config``::

    [target.target-triple]
    linker = "$TOOLCHAIN/toolchain-name-gcc"
    ar = "$TOOLCHAIN/toolchain-name-ar"

And rebuild. You may also want to delete the ``$HOME/.xargo`` and ``target``
folders before rebuilding.

If you get an error about ``can't find a crate for 'panic_unwind`` then
you may need to define a release profile and start building in release mode.

Add this to ``Cargo.toml``::

    [profile.release]
    panic = "abort"

Then build with::

    xargo build --target target --release
    
