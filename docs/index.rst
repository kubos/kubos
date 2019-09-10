
.. figure:: images/kubos_logo.png
    :align: center

Overview
--------

+----------------------------------------+------------------------------------------------+------------------+-------------------------------------------------+
| `Stable Docs <http://docs.kubos.co>`__ | `Nightly Docs <http://docs.kubos.co/master>`__ | :doc:`changelog` | `Kubos Repo <https://github.com/kubos/kubos>`__ |
+----------------------------------------+------------------------------------------------+------------------+-------------------------------------------------+



The KubOS platform provides satellite developers the tools and libraries necessary to quickly bring up space-ready software.
We leverage multiple existing open source projects, along with our own custom framework and SDK.

Just looking for an explanation of what KubOS is?

Check out our design guide for an overview of how KubOS works and what it provides,
and check out our ecosystem docs for a deeper explanation of each component:

 - :doc:`KubOS Design <kubos-design>`
 - :doc:`KubOS Ecosystem <ecosystem/index>`

Want to get started with development?
Follow our Getting Started guide to get your development environment set up and then check out our
tutorials or try your own project:

 - :doc:`Getting Started <getting-started/index>`
 - :doc:`New User Tutorials <tutorials/index>`

Trying to start developing your mission with KubOS?
Check out our mission development overview:

 - :doc:`Mission Development <mission-dev/index>`

Having issues? :doc:`Check out our FAQs <faq-troubleshooting>` for your issue, or `create a new issue <https://github.com/kubos/kubos/issues/new/choose>`__ on the repo to let us know.

Want a new feature? Create a `feature request <https://github.com/kubos/kubos/issues/new/choose>`__!

Just want to say hello? `Come talk to us! <https://slack.kubos.co/>`__ If for some reason Slack won't work for you, feel free to email us at info@kubos.com.

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

.. _supported-hardware:

Supported Hardware Devices
--------------------------

KubOS supports a selection of hardware devices in varying capacities.

Some devices have an additional :doc:`hardware service <ecosystem/services/hardware-services>` which can be built into
KubOS and provides a long-running process which allows easy, streamlined communication with the device.

All supported devices have a :doc:`device API <deep-dive/apis/device-api/index>` which may be used.

+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| Vendor                                                    | Device                                                                                                        | Service                                                                                      |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `Adcole Maryland Aerospace <https://www.adcolemai.com>`__ | `MAI-400 <https://www.adcolemai.com/wp-content/uploads/2019/02/AMA-MAI-400-Datasheet.pdf>`__                  | |MAI-400|                                                                                    |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `Clyde Space <https://www.aac-clyde.space/>`__            | `Starbuck Nano (formerly 3G EPS) <https://www.aac-clyde.space/satellite-bits/eps>`__                          | |Clydespace-EPS|                                                                             |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `GOMspace <https://www.gomspace.com>`__                   | `NanoPower P31u <https://gomspace.com/shop/subsystems/power-supplies/nanopower-p31u.aspx>`__                  | No                                                                                           |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `ISIS <https://www.isispace.nl>`__                        | `Antenna System <https://www.isispace.nl/products/cubesat-antenna-systems/>`__                                | |ISIS-AntS|                                                                                  |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `ISIS <https://www.isispace.nl>`__                        | `iMTQ <https://www.isispace.nl/product/isis-magnetorquer-board/>`__                                           | No                                                                                           |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `ISIS <https://www.isispace.nl>`__                        | `TRXVU <https://www.isispace.nl/product/isis-uhf-downlink-vhf-uplink-full-duplex-transceiver/>`__             | No                                                                                           |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `ISIS <https://www.isispace.nl>`__                        | `OBC Supervisor <https://www.isispace.nl/product/on-board-computer/>`__                                       | |iOBC-Supervisor|                                                                            |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `NovAtel <https://www.novatel.com>`__                     | `OEM6 GNSS Receivers <https://www.novatel.com/products/gnss-receivers/oem-receiver-boards/oem6-receivers/>`__ | |NovAtel-OEM6|                                                                               |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `NearSpace Launch <https://www.nearspacelaunch.com>`__    | `EyeStar-D2 Duplex Radio <https://www.nearspacelaunch.com/collections/eyestar-radiosolutions>`__              | |NSL-Duplex-D2|                                                                              |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+
| `Pumpkin <https://www.pumpkinspace.com>`__                | `All Pumpkin MCUs <https://www.pumpkinspace.com/store/c1/Featured_Products.html>`__                           | `Yes <https://github.com/kubos/kubos/blob/master/services/pumpkin-mcu-service/README.rst>`__ |
+-----------------------------------------------------------+---------------------------------------------------------------------------------------------------------------+----------------------------------------------------------------------------------------------+

Contributing to KubOS
---------------------

Want to get your code into space? Become a :doc:`KubOS contributor <contributing/index>` and you will!
We welcome community developers, and are always looking for new people to collaborate with us.
`Join us on Slack <https://slack.kubos.co/>`__ or visit our
`GitHub Issues <https://github.com/kubos/kubos/issues>`__ page to participate in discussion of features
or bugs, see what you can work on, and to give feedback.

.. toctree::
    :hidden:

    KubOS Design <kubos-design>

.. toctree::
   :hidden:
   :titlesonly:

   Getting Started <getting-started/index>

.. toctree::
    :hidden:

    Tutorials <tutorials/index>
    Working with an OBC <obc-docs/index>
    Mission Development <mission-dev/index>
    KubOS Ecosystem <ecosystem/index>
    Under the Hood <deep-dive/index>
    Kubos SDK <sdk-docs/index>
    Contributing to KubOS <contributing/index>
    Changelog <changelog>
    FAQs <faq-troubleshooting>

Indices and tables
------------------

* :ref:`genindex`
* :ref:`search`

.. |MAI-400| raw:: html

    <a href="rust-docs/mai400_service/index.html" target="_blank">Yes</a>

.. |Clydespace-EPS| raw:: html

    <a href="rust-docs/clyde_3g_eps_service/index.html" target="_blank">Yes</a>

.. |ISIS-AntS| raw:: html

    <a href="rust-docs/isis_ants_service/index.html" target="_blank">Yes</a>

.. |iOBC-Supervisor| raw:: html

    <a href="rust-docs/iobc_supervisor_service/index.html" target="_blank">Yes</a>

.. |NovAtel-OEM6| raw:: html

    <a href="rust-docs/novatel_oem6_service/index.html" target="_blank">Yes</a>

.. |NSL-Duplex-D2| raw:: html

    <a href="rust-docs/nsl_duplex_d2_comms_service/index.html" target="_blank">Yes</a>
