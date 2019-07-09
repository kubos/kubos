KubOS for the ISIS-OBC
======================

.. toctree::
    :maxdepth: 1

    installing-linux-iobc
    working-with-the-iobc
    
Reference Documents
-------------------

`ISIS-OBC Product Page <https://www.isispace.nl/product/on-board-computer/>`__

The :title:`ISIS-OBC Quickstart Guide` should have been packaged with the iOBC and is a useful
document for learning what each of the hardware components are, how to connect them, and what
drivers need to be installed to support them.

Debug Connection
----------------

The iOBC should be shipped with an FTDI cable.
This cable should be connected to the programming adapter, which should then be connected to the
iOBC, to create the debug UART connection. User file transfer will take place using this connection.

Additionally, a `SAM-ICE JTAG <https://www.microchip.com/DevelopmentTools/ProductDetails/AT91SAM-ICE>`__
may also need to be connected in order to successfully connect with an iOBC.

Peripherals
-----------

By default, KubOS exposes the following peripheral components:

- 1 I2C bus
- 1 SPI bus
- 2 UART buses
- 4 analog input pins
- 27 GPIO pins
- 6 PWM pins

More information about these peripheral devices can be found in the :doc:`working-with-the-iobc` doc.