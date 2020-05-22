Setting Up Your Local Environment
=================================

This document will lead you through the steps of setting up a local KubOS development environment. It is assumed that you are either running a Linux or macOS system. Windows users should refer to the :doc:`Windows setup <windows-setup>` guide.

.. _build-dependencies:

Development Dependencies
------------------------

The following tools and libraries will need to be installed in order to build and
run existing and new KubOS projects. You can install these however you like, however
we suggest using your distro's package manager (``apt``, ``dnf``, ``snap``, etc) on Linux,
and using Homebrew on macOS.

- git
- curl
- Rust + Cargo (We suggest using `rustup <https://rustup.rs/>`__ to install)

    - You will probably want to add ``~/.cargo/bin`` to your system's ``PATH``

- Rust v1.39.0 (``rustup default 1.39.0``)
- Clippy (``rustup component add clippy``)
- Rustfmt (``rustup component add rustfmt``)
- gcc
- pkg-config
- libssl-dev or openssl-dev (or openssl in macOS)
- sqlite **and** libsqlite3-dev (libsqlite3-dev is not needed in macOS)
- Python3.7
- pip3
- The Python3 libraries toml, mock, and responses (``pip3 install toml mock responses``)

After installing these dependencies, we suggest fetching the latest copy of the `KubOS source repo <https://github.com/kubos/kubos>`__::

    $ git clone https://github.com/kubos/kubos

After fetching the KubOS source repo, we suggest installing the Python ``app-api``, which is used in developing Python-based applications::

    $ pushd kubos/apis/app-api/python && pip3 install . --user && popd
    
We also suggest installing the Python ``kubos-service`` library, which is used in developing Python-based services::
 
    $ pushd kubos/libs/kubos-service && pip3 install . --user && popd

After installing all of these dependencies, we suggest running the following script
from the base of the kubos repo to verify everything is installed correctly and working::

    $ ./tools/kubos_verify.sh

Next Steps
----------

Now that your environment is set up, you can get started developing your first KubOS project.

We recommend that you look at the following documents next:

- :doc:`using-python`
- :doc:`using-rust`
- :doc:`../tutorials/index`
