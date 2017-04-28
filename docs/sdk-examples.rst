Kubos SDK Example Applications
==============================

We have provided a variety of example applications to help you get started with your Kubos project.
These examples are located in the 'Examples' folder of the `Kubos repo <http://github.com/kubostech/kubos/examples>`__, 
as well as within the $HOME/.kubos/kubos/examples folder.

This document was created to give a brief overview of each of the examples.

Using an Example Application
----------------------------

Each of the example applications contains the files necessary to run as an independent Kubos project. 
This means you can simply copy the desired example to a new folder and immediately, successfully build it with the ``kubos build`` command.

"Compatible Targets" indicates which target boards the application should work on without modification.

**Note:** The default target for all of these applications is ``stm32f407-disco-gcc``. 
You will need to manually change the target if this is not your desired endpoint device. 

**Exception:** The kubos-linux-example application was designed specifically to run on KubOS Linux. 
Since the ``stm32f407-disco-gcc`` target does not support KubOS Linux, this application will fail to build sucessfully until the target is changed to a board which is KubOS Linux-compatible.

Kubos RT Example
----------------

`kubos-rt-example <http://github.com/kubostech/kubos/examples/kubos-rt-example>`__

**Compatible Targets: All KubOS RT compatible targets**

Components Used:

+----------------------+-----------------+
| High-level Component | Specific Area   |
+======================+=================+
| Kubos HAL            | GPIO            |
+----------------------+-----------------+
| CSP                  | Sockets, ping   |
+----------------------+-----------------+
| FreeRTOS             | Threads, queues |
+----------------------+-----------------+

This is the default application included when the `kubos init` command is executed.

Four threads are created:

- A CSP server
- A CSP client
- A button poll
- An interval print

The interval thread prints out ``echo, x={n}`` to the default UART port, where ``{n}`` is a basic counter.

The button poll thread continually polls the board's button to see if it has been pressed.
If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. 
If found, it connects to the CSP server's port and sends a message ``Hello World``.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.

Kubos I2C Example
-----------------

`kubos-i2c-example <http://github.com/kubostech/kubos/examples/kubos-i2c-example>`__

**Compatible Targets: MSP430F5529, STM32F407 Discovery, STM32F405**

Components Used:

+----------------------+------------------------------+
| High-level Component | Specific Area                |
+======================+==============================+
| Kubos SDK            | config.json                  |
+----------------------+------------------------------+
| Kubos Core           | HTU21D sensor, BNO055 sensor |
+----------------------+------------------------------+
| Kubos HAL            | I2C, GPIO                    |
+----------------------+------------------------------+
| FreeRTOS             | Threads                      |
+----------------------+------------------------------+

This application gives several examples of how to interact with I2C devices in a Kubos project:

1. If no sensor has been defined in the project's config.json file, then this application will initialize a generic I2C connection over I2C bus 1 to a slave device with an address of '0x40'.
   It will then write a single byte command of '0xE3' to the slave and attempt to read back a three byte response.
   After this attempt, the application will end.
   
2. If the `HTU21D sensor <https://cdn-shop.adafruit.com/datasheets/1899_HTU21D.pdf>`__ has been defined in the project's config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
   In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.
   
3. If the `BNO055 sensor <https://cdn-shop.adafruit.com/datasheets/BST_BNO055_DS000_12.pdf>`__ has been defined in the project's config.json file, the sensor will be initialized in NDOF (Nine Degrees Of Freedom) mode and then the application will enter a loop.
   In each iteration of the loop, the application will fetch and print the current position data from the sensor to the default UART port.
   
Kubos Sensor Example
--------------------

`kubos-sensor-example <http://github.com/kubostech/kubos/examples/kubos-sensor-example>`__

**Compatible Targets: MSP430F5529, STM32F407 Discovery, STM32F405**

Components Used:

+----------------------+------------------------+
| High-level Component | Specific Area          |
+======================+========================+
| Kubos SDK            | config.json            |
+----------------------+------------------------+
| Kubos Core           | altimeter, temperature |
+----------------------+------------------------+
| FreeRTOS             | Threads                |
+----------------------+------------------------+

This application provides a streamlined approach to using the BME280 humidity/pressure sensor and the HTU21D temperature/humidity sensor.

Kubos CSP Example
-----------------

`kubos-csp-example <http://github.com/kubostech/kubos/examples/kubos-csp-example>`__

**Compatible Targets: All KubOS RT compatible targets**

Components Used:

+----------------------+-----------------+
| High-level Component | Specific Area   |
+======================+=================+
| Kubos HAL            | GPIO            |
+----------------------+-----------------+
| CSP                  | Sockets, ping   |
+----------------------+-----------------+
| FreeRTOS             | Threads, queues |
+----------------------+-----------------+

This application shows an example CSP interaction between client and server tasks.

Three threads are created:

- A CSP server
- A CSP client
- A button poll

The button poll thread continually polls the board's button to see if it has been pressed.
If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. 
If found, it connects to the CSP server's port and sends a message ``Hello World``.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.

Kubos SPI Example
-----------------

`kubos-spi-example <http://github.com/kubostech/kubos/examples/kubos-spi-example>`__

**Compatible Targets: MSP430F5529, STM32F407 Discovery, STM32F405**

Components Used:

This application gives two examples of how to interact with I2C devices in a Kubos project:

1. If no sensor has been defined in the project's config.json file, then this application will initialize a generic SPI connection over SPI bus 1.
   The application will then enter a loop and attempt to send and receive a dummy byte.

   **Note:** This case is not a complete example, because it omits the manual manipulation of a chip select pin that is required for SPI communication.
   
2. If the `BME280 sensor <https://cdn-shop.adafruit.com/datasheets/1899_HTU21D.pdf>`__ has been defined in the project's config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
   In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.

.. todo::

    ** DO NOT REMOVE THE 'TODO' DIRECTIVE UNTIL THE SLASH PR HAS BEEN MERGED **
    
    Kubos Shell Example
    TODO: Insert hyphens here to make this a section header (can't do while in 'todo' state)
    
    **FIXME: What's the purpose of the CSP server? Anything?**
    **FIXME: Where's the Slash documentation?**
    
    `kubos-shell-example <http://github.com/kubostech/kubos/examples/kubos-shell-example>`__
    
    **Compatible Targets: All KubOS RT compatible targets**
    
    Components Used:
    
    +----------------------+-------------------------------------------------+
    | High-level Component | Specific Area                                   |
    +======================+=================================================+
    | Kubos SDK            | config.json                                     |
    +----------------------+-------------------------------------------------+
    | Kubos Slash          | Shell init, custom commands, custom subcommands |
    +----------------------+-------------------------------------------------+
    | Kubos HAL            | LED                                             |
    +----------------------+-------------------------------------------------+
    | FreeRTOS             | Threads                                         |
    +----------------------+-------------------------------------------------+
    
    This application gives an example of how to initialize and interact with the Slash shell library.
    The Slash shell is started with the ``slash_loop`` command. 
    
    Two top-level Slash commands are added:
    
    - ``tasks`` - Displays the current FreeRTOS tasks
    - ``build_info`` - Displays the application's build information.
    
    One Slash command group is also added, ``led``, with the following sub-commands:
    
    - ``led info`` - Lists the LED pins and their colors
    - ``led on <pin>`` - Turns on the specified LED
    - ``led off <pin>`` - Turns off the specified LED
    - ``led blink <pin> [n=1]`` - Turns the LED on and off the specified number of times (default: 1 time)
    
    
