# Command And Control
## Architecture Overview {#command-and-control-overview}

```

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
```


The command line client is a binary exposed on the KubOS Linux command line.

Commands entered into this client will be encoded into a CBOR (Concise Binary Object Representation)[http://cbor.io/] message format and packed into a CSP packet and sent to the command service.

Existing functionality in the Kubos platform will be exposed in groups in a series of libraries.

The command service will first parse and look for an action argument, a group (library) name, and a set of optional arguments. These will be used as the following:

* Action argument - The corresponding member function of the service interface that will be called (ie. exec, status, help, output).
* Group name - The base name of the library containing the desired functionality (Core, Telemetry, CSP, HAL, IPC, etc.).
* Optional arguments - Passed through to the service implementation to perform more specific tasks.

The command service will look to load a corresponding library from a fixed path containing all of the Kubos and user defined libraries. Libraries will follow the lib<group_name>.so naming convention.

The appropriate function of the service interface will be called with the remaining arguments.

Once the API call returns, a CBOR-encoded message containing the execution time, the return code and stdout will be returned to the client.


## Module Design {#module-design}


Modules will need to implement to service interface and be compiled into a shared library in order to be compatible with this command and control system.

These functions will need to accept and parse arguments and handle routing the arguments to the desired functionality in that library.


```
     +--------------------------------------+
     |          Service Interface           |
     +--------------------------------------|
     |                                      |
     | int execute(int argc, char **argv)   |
     |                                      |
     | int status(int argc, char **argv)    |
     |                                      |
     | int help(int argc, char **argv)      |
     |                                      |
     | int output(int argc, char **argv)    |
     |                                      |
     +--------------------------------------+
```


## Library Implementation {#library-implementation}

Libraries exposed through the command and control framework will need to implement the Service Interface.

Note: Currently on the stdout from the library execution is returned to the client after running a command.

Currently only the core library (core commands for the "core" command and control commands) is implemented. Future releases will have further examples of shared libraries.

See the (core library)[<link to library code when #63 is merged>] for an example of a library following this format.

