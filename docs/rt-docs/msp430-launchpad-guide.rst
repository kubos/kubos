MSP430 Discovery Board Guide
============================

Reference Documents
-------------------

**MSP430 Documentation:**

-  `MSP430x5xx User's
   Guide <http://www.ti.com/lit/ug/slau208p/slau208p.pdf>`__ - Main
   manual. Describes the setup and configuration for the whole
   microcontroller.

-  `MSPEXP430F5529LP Quick Launch
   Guide <http://www.ti.com/lit/ml/slau536/slau536.pdf>`__ - Useful for
   the pin layouts.

-  `MSP430F5529 LaunchPad User's
   Guide <http://www.ti.com/lit/ug/slau533d/slau533d.pdf>`__ - Contains
   the full pinout schematics

-  `MSP430F5529 Header
   File <http://ece.wpi.edu/courses/ece2049smj/msp430f5529.h>`__ -
   Contains many of the pin and register constants

**Kubos Documentation:**

-  :doc:`Main HAL API documentation <../apis/kubos-hal/index>` - Overview of
   the high-level HAL. Useful for things like k\_uart\_write.
-  :doc:`MSP430F5 Specific HAL API documentation <../apis/kubos-hal/kubos-hal-msp430f5529/index>` -
   Specifics for the MSP430 version of the HAL. Useful for things like
   the configuration options.
-  :doc:`Installing the Kubos SDK <../installation-docs/sdk-installing>` - Basics of
   setting up the Kubos SDK environment
-  :doc:`Creating your first KubOS RT project <../rt-docs/first-rt-project>` - Steps to
   create and build a Kubos SDK project
-  :doc:`SDK Command Reference <../sdk-docs/sdk-reference>` - Overview of the
   common Kubos SDK commands

Pin Definitions
---------------

Unlike the STM32F4, there is only a single set of pin definitions for
each communication bus.

**Note:** There are two UART buses present in the launchpad. However,
UART2 is piped through to the micro-USB port and is reserved for console
output.

Finding the Pin Definitions for a Board
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

In order to locate the pin assignments for the MSP430 launchpad, please
refer to the following documents.

MSPEXP430F5529LP Quick Launch Guide:
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The first page of the `MSPEXP430F5529LP Quick Launch
Guide <http://www.ti.com/lit/ml/slau536/slau536.pdf>`__ has a pin
diagram for the board. The primary I2C, SPI, and UART buses are
color-coded.

-  I2C pins - UCB\_n\_\_{SCL\|SDA}
-  SPI pins - UC\_cn\_\_{SIMO\|SOMI\|CLK}
-  UART pins - UCA0\_{TXD\|RXD}

MSP430F5529 Launchpad User's Guide:
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The full pinout schematic of the processor can be found in section 2.2.1
of the `MSP430F5529 LaunchPad User's
Guide <http://www.ti.com/lit/ug/slau533d/slau533d.pdf>`__.

Startup Code
------------

Each project will contain a main() function.

--------------

**SUPER IMPORTANT** The MSP430's watchdog is on by default. It must
be manually turned off at the start of your program or you must create a
task to feed the watchdog. If you don't, the watchdog will continually
be starved and reboot your board.

--------------

You should also make a call to enable interrupts at startup.

Your main() should look something like this:

::

    int main(void) {

        /* Stop the watchdog. */
        WDTCTL = WDTPW + WDTHOLD;
        
        __enable_interrupt();
        
        xTaskCreate(task_blink, "blink", configMINIMAL_STACK_SIZE * 4, NULL, 2, NULL);
         
        vTaskStartScheduler();
        
        return 0;
    }

Configuration Notes
-------------------

The MSP430's inter-device communication methods do not support all of
the same options as the STM32F4. For example, the MSP430 does not
support 1-wire half-duplex SPI communication. Please refer to the User's
Guide or the :doc:`MSP430's HAL Documentation <../apis/kubos-hal/kubos-hal-msp430f5529/index>` 
for all of the supported options.

`UART <kubos-hal/uart_api.html#_CPPv29KUARTConf>`__ - Word length - Does not
support 9-bit mode

`I2C <kubos-hal/i2c_api.html#_CPPv28KI2CConf>`__ - Currently has all the same
capabilities as the STM32F4

`SPI <kubos-hal/spi_api.html#_CPPv28KSPIConf>`__ - Direction - Does not support
1-line mode - Data Size - Does not support 16-bit mode

Flashing the Board
------------------

Once you've built your project, you'll flash it onto your board using
the micro-USB port. You might need to install drivers in order for the
board to be properly detected by your computer. If you're using Windows,
the drivers can be found
`here <http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSP430_FET_Drivers/latest/index_FDS.html>`__.

If you have a Kubos vagrant image running, the USB connection should be
automatically passed through to the VM. The board should appear as the
"Texas Instruments" device if you issue the ``lsusb`` command. Run
``kubos flash`` in order to start the flash process.

If you see a "*No unused FET found*" message, the board either isn't
plugged into your computer or some other VM has control of the USB (only
one VM can have control of the USB at a time).

If you see any other error messages, like "*device initialization
failed*" re-run the flash command.

**Note:** The MSP430 flasher can be finicky. It may take several
attempts to successfully flash the board. If you reach a state where
you've run the flash commands many times and it still won't complete
successfully, try restarting your machine/VM.

The output of a successful flash should look like this:

::

    MSPDebug version 0.22 - debugging tool for MSP430 MCUs
    Copyright (C) 2009-2013 Daniel Beer <dlbeer@gmail.com>
    This is free software; see the source for copying conditions.  There is NO
    warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

    MSP430_GetNumberOfUsbIfs
    MSP430_GetNameOfUsbIf
    Found FET: ttyACM0
    MSP430_Initialize: ttyACM0
    Firmware version is 30403004
    MSP430_VCC: 3000 mV
    MSP430_OpenDevice
    MSP430_GetFoundDevice
    Device: MSP430F5529 (id = 0x0030)
    8 breakpoints available
    MSP430_EEM_Init
    Chip ID data: 55 29 19
    Erasing...
    Programming...
    Writing 4096 bytes at 4400 [section: .text]...
    Writing 4096 bytes at 5400 [section: .text]...
    Writing 4096 bytes at 6400 [section: .text]...
    Writing 4096 bytes at 7400 [section: .text]...
    Writing 4096 bytes at 8400 [section: .text]...
    Writing 2044 bytes at 9400 [section: .text]...
    Writing 1200 bytes at 9bfc [section: .rodata]...
    Writing   12 bytes at a0ac [section: .data]...
    Writing  128 bytes at ff80 [section: .vectors]...
    Done, 23864 bytes total
    MSP430_Run
    MSP430_Close

If something happens to the board's flashing firmware
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

It's possible for the version of the board's internal firmware to be
out-of-date from what the flash application is looking for. In this
case, the ``kubos flash`` command will automatically kick off the
firmware updater.

(This is guaranteed to happen if you change the OS of the computer that
the board is connected to.)

It will look something like this:

::

    MSPDebug version 0.22 - debugging tool for MSP430 MCUs
    Copyright (C) 2009-2013 Daniel Beer <dlbeer@gmail.com>
    This is free software; see the source for copying conditions.  There is NO
    warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

    MSP430_GetNumberOfUsbIfs
    MSP430_GetNameOfUsbIf
    Found FET: ttyACM0
    MSP430_Initialize: ttyACM0
    FET firmware update is required.
    Starting firmware update (this may take some time)...
    Initializing bootloader...
    Programming new firmware...
        75 percent done
        84 percent done
        [...]
        84 percent done
        91 percent done
       100 percent done
    tilib: MSP430\_FET\_FwUpdate: MSP-FET / eZ-FET core(communication layer) update failed (error = 74)
    tilib: device initialization failed

If you're using a VM, the "Texas Instruments MSP Tools Driver" device
name might no longer be present after running the command.

If that occurs, try selecting the "Unknown device" devices and
re-running the ``kubos flash`` command. The command should cause some
additional firmware to be loaded and the usual device name should appear
once it completes.

Select the "Texas Instruments MSP Tools Driver" device again and rerun
the flash command one more time. You should see firmware upload
messages, followed by your program being flashed to the board.

::

    MSPDebug version 0.22 - debugging tool for MSP430 MCUs
    Copyright (C) 2009-2013 Daniel Beer <dlbeer@gmail.com>
    This is free software; see the source for copying conditions.  There is NO
    warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

    MSP430_GetNumberOfUsbIfs
    MSP430_GetNameOfUsbIf
    Found FET: ttyACM0
    MSP430_Initialize: ttyACM0
    FET firmware update is required.
    Starting firmware update (this may take some time)...
    Initializing bootloader...
    Programming new firmware...
         4 percent done
         [...]
        84 percent done
        84 percent done
       100 percent done
    Update complete
    Done, finishing...
    MSP430_VCC: 3000 mV
    MSP430_OpenDevice
    MSP430_GetFoundDevice
    Device: MSP430F5529 (id = 0x0030)
    8 breakpoints available
    MSP430_EEM_Init
    Chip ID data: 55 29 19
    Erasing...
    Programming...
    Writing 4096 bytes at 4400 [section: .text]...
    Writing 4096 bytes at 5400 [section: .text]...
    Writing 4096 bytes at 6400 [section: .text]...
    Writing 4096 bytes at 7400 [section: .text]...
    Writing 4096 bytes at 8400 [section: .text]...
    Writing 2044 bytes at 9400 [section: .text]...
    Writing 1200 bytes at 9bfc [section: .rodata]...
    Writing   12 bytes at a0ac [section: .data]...
    Writing  128 bytes at ff80 [section: .vectors]...
    Done, 23864 bytes total
    MSP430_Run
    MSP430_Close

Debug Console
-------------

You can view the MSP430's console output by creating a serial connection
to the micro-USB port.

All of your program's printf statements will be routed through here. You
can change the settings of the console with the :json:object:`hardware.console` section
of the config.json file.

**NOTE:** If your MSP430 board loses power while you have a debug
console connection open, you might need to close the current console and
turn the board off and back on again in order to create a new successful
console session.

Default SDK Connection
~~~~~~~~~~~~~~~~~~~~~~

+-----------+---------+
| Option    | Setting |
+===========+=========+
| Baud rate | 115200  |
+-----------+---------+
| Bits      | 8       |
+-----------+---------+
| Parity    | N       |
+-----------+---------+
| Stop bits | 1       |
+-----------+---------+

If you are using the default communication settings, you can bring up
a serial connection in your SDK box using the ``minicom msp430`` command.
This opens a minicom session with the MSP430.

Non-Default SDK Connection
~~~~~~~~~~~~~~~~~~~~~~~~~~

If you have non-default communication settings, you can create a custom
minicom configuration.

1. Execute the ``minicom -s`` command to bring up the minicom settings menu.
   **Note:** If you would like to save this configuration in a new configuration 
   file, use ``sudo minicom -s`` instead.
2. Select the 'Serial port setup' option and press enter to open the serial port 
   setup menu.
3. Press the key that corresponds to the option you want to change.
    a. If the option is a text field, the cursor will appear in that field. 
       Update the option and then hit 'Enter' in order for the changes to be applied.
    b. If the option is a boolean, pressing the option key will toggle the setting 
       value (no need to hit 'Enter').
4. When all of the needed options have been updated, hit 'Enter' to save and return 
   to the previous menu.
5. If you would like to save your configuration:
    a. Select the 'Save setup as..' option and hit 'Enter'.
    b. Type in your desired configuration name and hit 'Enter'. This will create
       a new configuration file in */etc/minicom/minirc.{name}`*.
6. Select 'Exit' and hit 'Enter' to enter the serial connection session.

Exitting the SDK Connection
~~~~~~~~~~~~~~~~~~~~~~~~~~~

To exit a minicom session:

- Press Ctrl+A.
- Press 'Q' (quit with no reset) or 'X' (exit and reset). A confirmation dialog will appear.
- Hit 'Enter' to exit the session.

Example Program
---------------

Let's create a basic MSP430 program.

The goal is to create a program that will output a message once a
second. Additionally, a message should be issued when button 0 (P2.1) is
pressed.

Note: This is more simple than the STM32F4 example program because there
are no inter-device pins that can be connected to each other. UART2 is
dedicated to the debug console and slave mode hasn't been implemented
for I2C or SPI.

The Walkthrough:
~~~~~~~~~~~~~~~~

Create the project

::

    $ kubos init msp-test

Create the program in main.c:

.. code:: c

    /*
     * KubOS RT
     * Copyright (C) 2016 Kubos Corporation
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

    #include "kubos-hal/gpio.h"
    #include "kubos-hal/uart.h"

    /*
     * Flash specified LED for 100 milliseconds
     */
    static inline void blink(int pin) {
        k_gpio_write(pin, 1);
        vTaskDelay(100 / portTICK_RATE_MS);
        k_gpio_write(pin, 0);
    }

    /*
     * Print out a message when button 0 (P2.1) is pushed
     * and blink the red LED
     */
    void task_button_press(void *p) {
       int signal = 1;

        while (1) {
            if (k_gpio_read(K_BUTTON_0)) {

                printf("Button_0 pressed\r\n");
                blink(K_LED_RED);
            }
            vTaskDelay(100 / portTICK_RATE_MS);
        }
    }

    /*
     * Print out a basic message every 2 seconds
     * and blink the green LED
     */
    void task_echo(void *p) {
        static int x = 0;
        while (1) {
            printf("echo, x=%d\r\n", x);
            x++;
            blink(K_LED_GREEN);
            vTaskDelay(2000 / portTICK_RATE_MS);
        }
    }

    int main(void)
    {
        //Initialize the debug console (by default, UART2 @ 115200)
        k_uart_console_init();

        //Initialize the LEDs and button
        k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
        k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
        k_gpio_init(K_BUTTON_0, K_GPIO_INPUT, K_GPIO_PULL_UP);

        // Stop the watchdog
        WDTCTL = WDTPW + WDTHOLD;

        //Create the subtasks
        xTaskCreate(task_button_press, "BUTTON", configMINIMAL_STACK_SIZE, NULL, 3, NULL);
        xTaskCreate(task_echo, "ECHO", configMINIMAL_STACK_SIZE, NULL, 2, NULL);

        //Start the task scheduler
        vTaskStartScheduler();

        while (1);

        return 0;
    }

Set the target

::

    $ kubos target msp430f5529-gcc

Build the program

::

    $ kubos build

Flash the program

::

    $ kubos flash

Connect to the debug console. Should see an "echo, x=\ *n*" message
every second. If you press the P2.1 button, you should see "Button\_0
pressed".
