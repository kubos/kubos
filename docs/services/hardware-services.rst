Hardware Services
=================

Hardware Services are GraphQL Server endpoints that take in Queries and Mutations and exercise the Hardware API to complete them. 

It is highly recommended that you first read about GraphQL :doc:`here <graphql>`.

Hardware Services function mostly as a pass through to the API, managing control of the resource. Each Hardware Service has only a single worker, so the hardware will not be damaged or report an incorrect state due to simultaneous conflicting transactions. 

Queries
-------

Queries are telemetry requests. Usually (if the hardware has the capability), when a Query is submitted, the Hardware Service retrieves the live data from the hardware component and completes the transaction, returning it in a JSON format. 
Mutations
---------

Mutations are hardware commands. Submitting a mutation changes the state of the hardware. 

Service Types
-------------

There are a few general types that most satellite bus hardware will fall into. Each of these has a Hardware Service that is generalized, but specific to the type of hardware:

 - Power Systems (EPS and/or Battery)
 - Attitude Determination and Control Systems (ADCS)
 - Global Positioning System (GPS)

Other hardware components are given services that are more generalized. 