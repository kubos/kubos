
.. figure:: images/kubos_logo.png
    :align: center

Overview
--------

+----------------------------------------+------------------------------------------------+------------------+-------------------------------------------------+
| `Stable Docs <http://docs.kubos.co>`__ | `Nightly Docs <http://docs.kubos.co/master>`__ | :doc:`changelog` | `Kubos Repo <https://github.com/kubos/kubos>`__ |
+----------------------------------------+------------------------------------------------+------------------+-------------------------------------------------+



The KubOS platform provides satellite developers the tools and libraries necessary to quickly bring up space-ready software. We leverage multiple existing open source projects, along with our own custom framework and SDK.

TODO: Maybe tweak which things we're highlighting here
New to KubOS? Want to know what it is? Check out our architecture guide for an overview of how KubOS works and what it provides:

 - :doc:`KubOS Architecture Overview <architecture-overview>`
 - :doc:`Getting Started <getting-started/index>`

Getting started with development? Check out our tutorials and try your own project:

 - :doc:`New User Tutorials <tutorials/index>`

Trying to integrate a payload or create your mission code? Check out our mission-specific code documentation:

 - :doc:`Payload Services <os-docs/services/payload-services>`
 - :doc:`Mission Applications <mission-dev/index>`

Having issues? Want a new feature? Just want to say hello? `Come talk to us! <https://slack.kubos.co/>`__

If for some reason Slack won't work for you, feel free to email us at info@kubos.co.

.. _supported-boards:

Supported OBCs
--------------

+------------+-----------------------------------------------------------+
| Vendor     | Description                                               |
+============+===========================================================+
| ISIS       | :doc:`ISIS-OBC <obc-docs/iobc/index>`                     |
+------------+-----------------------------------------------------------+
| Pumpkin    | :doc:`Pumpkin Motherboard Module 2 <obc-docs/mbm2/index>` |
+------------+-----------------------------------------------------------+
| Beaglebone | :doc:`Beaglebone Black, Rev. C <obc-docs/bbb/index>`      |
+------------+-----------------------------------------------------------+

Supported Hardware Devices
--------------------------

KubOS supports a selection of hardware devices in varying capacities.

All supported devices have a :doc:`device API <deep-dive/apis/device-api/index>` which may be used.

Some devices have an additional :doc:`hardware service <os-docs/services/hardware-services>` which can be built into
KubOS and provides a long-running process which allows easy, streamlined communication with the device.

Contributing to KubOS
---------------------

Want to get your code into space? Become a KubOS contributor and you will!
We welcome community developers, and are always looking for new people to collaborate with us.
`Join us on Slack <https://slack.kubos.co/>`__ or visit our
`GitHub Issues <https://github.com/kubos/kubos/issues>`__ page to participate in discussion of features
or bugs, see what you can work on, and to give feedback.

.. toctree::
    :hidden:
    
    KubOS Architecture <architecture-overview>
    Getting Started <getting-started/index>
    Tutorials <tutorials/index>
    Working with an OBC <obc-docs/index>
    Mission Development <mission-dev/index>
    KubOS Ecosystem <os-docs/index>
    Under the Hood <deep-dive/index>
    Kubos SDK <sdk-docs/index>
    Contributing to KubOS <contributing/index>
    Changelog <changelog>

Indices and tables
------------------

* :ref:`genindex`
* :ref:`search`
