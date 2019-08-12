Getting Started with KubOS and Python
=====================================

This document will cover getting started using Python for local KubOS development.

Good news! If you have installed the build dependencies listed in :ref:`build-dependencies`
then you are ready to start developing in Python!

It is important to note that Kubos Linux runs Python3.7. It is strongly advised
that any local testing is done with this version to ensure compatibility.

You will also want to ensure that the ``app-api`` Python library has been installed
locally. If you have not already installed it, then follow these steps::

    $ git clone https://github.com/kubos/kubos
    $ cd kubos/apis/app-api/python
    $ pip3 install .

Great! If you have followed all of these steps then you should have all necessary
Python dependencies installed!

Next Steps
----------

Ready to create an application in Python? Take a look at the :doc:`Python mission application
<../tutorials/first-mission-app>` tutorial.

Want to create a payload service in Python? Take a look at the `example Python service
<https://github.com/kubos/kubos/tree/master/examples/python-service>`__ for a model of
how to structure a Python service.

Ready to transfer your Python script to hardware? Take a look at the :doc:`document <../sdk-docs/sdk-python>`
on using Python in the SDK for further instructions.

Interested in working on the KubOS Python source or contributing back? Take a look at
the :doc:`Contributing to KubOS <../contributing/index>` docs.