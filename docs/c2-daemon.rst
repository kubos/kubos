Command and Control (C2) Daemon
===============================

Overview
--------

The C2 daemon is a service that listens to the command line client on a pair of
named pipes. In a future release of KubOS Linux this will be migrated to using
TCP sockets. The client submits a command and its arguments to be run. The daemon
executes the command and returns the output.


Protocol Overview
-----------------

The command line client and the daemon communicate by transferring `CBOR <http://cbor.io/>`__ encoded data inside of CSP packets.
This allows for a dynamic protocol and for simple passing of messages with varying
data payloads.

These messages are passed between a client and the daemon. A client will initiate
a C2 "command transaction" by issuing a request. A transaction is comprised of
a command request which is processed by the command daemon and completed when
the daemon returns a response.

Daemon Input Message Format
---------------------------

The daemon accepts a CSP packet containing an encoded CBOR payload with the following fields:

+-----------------+--------+------------------------------------------------------------------+
| Field           | Type   | Use                                                              |
+=================+========+==================================================================+
| MSG_TYPE        | Int    | Designates the command type                                      |
+-----------------+--------+------------------------------------------------------------------+
| ARG_COUNT       | Int    | The number of arguments following the command                    |
+-----------------+--------+------------------------------------------------------------------+
| COMMAND_NAME    | String | The command to be run                                            |
+-----------------+--------+------------------------------------------------------------------+
| ARGS            | Array  | An array of argument values                                      |
+-----------------+--------+------------------------------------------------------------------+

Daemon Output Message Format
----------------------------

Upon a successful command request processing and execution the daemon will
respond with a CSP packet containing an encoded CBOR payload with the following
fields:

+-----------------+--------+------------------------------------------------------------------+
| Field           | Type   | Use                                                              |
+=================+========+==================================================================+
| RETURN_CODE     | Int    | The return code of the command that was run                      |
+-----------------+--------+------------------------------------------------------------------+
| EXEC_TIME       | Double | The amount of time that the command took to run                  |
+-----------------+--------+------------------------------------------------------------------+
| OUTPUT          | String | The stdout of the command that was run                           |
+-----------------+--------+------------------------------------------------------------------+

If there is an error that occurs while processing a command request, the daemon
will return an error packet containing the following fields:

+-----------------+--------+------------------------------------------------------------------+
| Field           | Type   | Use                                                              |
+=================+========+==================================================================+
| MSG_TYPE        | Int    | Designates the response type                                     |
+-----------------+--------+------------------------------------------------------------------+
| ERROR_MSG       | Int    | The description of the error that occurred                       |
+-----------------+--------+------------------------------------------------------------------+



