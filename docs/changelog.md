# KubOS Changelog

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
* [Upgrade Instructions](docs/kubos-sdk.md)
* [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.5/docker-manifest.xml)

## v0.0.4 - Jul 01 2016
* Added a new I2C HAL API for both STM32F4 and MSP430 based targets (master mode only, slave coming soon)
* Simplified hardware debugging with GDB using the new `kubos debug` and `kubos server` commands
* A new [I2C example application](https://github.com/openkosmosorg/kubos-i2c-example)
  with a [WIP driver for the H2U1D temperature sensor](https://github.com/rplauche/kubos-core/blob/1ca0d601e33ea0e0c85caa9d53b7f84a78d9c24a/source/modules/sensors/htu21d.c)
* [Upgrade Instructions](docs/kubos-sdk.md)
* [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.4/docker-manifest.xml)

## v0.0.3 - Jun 17 2016
 * Added support for NanoAvionics SatBus 3C0 OBC
 * Implemented link support in KubOS-SDK for local development modules
 * New KubOS-SDK projects are now based off our latest kubos-rt-example source
 * [Upgrade Instructions](docs/kubos-sdk.md)
 * [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.3/docker-manifest.xml)

## v0.0.2 - Jun 03 2016
 * Added support for STM32F405RG based PyBoard
 * Improved support for yotta commands in KubOS-SDK
 * Improved error handling in KubOS-SDK
 * [Upgrade Instructions](docs/kubos-sdk.md)
 * [Tagged repo manifest](https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.2/docker-manifest.xml)
