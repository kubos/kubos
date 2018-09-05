File Transfer Service
=====================

The file transfer service is used to transfer files between the mission
operations center and the OBC. It may also be used to transfer files
between a developer's system and the OBC when in a development environment.

The service provides file transfer functionality by implementing the
:doc:`file protocol <file-protocol>`. The file protocol is UDP-based
which means a connection is required between the OBC and ground segment
capable of transferring UDP packets. This could be established using the
:doc:`communication service <communication>` or a standard network connection.

Overview
--------

The file transfer service listens for requests on its configured UDP socket.

When a message is received, it is then processed using the file protocol message engine.
This logic keeps track of the current state of each client connection and takes
the appropriate action depending on the current state and the particular message received.

Actions may also be taken if the service experiences a timeout while waiting for
a follow-up message from a client. For example, if a client initiates an export operation
and then stops communicating while in the middle of sending file chunks, the service
will timeout, check the current status of the file, and then send a NAK to the client
with the current missing chunks. Receiving this NAK should cause the client to
resume transmitting file chunk data.

In order to support simultaneous client connections, whenever a message is received
on the main UDP socket, a new socket is spawned in order to handle the rest
of the transaction. As a result, after sending the initial import or export request,
the transfer client should listen for a reply and then use the new socket
as the destination for future transmissions.

Configuration
-------------

The file transfer service has several configuration options which may be
defined in the system's ``config.toml`` file:

    - ``[file-transfer-service]``
    
        - ``storage_dir`` - `Default: "file-transfer".` The directory which should be
          used for temporary storage of file chunks. Note: The directory will be
          created if it does not already exist.
          
    - ``[file-transfer-service.addr]``
    
        - ``ip`` - Specifies the service's IP address
        - ``port`` - Specifies the port on which the service will be listening for UDP packets
        
For example::

    [file-transfer-service]
    storage_dir = "my/storage/directory"
    
    [file-transfer-service.addr]
    ip = "0.0.0.0"
    port = 7000

Running the Service from KubOS
------------------------------

The Kubos Linux distribution (as of v1.3.0) ships with the file transfer
service installed and configured to run on boot. This can be verified by
booting the KubOS system, running the ``ps`` command and looking for the
``file-service`` process. If the service is not running then it can
be started like so::

    $ /etc/init.d/S90file-service start

Running the Service from Source
-------------------------------

The file transfer service can also be run from source if required.
The source is located in the folder ``kubos/services/file-service``
in the KubOS source repo. The service can be started like so::

    $ cd kubos/services/file-service
    $ cargo run -- -c config.toml

The service will look for the given ``config.toml`` file in order to get the
needed configuration options.

Communicating with the Service
------------------------------

The KubOS repo contains a `file transfer client program <https://github.com/kubos/kubos/tree/master/clients/file-client-rust>`__ 
which can be used to send and receive files to/from the service.