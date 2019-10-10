I Don't Have Hardware Yet, What Can I Do?
=========================================

Frequently mission projects are started with an idea of the intended mission architecture, but before
hardware has been received.
Hardware acquisition lead times can be quite long, in the realm of months, making it useful for
mission engineers to be able to begin developing their system's software before their hardware
arrives.

The Kubos SDK allows testing integration with :doc:`KubOS core services <../ecosystem/services/core-services>` and development of the high-level mission logic before
integrating with the final hardware system.

What Can Be Done
----------------

- Tutorials - The majority of the KubOS :doc:`tutorials <../tutorials/index>` can be completed
  without hardware. These tutorials will familiarize developers with basic system interactions and
  architecture
- Limited application development - High-level mission logic can be stubbed out and tested by running
  everything within a local development environment. Once hardware is acquired, the stubbed out
  code can be replaced with proper system calls.
- Core service testing - All :doc:`core service <../ecosystem/services/core-services>` can be run
  within a :doc:`local development environment <local-services>`, allowing service interactions to
  be developed and tested

What Can't Be Done
------------------

- Full simulation of hardware interactions - Applications can be developed with functions which stub
  out hardware interactions which would occur, however the Kubos SDK does not currently support
  proper hardware simulation
- Interact with hardware in the same manner as the main OS - If you have peripheral hardware available
  before the main OBC, you may be able to communicate with it via USB, however the device bus name will
  be different than when the hardware is connected in its final configuration (ex. ``/dev/ttyUSB0`` vs
  ``/dev/ttyS1``)
- Interact with logging in the same manner as the main OS - KubOS :doc:`breaks out logging <../ecosystem/linux-docs/logging>`
  into multiple files based on the severity level of the log message (debug, info, error) and the
  entity generating the message (app vs service). The Kubos SDK will route all log messages to
  ``/var/log/syslog`` instead
- Automatically start services - KubOS services are normally started automatically at boot time by
  init scripts. These scripts do not exist within the SDK, so the services will need to be
  :doc:`started manually <local-services>` for local testing
- Update and recovery testing - The KubOS :doc:`upgrade <../ecosystem/linux-docs/kubos-linux-upgrade>`
  and :doc:`recovery <../ecosystem/linux-docs/kubos-linux-recovery>` processes rely on the bootloader
  and specific partition configurations, so cannot be tested within a local environment
