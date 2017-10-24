
.. figure:: images/kubos_logo.png
    :align: center

Overview
========

The Kubos platform provides small satellite developers the tools and
libraries necessary to quickly bring up space ready software. We
leverage multiple existing open source projects like FreeRTOS and CSP,
along with our own custom framework and SDK.

Looking to build an application on Kubos? Check out our quick start guides:

 - :doc:`KubOS RT quick start guide <first-rt-project>`
 - :doc:`KubOS Linux quick start guide <first-linux-project>`

Having issues? Want a new feature? `Come talk to
us! <https://slack.kubos.co/>`__

If for some reason Slack won't work for you, feel free to email us at
info@kubos.co.

Boards Currently Supported by Kubos
-----------------------------------

+------------+------------------------------------------------------------------+--------------+
| MCU Family | Description                                                      | Supported OS |
+============+==================================================================+==============+
| STM32F4    | :doc:`STM32F407 Discovery Board <stm32f4-discovery-board-guide>` | KubOS RT     |
+------------+------------------------------------------------------------------+--------------+
|            | STM32F405 PyBoard                                                | KubOS RT     |
+------------+------------------------------------------------------------------+--------------+
|            | STM32F405 NanoAvionics SatBus 3c0 OBC                            | KubOS RT     |
+------------+------------------------------------------------------------------+--------------+
| MSP430     | :doc:`MSP430F5529 Launchpad <msp430-launchpad-guide>`            | KubOS RT     |
+------------+------------------------------------------------------------------+--------------+
| ISIS       | :doc:`ISIS-OBC <working-with-the-iobc>`                          | KubOS Linux  |
+------------+------------------------------------------------------------------+--------------+
| Pumpkin    | :doc:`Pumpkin Motherboard Module 2 <working-with-the-mbm2>`      | KubOS Linux  |
+------------+------------------------------------------------------------------+--------------+
| Beaglebone | :doc:`Beaglebone Black, Rev. C <working-with-the-bbb>`           | KubOS Linux  |
+------------+------------------------------------------------------------------+--------------+


.. toctree::
    :hidden:

    First Time Users <first-time-users>
    FAQs and Troubleshooting Tips <faq-troubleshooting>
    Installation Docs <installation-docs>
    Examples <sdk-examples>
    Kubos SDK <sdk-docs>
    KubOS RT <rt-docs>
    KubOS Linux <linux-docs>
    Kubos Middleware <middleware>
    Kubos APIs <apis>
    Developer Docs <dev-docs>

Indices and tables
------------------

* :ref:`genindex`
* :ref:`search`
