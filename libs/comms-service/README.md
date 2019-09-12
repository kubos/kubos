# Communications Service Framework Library

This library provides a framework which allows users to define and start communication
services within their hardware services.

The framework is used to simplify the process of reading messages from the ground,
forwarding them to the appropriate internal destination, and then sending properly
formatted messages back to the ground.

Currently the framework supports SpacePacket messages which contain either a UDP or
GraphQL payload. 