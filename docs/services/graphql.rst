GraphQL
=======

GraphQL is a query language used to simplify commanding and telemetry gathering on the satellite. More information about GraphQL in general can be found here: http://graphql.org/ 


What uses GraphQL? 
------------------

All Hardware Services are GraphQL endpoints where: 

 - mutations   = hardware commands
 - queries     = telemetry requests

Other services, such as the Telemetry Database Service also use GraphQL as their command and telemetry request language. 

All responses to GraphQL transactions are in JSON format. 

Why use a query language?
-------------------------

Using a human readable query language makes it obvious exactly what the satellite is going to do. There is no analyzing hex output or parsing bits to understand that the command is going to do exactly what I want it to. 

Why GraphQL?
------------

GraphQL gives the user more control over the endpoint, rather than dictating what every interaction entails. 

For example, say a mission application needs to quickly get the current status of a single power port on a module to check if a payload is powered. For a ReST endpoint, a separate GET would be required to be availabe with just that single telemetry item. Now say there are 15 different power ports, and those assignments can change depending on the payload configuration. For each port, the endpoint would have to change to accommodate the individual request, adding a new request for any subset of telemetry they want retrieved. If a combonation of 2 or more is required, it would have to be done in separate transactions. With GraphQL, it's only a single query. The mission application is simplified to only ever be a single query and the endpoint doesn't have to add queries to accommodate each change in configuration.

Also, it is also already supported in several languages, speeding up development substantially. 
