# KubOS RT Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/kubos/tree/master/kubos-rt) demonstrating some basic functionality from KubOS RT (CSP, UART, FreeRTOS). 

This is the default application included when the `kubos init` command is executed. It is intended to provide an overview of several components as they might run in a KubOS RT application.

## Project Overview

Four threads are created:
  - A CSP server
  - A CSP client
  - A button poll
  - An interval print

The interval thread prints out “echo, x={n}” to the default UART port, where {n} is a basic counter.

The button poll thread continually polls the board’s button to see if it has been pressed. If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. If found, it connects to the CSP server’s port and sends a message “Hello World”.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.
  
## Project Requirements

To successfully run this project, you must have one of the supported KubOS RT target boards.

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-rt-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board (let's assume it's an STM32F407 Discovery board)

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash

Power the board. Push the board's button twice (on the STM32F4, this is the black button; on the MSP430, this is P2.1).
The red light on that board should blink to indicate a message has been sent, followed by the green light blinking to
indicate a message has been received.

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/1.0.0/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/1.0.0/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/1.0.0/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/1.0.0/sdk-project-config.html)
- [Kubos HAL I2C Guide](http://docs.kubos.co/1.0.0/kubos-hal/i2c.html)
- [STM32F4 Discovery Board Guide](http://docs.kubos.co/1.0.0/stm32f4-discovery-board-guide.html) 
- [MSP430 Launchpad Guide](http://docs.kubos.co/1.0.0/msp430-launchpad-guide.html) 
