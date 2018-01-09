Command and Control
===================

Architecture Overview
---------------------

::


                                                    Daemon Process
                 +------------------------------------------------------------------------------------------+
                 |                                                                                          |
                 |                                                                                          |
                 |                                        Kubos                                             |
                 |                                                                     +-----------------+  |
                 |                                                             +-----> | service1        |  |
                 |                                                             |       +-----------------+  |
    +---------+  |   +-------------------------+     +----------------------+  |       +-----------------+  |
    | Command |  |   |                         |     |                      |  +-----> | service2        |  |
    | Line    | <----+                         |     |                      |  |       +-----------------+  |
    | Client  |  |   |                         |     |                      |  |       +-----------------+  |
    |         +----> |       Command Service   +---> |   Command Registry   +--------> | service3        |  |
    |         |  |   |                         |     |     (Directory)      |  |       +-----------------+  |
    |         |  |   |                         |     |                      |  |       +-----------------+  |
    |         |  |   |                         |     |                      |  +-----> | service4        |  |
    +---------+  |   +-------------------------+     +----------------------+  |       +-----------------+  |
                 |                                                             |       +-----------------+  |
                 |                                                             +-----> | service5        |  |
                 |                                                                     +-----------------+  |
                 |                      +------------------------------+                                    |
                 |                      |      Service Interface       |                                    |
                 |                      +------------------------------+                                    |
                 |                      |                              |                                    |
                 |                      |  + execute                   |                                    |
                 |                      |  | status                    |                                    |
                 |                      |  | help                      |                                    |
                 |                      |  + ...                       |                                    |
                 |                      |                              |                                    |
                 |                      +------------------------------+                                    |
                 |                                                                                          |
                 +------------------------------------------------------------------------------------------+

Overview
--------

Command and control is the system by which commands are received and
executed on the satellite. Commands can be received from external (a
radio) or internal (the command line client) sources. The received
commands are routed to the appropriate service.

Services can expose or implement any desired functionality that is
needed for a specific hardware interaction, future job scheduling or
mission requirements.

System Design
-------------

The command line client is exposed in the KubOS Linux shell as the
``c2`` command.

Commands entered into this client will be encoded into a CBOR `Concise
Binary Object Representation <http://cbor.io/>`__ message format and
packed into a CSP packet and sent to the command service.

Existing functionality in the Kubos platform will be exposed in groups
bundled into unique services.

The command service will first parse and look for a service (executable)
name, and a set of optional arguments. These will be used as follows:

-  Service name - The base name of the service containing the desired
   functionality (Core, Telemetry, CSP, HAL, IPC, etc.).
-  Optional arguments - The remaining arguments provided will be passed
   through to the service to parse and handle as it pleases.

The command service will look for a corresponding executable from a
fixed path (``/usr/local/kubos``) containing all of the service
executables.

Once the API call returns, a CBOR-encoded message containing the
execution time, the return code and any output from the command will be
returned to the client.

**Note:** Currently only the stdout from the service execution is
returned to the client after running a command.

Service Design
--------------

Services will need to be compiled as binaries and have the appropriate
argument parsing abilities to expose their desired functionality. See
the `Kubos Core Command
Service <https://github.com/kubos/kubos/tree/master/commands>`__ for
an example of how this is implemented.

Existing Services
-----------------

Currently only the "core" commands service is implemented for the ISIS
iOBC. This library implements the following commands:

+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+
| Command         | Function                                                                                                                             |
+=================+======================================================================================================================================+
| ping            | Run a "no-op" command through the system to ensure it is active and configured correctly                                             |
+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+
| info            | Returns the "build info" or version information from the iOBC supervisor                                                             |
+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+
| reboot          | Power cycles the iOBC for several seconds.                                                                                           |
+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+
| reset           | Performs a full software reset of the iOBC supervisor and iOBC                                                                       |
+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+
| emergency-reset | Performs an immediate reset of the iOBC and supervisor. This command can be damaging to the supervisor. Reset should be used instead |
+-----------------+--------------------------------------------------------------------------------------------------------------------------------------+

Examples
--------

Running one of the above commands from the core command library
would look like the following:

Ping:

::

        $ c2 core ping

Info:

::

        $ c2 core info

Congratulations! You have now run your first commands through the command
and control system
