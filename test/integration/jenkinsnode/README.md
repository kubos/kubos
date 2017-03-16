# Kubos Continuous Integration

This repository contains files used in the continuous integration testing for Kubos software. The code in this repository assumes the use of certain custom hardware, and thus may not work without similar hardware available to the end user. Kubos uses a core server to compile software, and less powerful single board computers to flash the software and record/report testing data. 

At present, the code supports reflashing of the STM32F4 Discovery board, the Micropython (STM32F405) board, and the MSP430F5529 Launchpad. All of these have USB interfaces and an intermediate MCU that handles the raw commands and uploading. Forthcoming: we will be able to flash either using a programmer interface, a raw interface from a single computer (say, UART or CAN bus), or an alternate MCU interface.
