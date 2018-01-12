# Kubos SPI SD Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubos/kubos/tree/master/kubos-rt) demonstrating SD over SPI using Kubos Core's FatFS library.

## Project Overview

This application runs a series of commands against an SD card connected to SPI bus 1 using the FatFS library.

The application covers how to:

Mount/unmount a SD card
  - Open a file for writing (file will be created if it doesn’t exist)
  - Open a file for reading
  - Close a file
  - Write a string to a file
  - Read a specified length from a file
  - Sync the file system
  - Get the stats (size, timestamp, attributes) of a file

## Connection Details

SPI Bus: K_SPI1

MSP430F5529 Launchpad:
  - MOSI - P3.0
  - MISO - P3.1
  - SCK  - P3.2
  - CS   - P3.7

STM32F407 Discovery:
  - MOSI - PA7
  - MISO - PA6
  - SCK  - PA5
  - CS   - PA4
 
## Project Requirements

To successfully run this project as-is, you must connect an SD-over-SPI device.

By default, this project is configured to run on the MSP430F5529 target. In order to run on the STM32F407 target, please update the chip select definition in the *config.json* file in this directory to be "PA4" instead of "P37".

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubos/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-sd-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board (let's assume it's an STM32F407 Discovery board)

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash

Connect the SD device and then power the board.

See the [STM32F4 discovery board Guide](docs.kubos.co/stm32f4-discovery-board-guide.html) or 
[MSP430 launchpad guide](docs.kubos.co/msp430-launchpad-guide.html) for information on how to connect to the
debug console and see the output from this project. 

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/latest/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/latest/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/latest/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/latest/sdk-project-config.html)
- [Kubos HAL I2C Guide](http://docs.kubos.co/latest/kubos-hal/i2c.html)
- [STM32F4 Discovery Board Guide](http://docs.kubos.co/latest/stm32f4-discovery-board-guide.html) 
- [MSP430 Launchpad Guide](http://docs.kubos.co/latest/msp430-launchpad-guide.html) 
