# KubOS RT Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/openkosmosorg/KubOS-rt).

Fork away and start building your own applications!

## Building

1. Clone our top level [Kubos project](https://github.com/openkosmosorg/KubOS)

2. Bootstrap our projects (this will also link the local yotta modules)

        $ cd KubOS

        $ ./bootstrap.sh

3. Setup your build environment:

    1. We recommend using Docker to quickly setup your environment. Our Dockerfile and instructions can be found [here](https://github.com/openkosmosorg/KubOS-rt)

    1. Want to build locally? Be sure to install these first

        1. Install ARM's [yotta build system](http://yottadocs.mbed.com/#installing)
        2. Install CMake 3.x
        3. Install the [ARM GCC toolchain](https://github.com/RIOT-OS/RIOT/wiki/Family:-ARM)

4. Navigate to our example code

        $ cd examples/kubos-rt-example

5. Install our custom `stm32f407-disco-gcc` target

        $ yotta target stm32f407-disco-gcc

6. Build the example app included with KubOS RT

        $ yotta build -- -v

## Flashing

1. Plugin your STM32F407 Discovery Board via USB. You should see LEDs turn on indicating power.

2. Be sure you have OpenOCD (0.9.x or above) and `arm-none-eabi-gdb` installed.

3. If you are working inside of a VM you will need to pass the STM USB device into the VM.

4. Run this command to flash your app

        $ yotta start


If you see this message:

    No compatible ST-Link device found

Verify that you can access the STM usb device (and that you have permission to).
