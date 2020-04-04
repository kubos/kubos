Kubos Changelog
===============

v1.21.0 - Apr 2 2020
--------------------

- `Added new config options to help fine-tune file transfers <https://docs.kubos.com/1.21.0/ecosystem/services/file.html#configuration>`__
- `Added Kubos Linux system requirements description to docs <https://docs.kubos.com/1.21.0/obc-docs/porting-kubos.html#system-requirements>`__
- `Added basic instructions on how to cross-compile for targets which are not fully supported by Rust <https://docs.kubos.com/1.21.0/sdk-docs/sdk-advanced-cross-compiling.html>`__
- Improved first-time user instructions

Bug Fixes
~~~~~~~~~

- Fixed scheduler service's wait time after requesting an app be started
- Corrected Rust-based example mission app behavior when run locally

v1.20.0 - Nov 22 2019
---------------------

- `Added ability to register apps in a single archive file <https://docs.kubos.com/1.20.0/ecosystem/services/app-service.html#registering>`__
- Upgraded Rust usage to v1.39.0

Community Contributions
~~~~~~~~~~~~~~~~~~~~~~~

- Miscellaneous tutorial fixes

Bug Fixes
~~~~~~~~~

- `Added instructions for changing the KubOS source version when building Kubos Linux <https://docs.kubos.com/1.20.0/deep-dive/klb/configuring-kubos.html#kubos-version-selection>`__
- `Added instructions for manually upgrading the Rust cross-compile toolchains <https://docs.kubos.com/1.20.0/faq-troubleshooting.html#updating-my-local-rust-installation>`__

v1.19.0 - Oct 30 2019
---------------------

- `Added Scheduler Service <https://docs.kubos.com/1.19.0/ecosystem/scheduler.html>`__
    - **note**: The scheduler service is currently not included in Kubos Linux builds for the iOBC.
- Updated kubos-shell-client to allow single command execution
- Updated default ``config.toml`` location from ``/home/system/etc/config.toml`` to ``/etc/kubos-config.toml``
- `Migrated to new default service ports <https://docs.kubos.com/master/1.19.0/ecosystem/services/service-dev.html#service-configuration>`__
- Removed run levels from apps and the associated app arg
- Changed app service uninstall logic to nicely kill a running app first
- `Added No Hardware doc <https://docs.kubos.com/1.19.0/getting-started/no-board.html>`__
- Added ``--stdout`` flag to services for easy output on stdout
- Standardized the default service storage location in ``tools/local_config.toml`` for easier local development
- Miscellaneous doc improvements

Bug Fixes:
~~~~~~~~~~

- Fixed crashing file service & client with bad defaults
- Fixed communications issue in ISIS Antenna Service

v1.18.0 - Sept 12 2019
----------------------

- `Added initial app monitoring capabilities <https://docs.kubos.com/1.18.0/ecosystem/services/app-service.html#application-execution-status>`__
- `Added ability to stop currently running applications <https://docs.kubos.com/1.18.0/ecosystem/services/app-service.html#stopping-an-application>`__
- `Added ability to do bulk telemetry database inserts asychronously <https://docs.kubos.com/1.18.0/ecosystem/services/telemetry-db.html#adding-entries-to-the-database-asynchronously>`__
- Updated Docker and Vagrant configurations to use Python3.7
- Upgraded Rust usage to v1.37.0
- Miscellanous doc improvements

v1.17.0 - Aug 15 2019
---------------------

- Added `example OBC housekeeping mission app <https://github.com/kubos/kubos/tree/master/apps/obc-hs>`__
- Updated the file transfer service to allow the downlink destination to be configurable
- Changed service configuration behavior to fail when config values cannot be read, rather than
  taking default values
- Documented `UDP passthrough behavior <https://docs.kubos.com/1.17.0/ecosystem/services/comms-framework.html>`__ for the comms service framework
- Documented `arg passthrough behavior <https://docs.kubos.com/1.17.0/ecosystem/apps/app-guide.html#additional-arguments>`__ in the app development guide
- `Added tips for reducing Rust binary sizes <https://docs.kubos.com/master/1.17.0/getting-started/using-rust.html#making-rust-binaries-smaller>`__
- Tweaked how C-based libraries are included in our Rust workspace to improve the ability to use
  KubOS software in a local development environment
- Updated all Python packages to contain accurate `requirements.txt` files
- Updated all packages and libraries to contain Readme files
- Upgraded Rust usage to v1.36.0
- Upgraded base CI Docker image to Ubuntu 18.04
- Migrated all remaining tooling to Python3

Bug Fixes:
~~~~~~~~~~

- Miscellaneous fixes to support building and running KubOS from a MacOS development environment

v1.16.0 - Jul 10 2019
---------------------

- The Great Docs Re-Org of 2019

    - `Added instructions for setting up a development environment without using the SDK <https://docs.kubos.com/1.16.0/getting-started/local-setup.html>`__
    - `Added instructions for running KubOS core services within a local development environment <https://docs.kubos.com/1.16.0/getting-started/local-services.html>`__
    - Reworked tutorials for local execution
    - `Added a high-level KubOS porting guide <https://docs.kubos.com/1.16.0/obc-docs/porting-kubos.html>`__
    - `Added a more in-depth communications setup guide <https://docs.kubos.com/1.16.0/obc-docs/comms-setup.html>`__
    - `Added mission development guides <https://docs.kubos.com/1.16.0/mission-dev/index.html>`__
    - `Added a service development guide <https://docs.kubos.com/1.16.0/ecosystem/services/service-dev.html>`__
    - `Expanded community contribution guides <https://docs.kubos.com/1.16.0/contributing/index.html>`__
    - Re-organized most docs in order to create a more accessible and intuitive user experience

- `Updated the comms service framework to use SpacketPacket instead of UDP <https://docs.kubos.com/1.16.0/ecosystem/services/comms-framework.html#data-packets>`__
- Added UDP passthrough to the comms service framework for non-GraphQL messages
- `Added ability to perform bulk inserts with the telemetry database service <https://docs.kubos.com/1.16.0/ecosystem/services/telemetry-db.html#adding-multiple-entries-to-the-database>`__
- `Improved size optimization for Rust executables when doing release builds <https://github.com/kubos/kubos/blob/master/Cargo.toml>`__

Bug Fixes:
~~~~~~~~~~

- Fixed ability to pass a custom config file to Rust applications
- Fixed assorted timing issues with CI tests

v1.15.0 - May 23 2019
---------------------

- `Added a comms service implementation for the NSL EyeStar-D2 Duplex radio <https://docs.kubos.com/1.15.0/rust-docs/nsl_duplex_d2_comms_service/index.html>`__
- `Updated comms service framework to use Space Packet Protocol <https://docs.kubos.com/1.15.0/services/comms-framework.html#data-packets>`__
- Added 'ping' query to all Kubos services
- `Added ability to query multiple telemetry fields in one request <https://docs.kubos.com/1.15.0/services/telemetry-db.html#querying-the-service>`__
- Updated docs to reflect use of latest Buildroot LTS release, 2019.02.2
- `Migrated to new CLA site <https://cla-assistant.io/kubos/kubos>`__
- Expanded application unit tests
- Added inter-service integration tests

Bug Fixes:
~~~~~~~~~~

- Cleaning up orphaned files after app uninstall
- Changing current working directory to an app's registered directory before starting it

v1.14.0 - Apr 3 2019
--------------------

- `Added logic to the applications service to check if an app immediately fails after being started <https://docs.kubos.com/1.14.0/app-docs/app-service.html#starting-an-application>`__
- `Removed UUIDs from the applications service. Apps will now be referenced by name <https://docs.kubos.com/1.14.0/app-docs/app-service.html>`__
- `Added ability to uninstall all versions of an application <https://docs.kubos.com/1.14.0/app-docs/app-service.html#de-registering>`__
- Updated app registration process to clean up all files if registration fails
- `Added ability to manually set the active version of an application <https://docs.kubos.com/1.14.0/app-docs/app-service.html#changing-versions>`__
- `Added parameter to Rust app API to allow minimum logging level to be controlled <https://docs.kubos.com/1.14.0/rust-docs/kubos_app/macro.app_main.html#arguments>`__
- `Added parameter to Python app API to allow minimum logging level to be controlled <https://docs.kubos.com/1.14.0/app-docs/python-app-api.html#app_api.logging_setup>`__
- `Updated comms service framework to translate between UDP and HTTP <https://docs.kubos.com/1.14.0/services/comms-framework.html>`__
- Added comms service framework unit tests
- `Added documentation for service configuration <https://docs.kubos.com/1.14.0/services/service-config.html>`__
- `Updated the Pumpkin supMCU API to match v4.22 of the firmware manual <https://github.com/kubos/kubos/blob/master/apis/pumpkin-mcu-api/mcu_api.py>`__
- `Added GraphQL service for the Clyde Space 3rd generation EPS <https://docs.kubos.com/1.14.0/rust-docs/clyde_3g_eps_service/index.html>`__
- Improved logging for all Kubos services

Bug Fixes:
~~~~~~~~~~

- Updated the comms service framework to correctly validate that a write function is present
- Updated the apps service to remove the parent directory if the last version of an app is uninstalled
- Updated app registration process to ensure current active version remains active if new registration fails
- Fixed Rust app API to use project name for logging
- Migrated the Kubos contributor's agreement to https://cla-assistant.io/kubos/kubos
- `Corrected the file transfer client syntax documentation <https://docs.kubos.com/1.14.0/tutorials/file-transfer.html#syntax>`__

v1.13.0 - Feb 15 2019
---------------------

- `Updated all Kubos services to use HTTP as their GraphQL front-end instead of UDP, allowing them to expose GraphiQL endpoints <https://docs.kubos.com/1.13.0/tutorials/app-register.html#graphiql>`__
- Removed all yotta usage. `C-based projects are now compiled using CMake <https://docs.kubos.com/1.13.0/sdk-docs/sdk-c.html>`__
- Pinning Rust version at v1.32.0 to prevent accidental version discrepancies

v1.12.0 - Feb 1 2019
--------------------

- Updated all Python code to be compatible with Python3.5
- `Updated the Python App API doc to be more verbose <https://docs.kubos.com/1.11.0/app-docs/python-app-api.html>`__
- `Updated the C HAL to directly use Linux conventions <https://docs.kubos.com/1.11.0/apis/kubos-hal/i2c-hal/c-i2c/c-i2c.html>`__
- Removed CSP from the code base
- Removed remaining usage of yotta configuration constants

v1.11.0 - Jan 18 2019
---------------------

- `Added communications service framework overview doc <https://docs.kubos.com/1.11.0/services/comms-framework.html>`__
- `Added example communications service <https://github.com/kubos/kubos/tree/master/examples/serial-comms-service>`__
- `Added communications service tutorial <https://docs.kubos.com/1.11.0/tutorials/comms-service.html>`__
- Updated all Rust modules to use Rust 2018

Community Contributions:
~~~~~~~~~~~~~~~~~~~~~~~~

- `Added communications service framework <https://docs.kubos.com/1.11.0/services/comms-framework.html>`__

v1.10.0 - Dec 20th 2018
-----------------------

- `Added process monitoring to most Kubos services <https://docs.kubos.com/1.10.0/os-docs/monitoring.html>`__
- `Updated the default logging template to include message severity <https://github.com/kubos/kubos-linux-build/blob/master/common/overlay/etc/rsyslog.conf#L31>`__
- `Updated the logging tutorial and examples to use the app API's logging initialization <https://docs.kubos.com/master/1.9.1+3/tutorials/first-mission-app.html#adding-logging>`__
- `Updated the BBB and MBM2 installation instructions to use the new eMMC install script <https://docs.kubos.com/master/1.9.1+3/installation-docs/installing-linux-bbb.html#flash-the-emmc>`__

v1.9.0 - Dec 6th 2018
---------------------

- `Added support for logging using rsyslog <https://docs.kubos.com/1.9.0/tutorials/first-mission-app.html#adding-logging>`__
- Updated all Kubos services to use new logging
- `Updated file transfer service to clean up temporary storage directories after successful transfer and on-request <https://docs.kubos.com/1.9.0/apis/kubos-libs/file-protocol.html#cleanup-request>`__
- `Added SLIP support to all boards <https://docs.kubos.com/1.9.0/os-docs/using-kubos-linux.html#slip>`__
- Updated the applications service to allow more than two files to be present in the directory used
  to register an application

Bug Fixes:
~~~~~~~~~~

- File transfer client now returns error when it fails to communicate with the file service
- Updated all Kubos services' GraphQL responses to follow the official response spec

v1.8.0 - Nov 9th 2018
---------------------

- `The shell service and client have both been rewritten into Rust <https://github.com/kubos/kubos/tree/master/services/shell-service>`__
- `Updated the applications service's schema to match the styling of the other services <https://docs.kubos.com/1.8.0/app-docs/app-service.html>`__
- Added cleanup logic when the app service encounters a corrupted app entry
- `Added the ability to upgrade an application with the applications service <https://docs.kubos.com/1.8.0/app-docs/app-service.html#upgrading>`__
- `Updated the Rust app API to utilize exit codes <https://docs.kubos.com/1.8.0/rust-docs/kubos_app/index.html>`__
- `Added Pumpkin MBM2 RTC support <https://docs.kubos.com/master/1.7.1+14/os-docs/working-with-the-mbm2.html#rtc>`__

Bug Fixes:
~~~~~~~~~~

- Updating telemetry database API to use a double to store the timestamp, rather than a 32-bit integer

Community Contributions:
~~~~~~~~~~~~~~~~~~~~~~~~

- Updated link descriptions for Kubos Linux release files

v1.7.0 - Oct 12th 2018
----------------------

- `Added overview documentation for the monitor service <https://docs.kubos.com/1.7.0/services/monitor-service.html>`__
- `Added tutorials to help new users write their first mission application <https://docs.kubos.com/1.7.0/tutorials/index.html#mission-development-tutorials>`__
- `Added tutorials to help new users interact with the telemetry database and file transfer services <https://docs.kubos.com/1.7.0/tutorials/index.html#system-interaction-tutorials>`__

Bug Fixes:
~~~~~~~~~~

- Updated the Python app API to accept an empty string in the 'errors' field of GraphQL responses
- Updated the file transfer service to not exit if an invalid data packet is received
- Updated the file transfer service's log location

v1.6.0 - Sept 28th 2018
-----------------------

- Added corrupted chunk and file hash mismatch error handling to the file transfer service
- Improved file transfer service multi-client handling
- `Added timeout and chunk-size configuration options to the file transfer service <https://docs.kubos.com/1.6.0/services/file.html#configuration>`__
- `Updated the telemetry database service to allow query results to be written to an output file <https://docs.kubos.com/1.6.0/services/telemetry-db.html#saving-results-for-later-processing>`__
- `Added insert and delete capabilities to the telemetry database service <https://docs.kubos.com/1.6.0/services/telemetry-db.html#adding-entries-to-the-database>`__
- Increased telemetry database timestamp key resolution from seconds to milliseconds
- `Updated applications service to add ability to passthrough arguments to the application being called <https://docs.kubos.com/1.6.0/app-docs/app-guide.html#additional-arguments>`__
- `Added a basic system-resource monitoring service <https://github.com/kubos/kubos/tree/master/services/monitor-service>`__

Bug Fixes:
~~~~~~~~~~

- `Updated Rust cross-compiling instructions to handle new CC arg requirement <https://docs.kubos.com/1.6.0/sdk-docs/sdk-rust.html#cross-compilation>`__
- `Added doc links to the pre-built hardware services' documentation <https://docs.kubos.com/1.6.0/services/hardware-services.html#pre-built-services>`__

v1.5.0 - Sep 7th 2018
---------------------

- `Added a community Trello board for contributors and KubOS team members <https://trello.com/b/pIWxmFua/kubos-community>`__
- `The file transfer service and client have both been rewritten into Rust <https://github.com/kubos/kubos/tree/master/services/file-service>`__
- `Added documentation for how to handle the deployment hold time countdown <https://docs.kubos.com/1.5.0/app-docs/deployment.html>`__
- Updated the app APIs to commonize behaviour between the `Python <https://github.com/kubos/kubos/tree/master/apis/app-api/python>`__ and `Rust <https://github.com/kubos/kubos/tree/master/apis/app-api/rust>`__ versions
- Added example mission applications for both `Rust <https://github.com/kubos/kubos/tree/master/examples/rust-mission-app>`__ and `Python <https://github.com/kubos/kubos/tree/master/examples/python-mission-app>`__
- `Added a verbose example mission application in Python for mission design <https://github.com/kubos/kubos/tree/master/examples/python-mission-application>`__

Bug Fixes:
~~~~~~~~~~

- Standardized usage of the Rust failure crate for version 0.1.2

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
- `Added documentation for using ethernet as primary board-communication method <http://docs.kubos.co/1.3.0/os-docs/using-kubos-linux.html#ethernet>`__
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
