Kubos Services
==============

Kubos Services are defined as any persistent process that is used to interact with the satellite. Services rarely make decisions, but will allow the user to accomplish typical Flight Software tasks such as telemetry storage, file management, shell access, hardware interaction, etc. 

There are 3 main types of services:

 - :doc:`Hardware Services <hardware-services>`
 - :doc:`Core Services <core-services>`
 - :doc:`Payload Services <example-payload-service>`

Refer to each specific type to understand more about what they do.

.. toctree::
    :maxdepth: 1
    
    Hardware Services <hardware-services>
    Payload Service with Example <example-payload-service>
    Core Services <core-services>
    