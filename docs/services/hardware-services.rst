Hardware Services
=================

Hardware services are GraphQL server endpoints that take in queries and mutations and exercise the hardware API to complete them.

It is highly recommended that you first read about GraphQL:

 - :doc:`GraphQL <graphql>`

Hardware services function mostly as a passthrough to the API, managing control of the resource.
Each hardware service has only a single worker thread, so the hardware will not be damaged or report an incorrect state due to simultaneous conflicting transactions.

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

There are a few common categories of satellite bus hadware:

 - Power Systems (EPS and/or Battery)
 - Attitude Determination and Control Systems (ADCS)
 - Global Positioning System (GPS)

Other hardware components are given services that are more generalized.

Creating a Service
------------------

The :doc:`service outline guide.<../dev-docs/service-outline-guide>` documents the preferred schema for all hardware services.
Commonizing the queries available from each service allows greater code re-use and reduces the number of new queries users must learn when swapping between different hardware devices.

Pre-Built Services
------------------

Some hardware services have been created by Kubos and are available for use.

The following list gives links to each service's documentation:

    - |MAI-400|
    - |ISIS-AntS|
    - |iOBC-Supervisor|
    - |NovAtel-OEM6|
    - `Pumpkin Supervisor MCUs <https://github.com/kubos/kubos/blob/master/services/pumpkin-mcu-service/README.rst>`__
    
.. |MAI-400| raw:: html
 
    <a href="../rust-docs/mai400_service/index.html" target="_blank">Adcole Maryland Aerospace MAI-400 ADACS</a>

.. |ISIS-AntS| raw:: html
 
    <a href="../rust-docs/isis_ants_service/index.html" target="_blank">ISIS Antenna Systems</a>

.. |iOBC-Supervisor| raw:: html
 
    <a href="../rust-docs/iobc_supervisor_service/index.html" target="_blank">ISIS-OBC Supervisor</a>

.. |NovAtel-OEM6| raw:: html
 
    <a href="../rust-docs/novatel_oem6_service/index.html" target="_blank">NovAtel OEM6 High Precision GNSS Receivers</a>

.. note:: 

    In order to be included in the OBC and automatically started at boottime, the package for each hardware service
    must be enabled when building Kubos Linux
