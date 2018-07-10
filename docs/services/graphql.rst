GraphQL
=======

GraphQL is a query language used to simplify commanding and telemetry gathering on the satellite. More information about GraphQL in general can be found here: http://graphql.org/


What uses GraphQL?
------------------

All :doc:`hardware services <hardware-services>` are GraphQL endpoints where:

 - mutations   = hardware commands
 - queries     = telemetry requests

Other services, such as the telemetry database service, also use GraphQL as their command and telemetry request language.

All responses to GraphQL transactions are in JSON format.

Why Use a Query Language?
-------------------------

Using a human readable query language makes it obvious exactly what the satellite is going to do, removing the need to analyze hex output or parse bits, which can obfuscate system operations.

Why GraphQL?
------------

GraphQL gives callers more control over how they fetch data. Where REST exposes the business domain as URL-addressable resources that clients fetch as needed, **GraphQL models the domain as a graph of fields** (sometimes with arguments) that may be fetched in a single query.

For example, say a mission application needs to quickly get the current status of a single power port on a module to check if a payload is powered. For a ReST endpoint, a separate GET would be required to be available with just that single telemetry item. Now say there are 15 different power ports, and those assignments can change depending on the payload configuration.

::

  GET localhost:9001/battery/powerport/1

For each port, the endpoint would have to change to accommodate the individual request, adding a new request for any subset of telemetry they want retrieved. If a combination of 2 or more is required, it would have to be done in separate transactions.

::

  GET localhost:9001/battery/powerport/1
  GET localhost:9001/battery/powerport/2
  GET localhost:9001/battery/powerport/3
  ...
  GET localhost:9001/battery/powerport/15

With GraphQL, it's a single query. The mission application is simplified and the endpoint doesn't have to add queries to accommodate each change in configuration.

::

  query {
    battery {
      ...powerports
    }
  }
