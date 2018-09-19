
.. figure:: images/kubos_logo.png
    :align: center

Overview
--------

+----------------------------------------+------------------------------------------------+---------------------------+-------------------------------------------------+
| `Stable Docs <http://docs.kubos.co>`__ | `Nightly Docs <http://docs.kubos.co/master>`__ | :doc:`dev-docs/changelog` | `Kubos Repo <https://github.com/kubos/kubos>`__ |
+----------------------------------------+------------------------------------------------+---------------------------+-------------------------------------------------+



The KubOS platform provides satellite developers the tools and libraries necessary to quickly bring up space-ready software. We leverage multiple existing open source projects, along with our own custom framework and SDK. 

New to KubOS? Want to know what it is? Check out our architecture guide for an overview of how KubOS works and what it provides:

 - :doc:`KubOS Architecture Overview <architecture-overview>`

Getting set up with KubOS for the first time? Check out our SDK and try your own project:

 - :doc:`First Time Users <first-time-users>`

Trying to integrate a payload or create your mission code? Check out our mission-specific code documentation: 

 - :doc:`Payload Services <services/payload-services>`
 - :doc:`Mission Applications <app-docs/index>`

Having issues? Want a new feature? Just want to say hello? `Come talk to us! <https://slack.kubos.co/>`__

If for some reason Slack won't work for you, feel free to email us at info@kubos.co.

.. _supported-boards:

Supported OBCs
--------------

+------------+---------------------------------------------------------------------+
| Vendor     | Description                                                         |
+============+=====================================================================+
| ISIS       | :doc:`ISIS-OBC <os-docs/working-with-the-iobc>`                     |
+------------+---------------------------------------------------------------------+
| Pumpkin    | :doc:`Pumpkin Motherboard Module 2 <os-docs/working-with-the-mbm2>` |
+------------+---------------------------------------------------------------------+
| Beaglebone | :doc:`Beaglebone Black, Rev. C <os-docs/working-with-the-bbb>`      |
+------------+---------------------------------------------------------------------+

Supported Hardware Devices
--------------------------

KubOS supports a selection of hardware devices in varying capacities.

All supported devices have a :doc:`device API <apis/device-api/index>` which may be used.

Some devices have an additional :doc:`hardware service <services/hardware-services>` which can be built into
KubOS and provides a long-running process which allows easy, streamlined communication with the device.

Contributing to KubOS:
----------------------

Want to get your code into space? Become a KubOS contributor and you will! 
We welcome community developers, and are always looking for new people to collaborate with us. 
Come check out our `community Trello Board <https://trello.com/b/pIWxmFua/kubos-community>`__ to see what's being worked on and what's next on the horizon! 
`Join us on Slack <https://slack.kubos.co/>`__ to participate in discussion of features or bugs, see what you can work on, and to give feedback.

.. toctree::
    :hidden:
    
    KubOS Architecture <architecture-overview>
    First Time Users <first-time-users>
    Installation Docs <installation-docs/index>
    Kubos SDK <sdk-docs/index>
    Kubos Mission Applications <app-docs/index>
    Kubos Services <services/index>
    Kubos APIs <apis/index>
    Kubos Linux <os-docs/index>
    Developer Docs <dev-docs/index>
    FAQs and Troubleshooting Tips <faq-troubleshooting>

Indices and tables
------------------

* :ref:`genindex`
* :ref:`search`
