KubOS for the Pumpkin MBM2
==========================

.. toctree::
    :maxdepth: 1

    installing-linux-mbm2
    working-with-the-mbm2
    
Reference Documents
-------------------

`Pumpkin MBM2 Product Page <https://www.pumpkinspace.com/store/p208/mbm2.html>`__

The :title:`CubeSat Kit Motherboard Module (MBM) 2` reference document
is available from Pumpkin and is a useful document for learning what
each of the hardware components are and how they are connected.

Debug Connection
----------------

The Pumpkin MBM2 should be shipped with a USB Debug Adapter board.

The white connection cable should be plugged into the labeled "UART0"
port on the edge of the board, with the exposed pins facing up.

The USB cable can then be plugged into your computer. Any required
drivers should be automatically installed.

This connection will be passed through to a Kubos Vagrant image as
`/dev/FTDI` and will be used for the serial console.

Ethernet Connection
-------------------

Please refer to our :ref:`ethernet overview <ethernet>` for help setting up an SSH connection over
ethernet.

Peripherals
-----------

By default, KubOS exposes the following peripheral components:

- Ethernet
- 1 real-time clock (RTC)
- 1 I2C bus
- 5 UART buses
- 7 ADC pins
- 6 GPIO pins

More information about these peripheral devices can be found in the
:ref:`Working with the Pumpkin MBM2 <peripherals-mbm2>` doc.