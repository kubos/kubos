# KubOS HAL
## {#kubos-hal-main}

KubOS-HAL is the primary hardware abstraction layer (HAL) for KubOS. One of our goals is to simplify development when it comes to interfacing with your MCU(s). This module provides a unified set of functions for interfacing with hardware which will work the same across all of our supported MCUs.

### API Documentation:

 - @subpage GPIO
 - [I2C](docs/i2c.md)
 - @subpage SDIO
 - [SPI](docs/spi.md)
 - [UART](docs/uart.md)

### Hardware Interfaces:

 - [KubOS Linux Devices](@ref kubos-hal-linux-main)
 - [STM32F4](@ref kubos-hal-stm32f4-main)
 - [MSP430F5529](@ref kubos-hal-msp430f5529-main)
