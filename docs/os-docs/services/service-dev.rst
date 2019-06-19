Developing a KubOS Service
==========================

This document assumes you already have a good base understanding of the KubOS ecosystem.

If you are unfamiliar with the system, we recommend first going through our
:doc:`new user tutorials <../../tutorials/index>`.

Resources
---------

- :doc:`graphql`
- :doc:`service-outline-guide`
- :doc:`service-config`
- :doc:`../../contributing/testing`
- :doc:`../../contributing/documentation`

Example Services
----------------

Rust
~~~~

- `Clyde 3G 1U EPS Service <https://github.com/kubos/kubos/tree/master/services/clyde-3g-eps-service>`__
- `NSL EyeStar-D2 Duplex Radio Service <https://github.com/kubos/kubos/tree/master/services/nsl-duplex-d2-comms-service>`__

Python
~~~~~~

- `Pumpkin MCU Service <https://github.com/kubos/kubos/tree/master/services/pumpkin-mcu-service>`__

Recommended Libraries
---------------------

There is existing tooling which we recommended you use when bringing up a new service.

TODO

Python
~~~~~~

The `kubos-service <https://github.com/kubos/kubos/tree/master/libs/kubos-service>`__ library has
been created to abstract the process needed to create and start a GraphQL server over HTTP.
It is built on top of Flask and automatically creates the the GraphQL and GraphiQL endpoints

Rust
~~~~

Creating the Schema
-------------------

All services should implement the base service schema, as documented in the
:doc:`service outline <service-outline-guide>` doc.

- Queries allow users to fetch information about the state of the system and other telemetry items.
- Mutations are operations which may affect the state of the system.

We recommend implementing the most basic operations first (ping, no-op, reset) to establish the
initial service framework before moving on to the more complex (or unique) features.

In general, we don't bother to expose all possible functionality of a particular hardware device.
Instead, we focus on the most common functionality as well as the specific operations we know we'll
need.
This allows us to reduce the amount of development time required in order to create a new service.

As a result, we make sure to include a ``commandRaw`` mutation in all hardware services.
This allows the service to still be able to execute any functionality which wasn't explicitly
programmed.

Logging
-------

We recommend logging any errors which your service encounters.
These errors will likely also be returned in a GraphQL response, however we feel that this
redudancy is important for maintaining overall system health.

All log messages issued by the service should be routed to the system logs.
You may also choose to echo the messages to ``stdout``, however that is not a required behavior.

Services should use the daemon logging facility (rather than the user facility).
This will cause all service log messages to be routed to the `/var/log/kubos-*.log` files.

Please refer to the :doc:`logging <../linux-docs/logging>` doc for more information about the setup
and behavior of log messages.

Service Configuration
---------------------

It may be useful for your service to have certain configurable settings.
For instance, you might want to be able to adjust certain timeout values, or change which device
bus your hardware is connected to.
In this case, your service should read the needed configuration values from the system's
`config.toml` file.

More information about setting and fetching configuration values can be found in the
:doc:`service config <service-config>` doc.

Testing Your Service
--------------------

TODO

Unit tests
Integration tests...
End-to-end tests

Creating an Init Script
-----------------------

If you would like your service to be automatically started at system boot, you will need to create
an init script.

For Rust-based services, the `monitor service's <https://github.com/kubos/kubos-linux-build/blob/master/package/kubos/kubos-monitor/kubos-monitor>`__
init script provides a good example.

For Python-based services, please refer to the `Pumpkin MCU service <https://github.com/kubos/kubos-linux-build/blob/master/package/kubos/kubos-pumpkin-mcu/kubos-pumpkin-mcu>`__
init script for reference.

In order to be successfully picked up by the init system, the init script's name must use the
following format: ``S{run-level}{application-name}``. The run-level value should be between 1 and 99.
The lower the value, the earlier it will be run in the system boot process.

Installing Your Service
-----------------------

Custom services may either live in the user data partition or in the root file system.

User Data Partition
~~~~~~~~~~~~~~~~~~~

If your service will live in the user data partition, then there will be two steps needed to install
the service.

1. :ref:`Transfer <file-transfer>` the service binary to `/home/system/usr/bin`. This is the
   preferred location for user executables and is in the system PATH.
2. Transfer the service init script to `/home/system/etc/init.d`.

Root File System
~~~~~~~~~~~~~~~~

In order for your service to be installed into the root file system, you will need to create a
custom Buildroot package and then generate your own KubOS image.

More information about creating custom packages can be found in the :ref:`Configuring KubOS <custom-packages>`
doc.