Hardware Services
=================

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them.

It is highly recommended that you first read about GraphQL:

 - :doc:`GraphQL <graphql>`

Hardware services function mostly as a passthrough to the API, managing control of the resource.
Each hardware service has only a single worker thread, so the hardware will not be damaged or report an incorrect state due to simultaneous conflicting transactions.

In order to see the the full schema of a service, which details its available commands and telemetry
objects, connect to the service's :ref:`GraphiQL <graphiql>` endpoint and click the "Docs" button
in the upper-right hand corner.

Queries
-------

Queries are telemetry requests.
Usually (if the hardware has the capability), when a query is submitted, the hardware service retrieves the live data from the hardware component and completes the transaction, returning it in a JSON format.

Mutations
---------

Mutations are hardware commands.
Submitting a mutation changes the state of the hardware.

Service Types
-------------

There are a few common categories of satellite bus hardware:

 - Power Systems (EPS and/or Battery)
 - Attitude Determination and Control Systems (ADCS)
 - Global Positioning System (GPS)

Other hardware components are given services that are more generalized.

Creating a Service
------------------

The :doc:`service outline guide <service-outline-guide>` documents the preferred schema for all hardware services.
Commonizing the queries available from each service allows greater code re-use and reduces the number of new queries users must learn when swapping between different hardware devices.

The :doc:`service development guide <service-dev>` goes into a bit more detail about the components
needed to create and install a new hardware service.

.. _pre-built-services:

Pre-Built Services
------------------

Some hardware services have been created by Kubos and are available for use.

The following list gives links to each service's documentation:

    - :doc:`Monitor Service <monitor-service>` - This service is unique in that it communicates with
      the OBC itself, rather than an external hardware device
    - |MAI-400|
    - |Clydespace-EPS|
    - |ISIS-AntS|
    - |iOBC-Supervisor|
    - |NovAtel-OEM6|
    - |NSL Duplex|
    - `Pumpkin Supervisor MCUs <https://github.com/kubos/kubos/blob/master/services/pumpkin-mcu-service/README.rst>`__

.. |MAI-400| raw:: html

    <a href="../../rust-docs/mai400_service/index.html" target="_blank">Adcole Maryland Aerospace MAI-400 ADACS</a>

.. |Clydespace-EPS| raw:: html

    <a href="../../rust-docs/clyde_3g_eps_service/index.html" target="_blank">Clyde Space 3rd Generation EPS</a>

.. |ISIS-AntS| raw:: html

    <a href="../../rust-docs/isis_ants_service/index.html" target="_blank">ISIS Antenna Systems</a>

.. |iOBC-Supervisor| raw:: html

    <a href="../../rust-docs/iobc_supervisor_service/index.html" target="_blank">ISIS-OBC Supervisor</a>

.. |NovAtel-OEM6| raw:: html

    <a href="../../rust-docs/novatel_oem6_service/index.html" target="_blank">NovAtel OEM6 High Precision GNSS Receivers</a>

.. |NSL Duplex| raw:: html

    <a href="../../rust-docs/nsl_duplex_d2_comms_service/index.html" target="_blank">NSL Duplex D2 Radio</a>

.. note::

    In order to be included in the OBC and automatically started at boot time, the package for each hardware service
    must be enabled when building Kubos Linux
