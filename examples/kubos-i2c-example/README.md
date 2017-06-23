# Kubos I2C Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/kubos/tree/master/kubos-rt) demonstrating our I2C HAL.

## Project Overview

This application gives several examples of how to interact with I2C devices in a Kubos project:

If the HTU21D sensor has been defined in the project’s config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.
  
If the BNO055 sensor has been defined in the project’s config.json file, the sensor will be initialized in NDOF (Nine Degrees Of Freedom) mode and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current position data from the sensor to the default UART port.
  
If no sensor has been defined in the project’s config.json file, then this application will initialize a generic I2C connection over I2C bus 1 to a slave device with an address of ‘0x40’.
  - It will then write a single byte command of ‘0xE3’ to the slave and attempt to read back a three byte response.
  - After this attempt, the application will end.

## Connection Details

I2C bus: K_I2C1

STM32F407 Discovery:
  SDA - PB7
  SCL - PB6

MSP430F5529 Launchpad
  SDA - P3.0
  SCL - P3.1
  
## Project Requirements

To successfully run this project as-is, you must connect at least one of the following:

  - An HTU21D sensor
  - A BNO055 sensor

To use a different I2C device, you should edit the *config.json* file in this directory and remove the "sensors" section.
The device should be capable of operating in 7-bit addressing mode at 10kHz.

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-i2c-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board (let's assume it's an STM32F407 Discovery board)

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash

Connect the desired I2C sensor and then power the board.

See the [STM32F4 discovery board Guide](docs.kubos.co/stm32f4-discovery-board-guide.html) or [MSP430 launchpad guide](docs.kubos.co/msp430-launchpad-guide.html) for information on how to connect to the debug console and see the output from this project. 

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](docs.kubos.co)
- [Installing the Kubos SDK](docs.kubos.co/sdk-installing.html)
- [Kubos SDK Cheatsheet](docs.kubos.co/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](docs.kubos.co/sdk-reference.html) 
- [Kubos Project Configuration](docs.kubos.co/sdk-project-config.html)
- [Kubos HAL I2C Guide](http://docs.kubos.co/kubos-hal/i2c.html)
- [STM32F4 Discovery Board Guide](docs.kubos.co/stm32f4-discovery-board-guide.html) 
- [MSP430 Launchpad Guide](docs.kubos.co/msp430-launchpad-guide.html) 
