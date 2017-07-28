# Kubos Sensor Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/kubos/tree/master/kubos-rt) demonstrating our sensor interface.

## Project Overview

This application provides a streamlined approach to using the BME280 humidity/pressure sensor and the HTU21D temperature/humidity sensor via
the Kubos Core sensors API.
  
## Project Requirements

To successfully run this project as-is, you must connect at least one of the following:

  - An HTU21D sensor
  - A BME280 sensor

This project is intended for the STM32F407 target only. The MSP430 does not currently have support for floating point variables, so this example project will compile but not successfully run on the MSP430 target.

You can view the STM32F4's console output by connecting an [FTDI serial cable](https://cdn-shop.adafruit.com/1200x900/70-03.jpg) to the board's console UART bus (default is UART6, baud rate @ 115200).

-  The yellow wire is the TX wire (default pin PC6).
-  The orange wire is the RX wire (default pin PC7).

## Connection Details

I2C bus: K_I2C1
  - SDA - PB11
  - SCL - PB10

SPI bus: K_SPI1
  - SDI - PA7
  - SDO - PA6
  - SCK - PA5
  - CS  - PA4

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-sensor-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash

Connect the sensors and then power the board.

Create a minicom session in order to connect to your board and see the project output

    $ minicom kubos

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/1.0.0/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/1.0.0/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/1.0.0/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/1.0.0/sdk-project-config.html)
- [STM32F4 Discovery Board Guide](http://docs.kubos.co/1.0.0/stm32f4-discovery-board-guide.html) 

