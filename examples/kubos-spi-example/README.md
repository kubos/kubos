# Kubos SPI Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/kubos/tree/master/kubos-rt) demonstrating our SPI HAL.

## Project Overview

This application gives two examples of how to interact with SPI devices in a Kubos project:

If the BME280 sensor has been defined in the project’s config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.

If no sensor has been defined in the project’s config.json file, then this application will initialize a generic SPI connection over SPI bus 1.
  - The application will then enter a loop and attempt to send and receive a dummy byte.
  - **Note**: This case is not a complete example, because it omits the manual manipulation of a chip select pin that is required for SPI communication.

## Connection Details

SPI bus: K_SPI1
  - SDI - PA7
  - SDO - PA6
  - SCK - PA5
  - CS  - PA4
  
## Project Requirements

To successfully run this project as-is, you must connect a BME280 sensor to an STM32F407 Discovery board.

To use a different SPI device, you should edit the *config.json* file in this directory and remove the "sensors" section.
The device should be capable of operating in bi-directional mode with an 8-bit data size at 10kHz.

This project is intended for the STM32F407 target only. The MSP430 does not currently have support for floating point variables, so this example project will compile but not successfully run on the MSP430 target.

You can view the STM32F4's console output by connecting an [FTDI serial cable](https://cdn-shop.adafruit.com/1200x900/70-03.jpg) to the board's console UART bus (default is UART6, baud rate @ 115200).

-  The yellow wire is the TX wire (default pin PC6).
-  The orange wire is the RX wire (default pin PC7).

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-spi-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash

Connect the sensor and then power the board.

Create a minicom session in order to connect to your board and see the project output

    $ minicom kubos

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/1.0.0/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/1.0.0/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/1.0.0/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/1.0.0/sdk-project-config.html)
- [Kubos HAL SPI Guide](http://http://docs.kubos.co/1.0.0/kubos-hal/spi.html)
- [STM32F4 Discovery Board Guide](http://docs.kubos.co/1.0.0/stm32f4-discovery-board-guide.html)
