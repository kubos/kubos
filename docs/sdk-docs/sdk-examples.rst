Example Kubos Projects
======================

We have provided a variety of example applications to help you get started with your Kubos project.
These examples are located in the 'Examples' folder of the `Kubos repo <http://github.com/kubos/kubos/tree/master/examples>`__, 
as well as within the `/home/vagrant/.kubos/kubos/examples` folder of the Kubos SDK box.

Each example project directory contains a `README.md` file which details the purpose of the example and how to use it.

Setting Up a C Example Application
----------------------------------

Each of the example applications written in C contains the files necessary to run as an independent Kubos project. 

In order to use them, copy the example into the desired location and then run these commands from within the top level
of the example folder::

    $ kubos link -a
    $ kubos target {desired target}
    $ kubos build

The ``kubos flash`` command can then be used to transfer the compiled binary onto your OBC.

Once transferred, you can connect to your OBC and run the binary.

Using a Rust Example Application
--------------------------------

To use a Rust example, copy the example into the desired location, then run::

    $ cargo build --target {desired target}
    
.. note:: 

    While they ultimately resolve to the same underlying target, the target names for Cargo are not the same as the
    target names used by ``kubos target``. For more information, see :ref:`info about Caro targets <TODO>`.
    
From here, please refer to the :ref:`Rust project transfer instructions <TODO>` for information about how to transfer and run
a Rust project.

Using a Python Example Application
----------------------------------

Since Python modules do not require compilation, the Python examples can be directly transferred to the OBC and
run. For more information, see the :doc:`Python SDK guide <sdk-python>`.
