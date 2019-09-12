KubOS for the Beaglebone Black
==============================

.. toctree::
    :maxdepth: 1

    installing-linux-bbb
    working-with-the-bbb
    
Reference Documents
-------------------

- `Beaglebone Black Web Page <https://beagleboard.org/black>`__
- `Beaglebone Black Wiki <http://elinux.org/Beagleboard:BeagleBoneBlack>`__
- `Beaglebone Black Hardware Diagrams <http://beagleboard.org/Support/bone101/#hardware>`__
- `Beaglebone Black System Reference Manual Rev C <http://static6.arrow.com/aropdfconversion/8fff89aa85f5c451318cbdee2facd9c9fac36872/bbb_srm.pdf>`__

Debug Console Connection
------------------------

As documented in section 7.5 of the :title:`Beaglebone Black System
Reference Manual`, an FTDI cable can be connected to the serial debug
connector in order to establish a debug console connection.

This connection will be passed through to a Kubos Vagrant image as
`/dev/FTDI`.

Ethernet Connection
-------------------

Please refer to our :ref:`ethernet overview <ethernet>` for help setting up an SSH connection over
ethernet.

Peripherals
-----------

By default, KubOS exposes the following peripheral components:

- Ethernet
- 2 I2C buses
- 1 SPI bus
- 5 UART buses
- 7 ADC pins
- Many GPIO pins

More information about these peripheral devices can be found in the :doc:`working-with-the-bbb` doc.