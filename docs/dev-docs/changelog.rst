Kubos Changelog
===============

v1.4.0 - July 23 2018
---------------------

- `Added UDP/GraphQL service for ISIS Antenna Systems <https://github.com/kubos/kubos/tree/master/services/isis-ants-service>`__
- `Updated Pumpkin MCU service to be compliant with the latest ICD <https://github.com/kubos/kubos/tree/master/services/pumpkin-mcu-service>`__
- `Added initial version of the mission applications service <https://github.com/kubos/kubos/tree/master/services/app-service>`__
- `Added initial mission applications Rust API <https://github.com/kubos/kubos/tree/master/apis/app-api>`__
- `Added initial mission applications Python API <https://github.com/kubos/kubos/tree/master/apis/python-app-api>`__
- `Added system Rust API for system-wide common functionality <https://github.com/kubos/kubos/tree/master/apis/system-api>`__
- `Upgraded CircleCI automation config to use the 2.0 configuration format <https://github.com/kubos/kubos/blob/master/.circleci/config.yml>`__
- Updated docs to reflect changes in how auxiliary SD images are generated

v1.3.0 - Jun 21 2018
--------------------

- `Added communication core service <https://github.com/kubos/kubos/tree/master/services/communication-service>`__
- `Added file transfer core service <https://github.com/kubos/kubos/tree/master/services/file-service>`__
- `Added shell core service <https://github.com/kubos/kubos/tree/master/services/shell-service>`__
- `Added telemetry database core service <https://github.com/kubos/kubos/tree/master/services/telemetry-service>`__
- `Added Rust API and UDP/GraphQL service for NovAtel OEM6 High Precision GNSS Receiver <https://github.com/kubos/kubos/blob/master/services/novatel-oem6-service>`__
- `Added Python API and UDP/GraphQL service for Pumpkin MCUs <https://github.com/kubos/kubos/tree/master/services/pumpkin-mcu-service>`__
- `Added Rust API for ClydeSpace 3G EPS <https://github.com/kubos/kubos/tree/master/apis/clyde-3g-eps-api>`__
- `Added Rust API and UDP/GraphQL service for Adcole Maryland Aerospace MAI-400 ADACS <https://github.com/kubos/kubos/tree/master/services/mai400-service>`__
- `Added API for GOMspace NanoPower P31U <https://github.com/kubos/kubos/blob/master/apis/gomspace-p31u-api>`__
- `Added C and Rust APIs for ISIS Antenna Systems <https://github.com/kubos/kubos/tree/master/apis/isis-ants-api>`__
- `Added C and Rust APIs for ISIS iMTQ <https://github.com/kubos/kubos/tree/master/apis/isis-imtq-api>`__
- `Added C API for ISIS TRXVU radio <https://github.com/kubos/kubos/tree/master/apis/isis-trxvu-api>`__
- `Added Python library for use when creating Kubos services <https://github.com/kubos/kubos/blob/master/libs/kubos-service>`__
- `Added Rust helper crate for use when creating Kubos services <https://github.com/kubos/kubos/tree/master/services/kubos-service>`__
- `Added Python library for I2C HAL <https://github.com/kubos/kubos/tree/master/hal/python-hal/i2c>`__
- `Added Rust crate for I2C HAL <https://github.com/kubos/kubos/tree/master/hal/rust-hal/rust-i2c>`__
- Upgraded Kubos SDK Vagrant and Docker images to use `Rust 1.26 <https://blog.rust-lang.org/2018/05/10/Rust-1.26.html>`__
- `Added link to nightly version of docs to main docs page <http://docs.kubos.co/master>`__
- `Added documentation for using ethernet as primary board-communication method <http://docs.kubos.co/latest/os-docs/using-kubos-linux.html#ethernet>`__
- Improved documentation about using Python and Rust for development with KubOS

Bug Fixes:
~~~~~~~~~~

- `Forcibly downgrading pip to <v10 to prevent incompatibility issue with yotta <https://github.com/kubos/kubos/blob/master/tools/dist/Dockerfile>`__

Community Contributions:
~~~~~~~~~~~~~~~~~~~~~~~~

- `Added Rust crate for UART HAL <https://github.com/kubos/kubos/tree/master/hal/rust-hal/rust-uart>`__


v1.2.0 - Mar 5 2018
-------------------

- `Added Rust crate for wrapping isis-iobc-supervisor <https://github.com/kubos/kubos/tree/c7bb5f1928aeb0aa3d45d649f90bd2cdccbe2bc5/hal/isis-iobc-supervisor>`__
- `Added iOBC supervisor GraphQL service <https://github.com/kubos/kubos/tree/master/services/iobc-supervisor-service>`__
- Migrating ``cargo-kubos`` into `own repo <https://github.com/kubos/cargo-kubos>`__
- `Added iOBC ADC support and demo <http://docs.kubos.co/1.2.0/os-docs/working-with-the-iobc.html#adc>`__
- `Added iOBC PWM support <http://docs.kubos.co/1.2.0/os-docs/working-with-the-iobc.html#adc>`__
- `Added API for EyeStar-D2 Duplex radio <https://github.com/kubos/kubos/tree/master/apis/nsl-duplex-d2>`__
- `Adding telemetry database service <https://github.com/kubos/kubos/blob/master/services/telemetry-database-service>`__
- Cleaning up doc generation warnings
- Finalizing name changes

v1.1.0 - Jan 24 2018
--------------------

- `Added iOBC UART support and demo <http://docs.kubos.co/1.1.0/os-docs/working-with-the-iobc.html#uart>`__
- `Added iOBC SPI support <http://docs.kubos.co/1.1.0/os-docs/working-with-the-iobc.html#spi>`__

- `Added Ethernet support for MBM2/BBB <http://docs.kubos.co/1.1.0/os-docs/working-with-the-bbb.html#ethernet>`__

- `Added generic radio API <http://docs.kubos.co/1.1.0/apis/device-api/radio.html>`__
- `Added generic ADCS API <http://docs.kubos.co/1.1.0/apis/device-api/adcs.html>`__

- `Added an I2C HAL for Linux <http://docs.kubos.co/1.1.0/apis/kubos-hal/i2c.html>`__

- Decided on using GraphQL, Rust, and Python for services and applications
- `Added Python-based example subsystem handler <https://github.com/kubos/kubos/tree/1.1.0/examples/python-service>`__
- `Added Rust-based example subsystem handler <https://github.com/kubos/kubos/tree/1.1.0/examples/rust-service>`__
- `Added 'cargo kubos' subcommand for Cargo-->yotta integration <https://github.com/kubos/kubos/tree/1.1.0/cargo-kubos>`__

- Upgraded to BuildRoot LTS 2017.2.8
- `Documented Windows PowerShell v3+ requirement <http://docs.kubos.co/1.1.0/installation-docs/sdk-installing.html#install-windows-powershell-v3-windows-7-sp1-only>`__

- `Updated architecture documentation <http://docs.kubos.co/1.1.0/architecture-overview.html>`__
- Refactored the repo to remove deprecated code
- Re-organized the docs to have a more nested structure
- `Updated naming conventions and coding standards <http://docs.kubos.co/1.1.0/dev-docs/kubos-standards.html>`__

v1.0.1 - Aug 4 2017
-------------------

- Adding support for Beaglebone Black
- Adding support for Pumpkin MBM2
- Adding Windows Dev Environment guide

v1.0.0 - June 27 2017
---------------------

- `KUBOS-442 <https://kubostech.atlassian.net/browse/KUBOS-442>`__
  Added support for iOBC I2C bus
- `KUBOS-445 <https://kubostech.atlassian.net/browse/KUBOS-445>`__
  Integrated iOBC supervisor
- `KUBOS-274 <https://kubostech.atlassian.net/browse/KUBOS-274>`__
  Completed Kubos Telemetry integration into KubOS Linux
- `KUBOS-487 <https://kubostech.atlassian.net/browse/KUBOS-487>`__
  Added support for tab-completion to Kubos CLI
- `Created an initial QA integration test suite <https://github.com/kubos/kubos/tree/master/test/integration/linux>`__
-  Migrated from Markdown to ReStructuredText as the documentation
   language of choice
-  Migrated from only Doxygen to Sphinx and Doxygen as the documentation
   generation tools of choice
-  Massively improved documentation basically everywhere
-  Polished everything to a shiny gleam

v0.2.2 - April 7 2017
---------------------

-  `KUBOS-350 <https://kubostech.atlassian.net/browse/KUBOS-350>`__
   Added multi-process communication support to telemetry library
-  `KUBOS-283 <https://kubostech.atlassian.net/browse/KUBOS-283>`__
   Created a background telemetry management service
-  `KUBOS-391 <https://kubostech.atlassian.net/browse/KUBOS-391>`__
   Created a background C&C service
-  `KUBOS-409 <https://kubostech.atlassian.net/browse/KUBOS-409>`__
   Added logging to C&C
-  `KUBOS-376 <https://kubostech.atlassian.net/browse/KUBOS-376>`__
   Added C&C 'build info' command
-  `KUBOS-372 <https://kubostech.atlassian.net/browse/KUBOS-372>`__
   Added C&C 'reboot' command
-  `KUBOS-338 <https://kubostech.atlassian.net/browse/KUBOS-338>`__
   Added KubOS Linux kernel rollback and recovery
-  `KUBOS-340 <https://kubostech.atlassian.net/browse/KUBOS-340>`__
   Added init script generation and flashing for KubOS Linux projects

v0.2.1 - Mar 7 2017
-------------------

-  `KUBOS-388 <https://kubostech.atlassian.net/browse/KUBOS-388>`__
   Created initial Command & Control framework
-  `KUBOS-350 <https://kubostech.atlassian.net/browse/KUBOS-350>`__
   Added support for inter-process communication between Kubos services
-  `KUBOS-313 <https://kubostech.atlassian.net/browse/KUBOS-313>`__
   Updated Kubos SDK to allow flashing of non-application files to iOBC
-  `KUBOS-321 <https://kubostech.atlassian.net/browse/KUBOS-321>`__
   Added ability to flash and install new KubOS Linux upgrade packages
-  `KUBOS-360 <https://kubostech.atlassian.net/browse/KUBOS-360>`__
   Added Kubos CLI integration testing
-  `KUBOS-363 <https://kubostech.atlassian.net/browse/KUBOS-363>`__
   Fixed Kubos CLI error reporting

v0.2 - Feb 3 2017
-----------------

-  Created KubOS Linux
-  Major documentation updates
-  `KUBOS-170 <https://kubostech.atlassian.net/browse/KUBOS-170>`__
   Created Kubos CLI as primary Kubos SDK command tool
-  `KUBOS-175 <https://kubostech.atlassian.net/browse/KUBOS-175>`__
   Migrated from Docker to Vagrant for the Kubos SDK distribution system
-  `KUBOS-329 <https://kubostech.atlassian.net/browse/KUBOS-329>`__
   Added KubOS Linux support to Kubos SDK
-  `KUBOS-361 <https://kubostech.atlassian.net/browse/KUBOS-361>`__
   Added ability to use branched versions of Kubos code to Kubos CLI
-  `KUOBS-267 <https://kubostech.atlassian.net/browse/KUBOS-267>`__
   Added telemetry service libraries for both KubOS RT and KubOS Linux
-  `KUBOS-213 <https://kubostech.atlassian.net/browse/KUBOS-213>`__
   Added telemetry aggregation service libraries
-  `KUBOS-201 <https://kubostech.atlassian.net/browse/KUBOS-201>`__
   Added inter-subsystem communication libraries

v0.1.4 - Oct 14 2016
--------------------

-  `KUBOS-81 <https://kubostech.atlassian.net/browse/KUBOS-81>`__
   Implemented FatFs SPI SD support (Current MSP430 only)
-  A new `example using the FatFs SPI SD
   interface <https://github.com/kubos/kubos-sd-example>`__

v0.1.3 - Sep 16 2016
--------------------

-  `KUBOS-132 <https://kubostech.atlassian.net/browse/KUBOS-132>`__
   Standardized status codes for I2C, SPI and UART HAL
-  `KUBOS-131 <https://kubostech.atlassian.net/browse/KUBOS-131>`__
   Added basic unit testing for Kubos-HAL-MSP430F5529 and updated MSP430
   documentation.
-  `KUBOS-62 <https://kubostech.atlassian.net/browse/KUBOS-62>`__ Added
   preliminary OSX analytics tracking

v0.1.1 - Sep 2 2016
-------------------

-  Documentation updates
-  Added basic unit testing for Kubos-HAL, Kubos-HAL-STM32F4 and
   Kubos-SDK
-  Miscellaneous bug fixes

v0.1.0 - Aug 19 2016
--------------------

-  Implemented `SPI <./kubos-hal/group__SPI.html>`__ for the STM32F4.
-  Added a new sensor interface:
-  `IMU <./kubos-core/group__IMU.html>`__
-  `Altimeter <./kubos-core/group__ALTIMETER.html>`__
-  `Temperature <./kubos-core/group__TEMPERATURE.html>`__
-  A new `sensor example
   application <https://github.com/kubos/kubos-sensor-example>`__
-  A new `example using CSP over
   uart <https://github.com/kubos/kubos-csp-example>`__
-  Added the ability to link in local targets with KubOS-SDK

v0.0.5 - Aug 05 2016
--------------------

-  Added a `SPI HAL API <./kubos-hal/group__SPI.html>`__ for MSP430
   based targets (STM32F4 compatibility coming soon)
-  Added a `SDIO HAL API <./kubos-hal/group__SDIO.html>`__ for STM32F4
   based targets
-  Added filesystem support for FatFs based SD Cards, using SDIO
   (STM32F4 only)
-  Added drivers for three sensors:
-  `HTU21D Temperature/Humidity <./kubos-core/group__HTU21D.html>`__
-  `BNO055 IMU <./kubos-core/group__BNO055.html>`__
-  `BME280
   Temperature/Humidity/Pressure <./kubos-core/group__BME280.html>`__
-  Added support for CSP over the Kubos-HAL UART interface
-  A new `SPI example
   application <https://github.com/openkosmosorg/kubos-i2c-example>`__
   using the `driver for the BME280
   sensor <./kubos-core/group__BME280.html>`__
-  `Upgrade Instructions <sdk-upgrading>`__
-  `Tagged repo
   manifest <https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.5/docker-manifest.xml>`__

v0.0.4 - Jul 01 2016
--------------------

-  Added a new I2C HAL API for both STM32F4 and MSP430 based targets
   (master mode only, slave coming soon)
-  Simplified hardware debugging with GDB using the new ``kubos debug``
   and ``kubos server`` commands
-  A new `I2C example
   application <https://github.com/openkosmosorg/kubos-i2c-example>`__
   with a `WIP driver for the H2U1D temperature
   sensor <https://github.com/rplauche/kubos-core/blob/1ca0d601e33ea0e0c85caa9d53b7f84a78d9c24a/source/modules/sensors/htu21d.c>`__
-  `Upgrade Instructions <sdk-upgrading>`__
-  `Tagged repo
   manifest <https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.4/docker-manifest.xml>`__

v0.0.3 - Jun 17 2016
--------------------

-  Added support for NanoAvionics SatBus 3C0 OBC
-  Implemented link support in KubOS-SDK for local development modules
-  New KubOS-SDK projects are now based off our latest kubos-rt-example
   source
-  `Upgrade Instructions <sdk-upgrading>`__
-  `Tagged repo
   manifest <https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.3/docker-manifest.xml>`__

v0.0.2 - Jun 03 2016
--------------------

-  Added support for STM32F405RG based PyBoard
-  Improved support for yotta commands in KubOS-SDK
-  Improved error handling in KubOS-SDK
-  `Upgrade Instructions <sdk-upgrading>`__
-  `Tagged repo
   manifest <https://github.com/openkosmosorg/kubos-manifest/blob/v0.0.2/docker-manifest.xml>`__
