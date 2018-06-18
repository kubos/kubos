Kubos Applications Service
==========================

Kubos' applications service is responsible for monitoring and managing all mission applications for a system.

Something something installation, upgrades, and recovery

TODO: User/App/Service interaction diagram

Overview
--------

Architecture. The file directory versioning system, plus the concept of everything being done by UUID, rather than app name.

Communicating with the Service
------------------------------

How to talk to the service. Do we have a good tool for this?

Querying
--------

- Get list of all applications and versions
- Query about specific application

Registering
-----------

TODO: How to transfer app to stack. Can probably link to some other doc

De-Registering (TODO: is that even a word?)
-------------------------------------------


Upgrading
---------

i.e. adding a new version of an existing application

- Get the UUID of the current app
- Register with said UUID
- Verify by querying?

Recovery
--------

Is not a thing that actually exists yet...

Q: Is it possible to do manual rollback?

Customizing the Applications Service
------------------------------------

- Service port
- Non-default registry location
- Any other options?
