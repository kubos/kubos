Developing a KubOS Service
==========================

This document goes over the basic components of creating a new service for KubOS.

It assumes you already have a good base understanding of the KubOS ecosystem and is intended as more
of a reference document, rather than a detailed tutorial.

If you are unfamiliar with the system, we recommend first going through our
:doc:`new user tutorials <../../tutorials/index>`.

Resources
---------

- :doc:`graphql`
- :doc:`service-outline-guide`
- :doc:`Service Configuration <service-config>`
- :doc:`Testing Guide <../../contributing/testing>`
- :doc:`Documentation Guide <../../contributing/documentation>`

Example Services
----------------

The underlying framework tends to be quite common between services.
As a result, it will likely be useful to refer to an existing service when creating a new one.

The following are recommended example services:

Rust
~~~~

- `Clyde 3G 1U EPS Service <https://github.com/kubos/kubos/tree/master/services/clyde-3g-eps-service>`__
- `NSL EyeStar-D2 Duplex Radio Service <https://github.com/kubos/kubos/tree/master/services/nsl-duplex-d2-comms-service>`__

Python
~~~~~~

- `Pumpkin MCU Service <https://github.com/kubos/kubos/tree/master/services/pumpkin-mcu-service>`__

Recommended Libraries
---------------------

There is existing tooling which we use internally and recommend you use when bringing up a new
service.

Python
~~~~~~

- `kubos-service <https://github.com/kubos/kubos/tree/master/libs/kubos-service>`__  - Abstracts the
  process needed to create and start a GraphQL server over HTTP.
  It is built on top of `Flask <https://github.com/graphql-python/flask-graphql>`__ and
  automatically creates the the GraphQL and :ref:`GraphiQL <graphiql>` endpoints.
- `Graphene <https://graphene-python.org/>`__ - Library for constructing the GraphQL schema

Rust
~~~~

- `kubos-service <https://github.com/kubos/kubos/tree/master/services/kubos-service>`__

    - Abstracts the process of starting a service. Automatically fetches the IP information from
      the config file and presents the GraphQL and :ref:`GraphiQL <graphiql>` endpoints
    - Provides helper macros which can automatically collect and process errors when running
      operations against hardware

- `Juniper <https://graphql-rust.github.io/juniper/current/>`__ - Library for constructing the
  GraphQL schema
- `Failure <https://github.com/rust-lang-nursery/failure>`__ - Library used for error handling

Creating the Schema
-------------------

All services should implement the base service schema, as documented in the
:doc:`service outline <service-outline-guide>` doc.

At a high level, the service should present operations which can be broken into two categories:

- Queries allow users to fetch information about the state of the system and other telemetry items
- Mutations are operations which may affect the state of the system

We recommend implementing the most basic operations first (ping, no-op, reset) to establish the
initial service framework before moving on to the more complex (or unique) features.

In general, we don't bother to expose all possible functionality of a particular hardware device.
Instead, we focus on the most common functionality as well as the specific operations we know we'll
need.
This allows us to reduce the amount of development time required to create a new service.

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

.. note::

    If you choose to route messages to ``stdout``, they will only appear in the console of the user
    who started the process. As a result, services which are started automatically at boot will not
    issue messages to ``stdout`` if you log into the OBC at a later time.

Services should use the daemon logging facility (rather than the user facility).
This will cause all service log messages to be routed to the `/var/log/kubos-*.log` files.

Please refer to the :doc:`logging <../linux-docs/logging>` doc for more information about the setup
and behavior of log messages.

Service Configuration
---------------------

By default, all services require that the IP address and port of their GraphQL endpoint be defined
in the system's `config.toml` file.
It is generally assumed that any port which is not already listed in the config file is available
for use, however there are some existing conventions:

- Kubos core services use ports 8000-8079
- Communications services use ports 8080-8099 for their downlink ports
- Hardware services use ports 8100 and up

It may be useful for your service to have certain additional configurable settings.
For instance, you might want to be able to adjust certain timeout values, or change which device
bus your hardware is connected to.
In this case, your service should read the needed configuration values from the system's
`config.toml` file.

More information about setting and fetching configuration values can be found in the
:doc:`service config <service-config>` doc.

Testing Your Service
--------------------

The :ref:`GraphiQL <graphiql>` interface provides a good way to dynamically test each of your
service's operations.

Unit and integration tests are a good way to ensure that your service remains functional and
compatible with the KubOS ecosystem over time.
More information about setting up testing can be found in our :doc:`testing <../../contributing/testing>`
doc.

Creating an Init Script
-----------------------

If you would like your service to be automatically started at system boot, you will need to create
an init script.
KubOS uses BusyBox's init system, so the init scripts will need to use the following naming
convention: ``S{run-level}{application-name}``. The run-level value should be between 1 and 99.
The lower the value, the earlier it will be run in the system boot process.

.. note::

    The BusyBox init system does *not* require compliance with the `LSB init script <https://wiki.debian.org/LSBInitScripts>`__
    standard.

For Rust-based services, the `monitor service's <https://github.com/kubos/kubos-linux-build/blob/master/package/kubos/kubos-monitor/kubos-monitor>`__
init script provides a good example.
Rust services (and other executables) should be started using `start-stop-daemon <http://man7.org/linux/man-pages/man8/start-stop-daemon.8.html>`__.

For Python-based services, please refer to the `Pumpkin MCU service <https://github.com/kubos/kubos-linux-build/blob/master/package/kubos/kubos-pumpkin-mcu/kubos-pumpkin-mcu>`__
init script for reference.
Python services should be started by using the `python` command to start the service as a
background process.

Installing Your Service
-----------------------

Once you have finished service development, you should install the service in its final location in
your OBC.

Custom services may either live in the user data partition or in the root file system.
If the service interacts with core avionics or communications hardware, and is not expected to
change after launch, we recommend including it in the root file system for recovery purposes.
Only services included in the root file system are recovered automatically by the OS recovery
process.
Please refer to our :doc:`recovery architecture doc <../linux-docs/kubos-linux-recovery>` for
more information about our OS recovery system.

In either case, you will need to update your system's `config.toml` file in order to define the
IP address and port for your service's GraphQL endpoint.

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