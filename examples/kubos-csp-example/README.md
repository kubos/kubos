# Kubos CSP Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/kubos/tree/master/kubos-rt) demonstrating CSP over the Kubos HAL's UART interface.

## Project Overview

This application shows an example CSP (Cubesat Space Protocol) interaction between client and server tasks.

Three threads are created:
  - A CSP server
  - A CSP client
  - A button poll

The button poll thread continually polls the board’s button to see if it has been pressed. If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. If found, it connects to the CSP server’s port and sends a message “Hello World”.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.

**Note**: Due to a current peculiarity with the debouncing logic, the button must be pressed twice in order for the 'send message' event to occur.

## Connection Details

The CSP connection configuration is set using the *config.json* file included in this directory.

UART Bus: K_UART1

STM32F407 Discovery:
  - TX - PA9
  - RX - PA10

MSP430F5529 Launchpad
  - TX - P3.3
  - RX - P3.4
  
## Project Requirements

To successfully build and run this project, two boards must be used. This project will be flashed onto each.
One of the boards will need to be flashed after changing the *config.json* file to reverse the "my_address" and "target_address" 
values.

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-csp-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your first board (let's assume it's an STM32F407 Discovery board)

    $ kubos target stm32f407-disco-gcc
    
Build the project

    $ kubos build
    
Connect the first board to your computer. The USB connection should be automatically passed through to your VM.

Flash the project onto the board

    $ kubos flash
    
Now we'll set up the second board. Edit the *config.json* file in this directory and reverse the "my_address" and "target_address" values.

It should end up looking like this:

    {
        "CSP": {
            "my_address": "2",
            "target_address": "1",
            "port": "10",
            "uart_bus": "K_UART1",
            "uart_baudrate": "115200",
            "usart": {            
                }
        }
    }
    
(optional) If your second board is a different type, you'll need to set the new target

    $ kubos target msp430f5529-gcc

Re-build the project

    $ kubos build

Disconnect board 1 from your computer and then connect board 2.

Flash the project onto the second board.    

    $ kubos flash
    
Connect TX <--> RX UART pins between the boards.

For example, if you're using two STM32F407 Discovery boards:

  - Connect PA9 on board 1 to PA10 on board 2
  - Connect PA10 on board 1 to PA9 on board 2 

Power both of the boards. Push one of the board's buttons twice (on the STM32F4, this is the black button; on the MSP430, this is P2.1).
The red light on that board should blink, followed by the green light on the other board blinking.

## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/1.0.0/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/1.0.0/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/1.0.0/sdk-project-config.html)
- [STM32F4 Discovery Board Guide](http://docs.kubos.co/1.0.0/stm32f4-discovery-board-guide.html) 
- [MSP430 Launchpad Guide](http://docs.kubos.co/1.0.0/msp430-launchpad-guide.html) 

    
    
