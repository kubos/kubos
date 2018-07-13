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
