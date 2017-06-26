STM32F4 Discovery Board Guide
=============================

Reference Documents
-------------------

**STM32F4 Documentation:** 

These are the two most useful documents to have while working with the STM32F4:

-  `STM32F4 Reference
   Manual <http://www.st.com/content/ccc/resource/technical/document/reference_manual/3d/6d/5a/66/b4/99/40/d4/DM00031020.pdf/files/DM00031020.pdf/jcr:content/translations/en.DM00031020.pdf>`__
   Main manual. Describes the setup and configuration for the whole
   board.

-  `STM32F4 Discovery Board User
   Manual <http://www.st.com/content/ccc/resource/technical/document/user_manual/70/fe/4a/3f/e7/e1/4f/7d/DM00039084.pdf/files/DM00039084.pdf/jcr:content/translations/en.DM00039084.pdf>`__
   Useful for the pin layouts.

**Kubos Documentation:**

-  :doc:`Main HAL API documentation <./kubos-hal/index>` - Overview of
   the high-level HAL. Useful for things like k\_uart\_write.
-  FIXME:`STM32F4 Specific HAL API
   documentation <./kubos-hal/kubos-hal-stm32f4/index.html>`__ -
   Specifics for the STM32F4 version of the HAL. Useful for things like
   the configuration options.
-  :doc:`Installing the Kubos SDK <sdk-installing>` - Basics of
   setting up the Kubos SDK environment
-  :doc:`Creating your first KubOS RT project <first-rt-project>` - Steps to
   create and build a Kubos SDK project
-  :doc:`SDK Command Reference <sdk-reference>` - Overview of the
   common Kubos SDK commands

Pin Definitions
---------------

There are multiple pin definitions for many of the buses. You can find a
descriptive picture of all of the pins
`here <https://chippedwood.files.wordpress.com/2015/07/stm32f4-discovery-with-spi-pinout-wiring.png>`__.

For example, I2C bus 1 can use pins PB6 and PB7, pins PB8 and PB9, or
some combination.

In order to see which pins are actually being used you'll need to refer
to the source code's target.json file or your project's yotta\_config.\*
files.

Finding the Pin Definitions for a Board
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Within the User Manual
^^^^^^^^^^^^^^^^^^^^^^

The `STM32F4 Discovery Board User
Manual <http://www.st.com/content/ccc/resource/technical/document/user_manual/70/fe/4a/3f/e7/e1/4f/7d/DM00039084.pdf/files/DM00039084.pdf/jcr:content/translations/en.DM00039084.pdf>`__
lists all pin definitions in section 6.11 (Extension connectors)

-  I2C pins - I2C\_n\_\_{SCL\|SDA}
-  SPI pins - SPI\_n\_\_{MISO\|MOSI\|SCK}
-  UART pins - USART\_n\_\_{TX\|RX} or UART\_n\_\_{TX\|RX}

Within the Kubos SDK
^^^^^^^^^^^^^^^^^^^^

**Note:** You must have built your project in order for these files to
be available.

Run the config command:

::

    kubos config

Look for the I2C, SPI, or UART section to find the pin definitions.

Alternatively, you can manually look at the yotta config files.

Look at:

::

    {Kubos SDK project}/build/stm32f407-disco-gcc/yotta_config.h

To see the YOTTA\_CFG\_\* defines. Look for
YOTTA\_CFG\_HARDWARE\_{I2C\|SPI\|UART} to find the pin definitions

Or, look at:

::

    {Kubos SDK project}/build/stm32f407-disco-gcc/yotta_config.json

To see the configuration of the project.
Look for the I2C, SPI, or UART section to find the pin definitions

Within the Source Code
^^^^^^^^^^^^^^^^^^^^^^

The target.json file for the STM32F4 contains the default pin
configuration.

Look at:

::

    kubos/targets/target-stm32f407-disco-gcc/target.json

Look for the I2C, SPI, or UART section to find the pin definitions

Changing the pin definitions for a board
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

In order to use non-default pins, you'll need to update the :json:object:`hardware`
section of your config.json file for your project.

The format of the file will mirror the organization in the
target-stm32f407-disco-gcc/target.json file.

Let's say you want to use the alternate pins for I2C bus 1. Your
config.json file should look like this:

::

    {
        "hardware": {
            "i2c": {
                "i2c1": {
                    "scl": {
                        "pin": "PB8"
                    },
                    "sda": {
                        "pin": "PB9"
                    }
                }
            }
        }
    }

**Note**: Any parameters that aren't explicitly specified will default
to the value in the target.json file.

The project will need to be rebuilt (``kubos build``) in order to
incorporate any new configuration changes.

Flashing the Board
------------------

Once you've built your project, you'll flash it onto your board using
the mini-USB port. If your host machine is running Windows, you may need
to install the `STM32F4
drivers <http://www.st.com/content/st_com/en/products/embedded-software/development-tool-software/stsw-link009.html>`__
in order for the board to be properly detected by your computer.

If you're using a Kubos SDK box, the USB connection should be
automatically passed through to the box and available for use.

Run ``kubos flash`` in order to start the flash process.

If you see a "No compatible ST-Link device found" message, the board
either isn't plugged into your computer, or the USB hasn't been passed
through to the Kubos SDK box. Only one box can have possession of a USB
device at a time, so make sure that no other boxes are running.

If you see any other error messages, like "error writing to flash at
address 0x08000000 at offset 0x00000000" or "reset device failed",
re-run the flash command.

The output of a successful flash should look like this:

::

    Open On-Chip Debugger 0.9.0 (2015-09-02-10:42)
    Licensed under GNU GPL v2
    For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
    WARNING: target/stm32f4x_stlink.cfg is deprecated, please switch to target/stm32f4x.cfg
    Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
    Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
    adapter speed: 2000 kHz
    adapter_nsrst_delay: 100
    none separate
    trst_only separate trst_push_pull
    stm_run
    Info : Unable to match requested speed 2000 kHz, using 1800 kHz
    Info : Unable to match requested speed 2000 kHz, using 1800 kHz
    Info : clock speed 1800 kHz
    Info : STLINK v2 JTAG v14 API v2 SWIM v0 VID 0x0483 PID 0x3748
    Info : using stlink api v2
    Info : Target voltage: 2.877745
    Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
    target state: halted
    target halted due to debug-request, current mode: Thread 
    xPSR: 0x01000000 pc: 0x0800c0e8 msp: 0x20008188
    Info : device id = 0x10076413
    Info : flash size = 1024kbytes
    stm32f2x unlocked.
    INFO: a reset or power cycle is required for the new settings to take effect.
    auto erase enabled
    Info : Padding image section 0 with 632 bytes
    wrote 131072 bytes from file /home/catherine/kubos-hal-test/build/stm32f407-disco-gcc/source/kubos-hal-test in 9.705738s (13.188 KiB/s)
    shutdown command invoked

Debug Console
-------------

You can view the STM32F4's console output by connecting an `FTDI serial
cable <https://cdn-shop.adafruit.com/1200x900/70-03.jpg>`__ to the
board's console UART bus (default is UART6, baud rate @ 115200).

-  The yellow wire is the TX wire (default pin PC6).
-  The orange wire is the RX wire (default pin PC7).

All of your program's printf statements will be routed through here. You
can change the settings of the console with the :json:object:`hardware.console` section
of the config.json file.

FDTI connections are also automatically passed through to the Kubos SDK
box and will be available as the '/dev/FTDI' device. Minicom is
pre-installed and can be used to connect to the board with the
``minicom kubos`` command.

Example Program
---------------

Let's create a basic STM32F4 program.

The goal is to use UART2 to talk to UART4. UART2 will transmit a ping
every second. UART4 will be listening for messages and will print out
anything that is received to the console.

We'll be using default everything, so there is no need to create a
config.json file.

(Why UART2 and UART4? Because their pins are right next to each other)

The Walkthrough:
~~~~~~~~~~~~~~~~

Connect UART2 and UART4

-  PA2 to PA1 (UART2 TX to UART4 RX)
-  PA0 to PA3 (UART4 TX to UART2 RX)

Create the project

::

    $ kubos init stm-test

Create the program in main.c:

.. code:: c

    /*
     * KubOS RT
     * Copyright (C) 2017 Kubos Corporation
     *
     * Licensed under the Apache License, Version 2.0 (the "License");
     * you may not use this file except in compliance with the License.
     * You may obtain a copy of the License at
     *
     *     http://www.apache.org/licenses/LICENSE-2.0
     *
     * Unless required by applicable law or agreed to in writing, software
     * distributed under the License is distributed on an "AS IS" BASIS,
     * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
     * See the License for the specific language governing permissions and
     * limitations under the License.
     */

    #include "kubos-hal/uart.h"

    /*
     * Transmitter task.  Should send a ping message via uart every 2 seconds.
     */
    void task_transmitter(void *p) {

        KUARTConf config;
        char * ping = "ping";
        int len = strlen(ping);

          /*
           * Load the uart configuration defaults:
           *   Baud = 9600
           *   Word length = 8
           *   Stop bits = 1
           *   Parity = none
           *   RX queue len = 128
           *   TX queue len = 128
           */
        config = k_uart_conf_defaults();

        //Initialize the uart bus
        k_uart_init(K_UART2, &config);

        while (1) {

            //Write the ping string out of the uart bus
            k_uart_write(K_UART2, ping, len);

                //Delay 1 second
            vTaskDelay(1000 / portTICK_RATE_MS);
        }
    }

    /*
     * Receiver task.  Will print out any received data.
     */
    void task_receiver(void *p) {

        KUARTConf config;
        char buffer[10] = {0};
        int bytesRead = 0;

        //Load the uart configuration defaults
        config = k_uart_conf_defaults();

        //Initialize the uart bus
        k_uart_init(K_UART4, &config);

        while (1) {

                //Read in any received bytes
            bytesRead = k_uart_read(K_UART4, buffer, sizeof buffer);

            if(bytesRead > 0)
            {
                printf("Received: %s\r\n", buffer);
            }

                //Give a small delay before trying to receive again
            vTaskDelay(100);
        }
    }

    //Main function.  The program will start here.
    int main(void)
    {
         //Initialize the debug console (by default, UART6 @ 115200)
        k_uart_console_init();

         //Create the transmitter and receiver tasks
        xTaskCreate(task_transmitter, "TRANSMITTER", configMINIMAL_STACK_SIZE, NULL, 2, NULL);
        xTaskCreate(task_receiver, "RECEIVER", configMINIMAL_STACK_SIZE, NULL, 2, NULL);

        //Start the task scheduler
        vTaskStartScheduler();

        while (1);

        return 0;
    }

Set the target

::

    $ kubos target stm32f407-disco-gcc

Build the program

::

    $ kubos build

Flash the program

::

    $ kubos flash

Connect to the debug console (UART6). Should see a "Received: ping"
message every second.
