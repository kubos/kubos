# Kubos Changelog

## v0.2.2 - April 7 2017
* [KUBOS-350](https://kubostech.atlassian.net/browse/KUBOS-350) Added multi-process communication support to telemetry library
* [KUBOS-283](https://kubostech.atlassian.net/browse/KUBOS-283) Created a background telemetry management service
* [KUBOS-391](https://kubostech.atlassian.net/browse/KUBOS-391) Created a background C&C service
* [KUBOS-409](https://kubostech.atlassian.net/browse/KUBOS-409) Added logging to C&C
* [KUBOS-376](https://kubostech.atlassian.net/browse/KUBOS-376) Added C&C 'build info' command
* [KUBOS-372](https://kubostech.atlassian.net/browse/KUBOS-372) Added C&C 'reboot' command
* [KUBOS-338](https://kubostech.atlassian.net/browse/KUBOS-338) Added KubOS Linux kernel rollback and recovery
* [KUBOS-340](https://kubostech.atlassian.net/browse/KUBOS-340) Added init script generation and flashing for KubOS Linux projects

## v0.2.1 - Mar 7 2017
* [KUBOS-388](https://kubostech.atlassian.net/browse/KUBOS-388) Created initial Command & Control framework
* [KUBOS-350](https://kubostech.atlassian.net/browse/KUBOS-350) Added support for inter-process communication between Kubos services
* [KUBOS-313](https://kubostech.atlassian.net/browse/KUBOS-313) Updated Kubos SDK to allow flashing of non-application files to iOBC
* [KUBOS-321](https://kubostech.atlassian.net/browse/KUBOS-321) Added ability to flash and install new KubOS Linux upgrade packages 
* [KUBOS-360](https://kubostech.atlassian.net/browse/KUBOS-360) Added Kubos CLI integration testing
* [KUBOS-363](https://kubostech.atlassian.net/browse/KUBOS-363) Fixed Kubos CLI error reporting

## v0.2 - Feb 3 2017
* Created KubOS Linux
* Major documentation updates
* [KUBOS-170](https://kubostech.atlassian.net/browse/KUBOS-170) Created Kubos CLI as primary Kubos SDK command tool
* [KUBOS-175](https://kubostech.atlassian.net/browse/KUBOS-175) Migrated from Docker to Vagrant for the Kubos SDK distribution system
* [KUBOS-329](https://kubostech.atlassian.net/browse/KUBOS-329) Added KubOS Linux support to Kubos SDK
* [KUBOS-361](https://kubostech.atlassian.net/browse/KUBOS-361) Added ability to use branched versions of Kubos code to Kubos CLI
* [KUOBS-267](https://kubostech.atlassian.net/browse/KUBOS-267) Added telemetry service libraries for both KubOS RT and KubOS Linux
* [KUBOS-213](https://kubostech.atlassian.net/browse/KUBOS-213) Added telemetry aggregation service libraries
* [KUBOS-201](https://kubostech.atlassian.net/browse/KUBOS-201) Added inter-subsystem communication libraries

## v0.1.4 - Oct 14 2016
* [KUBOS-81](https://kubostech.atlassian.net/browse/KUBOS-81) Implemented FatFs SPI SD support (Current MSP430 only)
* A new [example using the FatFs SPI SD interface](https://github.com/kubostech/kubos-sd-example)

## v0.1.3 - Sep 16 2016
* [KUBOS-132](https://kubostech.atlassian.net/browse/KUBOS-132) Standardized status codes for I2C, SPI and UART HAL
* [KUBOS-131](https://kubostech.atlassian.net/browse/KUBOS-131) Added basic unit testing for Kubos-HAL-MSP430F5529 and updated MSP430 documentation.
* [KUBOS-62](https://kubostech.atlassian.net/browse/KUBOS-62) Added preliminary OSX analytics tracking

## v0.1.1 - Sep 2 2016
* Documentation updates
* Added basic unit testing for Kubos-HAL, Kubos-HAL-STM32F4 and Kubos-SDK
* Miscellaneous bug fixes

## v0.1.0 - Aug 19 2016
* Implemented [SPI](./kubos-hal/group__SPI.html) for the STM32F4.
* Added a new sensor interface:
  * [IMU](./kubos-core/group__IMU.html)
  * [Altimeter](./kubos-core/group__ALTIMETER.html)
  * [Temperature](./kubos-core/group__TEMPERATURE.html)
* A new [sensor example application](https://github.com/kubostech/kubos-sensor-example)
* A new [example using CSP over uart](https://github.com/kubostech/kubos-csp-example)
* Added the ability to link in local targets with KubOS-SDK

## v0.0.5 - Aug 05 2016
* Added a [SPI HAL API](./kubos-hal/group__SPI.html)
  for MSP430 based targets (STM32F4 compatibility coming soon)
* Added a [SDIO HAL API](./kubos-hal/group__SDIO.html)
  for STM32F4 based targets
* Added filesystem support for FatFs based SD Cards, using SDIO (STM32F4 only)
* Added drivers for three sensors:
  * [HTU21D Temperature/Humidity](./kubos-core/group__HTU21D.html)
  * [BNO055 IMU](./kubos-core/group__BNO055.html)
  * [BME280 Temperature/Humidity/Pressure](./kubos-core/group__BME280.html)
* Added support for CSP over the Kubos-HAL UART interface
* A new [SPI example application](https://github.com/openkosmosorg/kubos-i2c-example)
  using the [driver for the BME280 sensor](./kubos-core/group__BME280.html)
* [Upgrade Instructions](docs/sdk-upgrading.md)
* [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.5/docker-manifest.xml)

## v0.0.4 - Jul 01 2016
* Added a new I2C HAL API for both STM32F4 and MSP430 based targets (master mode only, slave coming soon)
* Simplified hardware debugging with GDB using the new `kubos debug` and `kubos server` commands
* A new [I2C example application](https://github.com/openkosmosorg/kubos-i2c-example)
  with a [WIP driver for the H2U1D temperature sensor](https://github.com/rplauche/kubos-core/blob/1ca0d601e33ea0e0c85caa9d53b7f84a78d9c24a/source/modules/sensors/htu21d.c)
* [Upgrade Instructions](docs/sdk-upgrading.md)
* [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.4/docker-manifest.xml)

## v0.0.3 - Jun 17 2016
 * Added support for NanoAvionics SatBus 3C0 OBC
 * Implemented link support in KubOS-SDK for local development modules
 * New KubOS-SDK projects are now based off our latest kubos-rt-example source
 * [Upgrade Instructions](docs/sdk-upgrading.md)
 * [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.3/docker-manifest.xml)

## v0.0.2 - Jun 03 2016
 * Added support for STM32F405RG based PyBoard
 * Improved support for yotta commands in KubOS-SDK
 * Improved error handling in KubOS-SDK
 * [Upgrade Instructions](docs/sdk-upgrading.md)
 * [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.2/docker-manifest.xml)
