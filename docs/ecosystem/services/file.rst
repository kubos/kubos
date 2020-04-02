File Transfer Service
=====================

The file transfer service is used to transfer files between the mission
operations center and the OBC. It may also be used to transfer files
between a developer's system and the OBC when in a development environment.

The service provides file transfer functionality by implementing the
:doc:`file protocol <../../deep-dive/protocols/file-protocol>`. The file protocol is UDP-based
which means a connection is required between the OBC and ground segment
capable of transferring UDP packets. This should be established using
a standard network connection.

Overview
--------

.. uml::

    @startuml
    
    hide empty description
    
    state "Receive Message" as Receive
    state "Export Request" as Export
    state "Import Request" as Import

    [*] --> Receive
    Receive : Process transaction request from client
    
    Receive -down-> Metadata
    Metadata : Save file metadata information
    Metadata --> Done
    
    Receive -left-> Export
    Export : Prepare to receive file from client
    Export --> Receiving
    
    state Receiving {
        state "Chunk Message" as Chunk
        state "Success" as Receive_Success
        state "Failure" as Receive_Failure
        state "Wait" as Receive_Wait
        state "ACK" as Receive_ACK
        state "NAK" as Receive_NAK
        state "Timeout" as Receive_Timeout
        
        [*] -down-> Verify
        Verify : Check file status
        
        Verify -right-> Receive_ACK : All chunks received
        Receive_ACK : Send ACK message to client
        Receive_ACK --> Finalize
        
        Verify -left-> Receive_NAK : Some chunks missing
        Receive_NAK : Send NAK message to client
        
        Receive_NAK --> Receive_Wait
        Receive_Wait : Wait for chunk data from client
        
        Receive_Wait -down-> Chunk
        Chunk : Save chunk data
        Chunk --> Receive_Wait
        
        Receive_Wait -right-> Receive_Timeout
        Receive_Timeout : Increase timeout counter
        Receive_Timeout -up-> Verify : Counter < limit
        Receive_Timeout --> Finalize : Counter >= limit
        
        Finalize : Re-assemble chunks and verify integrity
        
        Finalize --> Receive_Success : Hash verification passed
        Receive_Success : Send success message to client
        Receive_Success --> [*]
        
        Finalize --> Receive_Failure : Hash verification failed
        Receive_Failure : Send failure message to client
        Receive_Failure --> [*]
    }
    Receiving --> Done
    
    Receive -right-> Import
    Import : Prepare to transmit file to client
    
    Import --> Initialize
    Initialize : Import file into temporary storage and calculate hash
    
    Initialize --> Transmit_Failure
    Transmit_Failure : Send failure message to client
    Transmit_Failure --> Done    
    
    Initialize --> Transmit_Success
    Transmit_Success : Send success message to client with file metadata
    Transmit_Success --> Transmitting
    
    state Transmitting {
        state "Success" as Transmit_Success
        state "Failure" as Transmit_Failure
        state "Wait" as Transmit_Wait
        state "ACK" as Transmit_ACK
        state "NAK" as Transmit_NAK
        state "Timeout" as Transmit_Timeout
        state "Send Chunk" as Send
        
        [*] --> Transmit_Wait
        Transmit_Wait : Wait for file status message from client
        
        Transmit_Wait --> Transmit_ACK
        Transmit_ACK : Receive ACK from client
        Transmit_ACK --> [*]
        
        Transmit_Wait --> Transmit_NAK
        Transmit_NAK : Receive NAK from client
        Transmit_NAK -up-> Send
        
        Send --> Send : For all missing chunks
        Send : Send chunk data message to client
        Send -left-> Transmit_Wait
        
        Transmit_Wait --> Transmit_Timeout
        Transmit_Timeout --> Transmit_Wait : Counter < limit
        Transmit_Timeout : Increase timeout counter
        Transmit_Timeout --> [*] : Counter >= limit
    }
    
    Transmitting --> Done
    
    @enduml

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

.. note::

    This timeout is currently hardcoded to two seconds.

In order to support simultaneous client connections, whenever a message is received
on the main UDP socket, a new socket is spawned in order to handle the rest
of the transaction. As a result, after sending the initial import or export request,
the transfer client should listen for a reply and then use the new socket
as the destination for future transmissions.

Configuration
-------------

The file transfer service has several configuration options which may be
defined in the system's :doc:`config.toml <../services/service-config>` file:

    - ``[file-transfer-service]``

        - ``storage_dir`` - `Default: "file-transfer".` The directory which should be
          used for temporary storage of file chunks. Note: The directory will be
          created if it does not already exist.
        - ``timeout`` - `Default: 2.` The length of time, in seconds, for which the service
          should wait for new messages from the client once a file protocol transaction has
          been started
        - ``transfer_chunk_size`` - `Default: 1024.` Each file is broken up into equally sized
          chunks prior to transfer. This option specifies the size of those chunks
          in bytes.
        - ``hash_chunk_size`` - `Default: 2048.` Each file is broken up into equally sized
          chunks which are used to calculate the file's hash. This option specifies the size
          of those chunks.
        - ``hold_count`` - `Default: 5.` The number of times the protocol waits for
          a new message before ending the transaction.
        - ``downlink_ip`` - `Required.` The IP address that the file service responds to.
        - ``downlink_port`` - `Required.` The port that the file service responds to.
        - ``inter_chunk_delay`` - `Default: 1.` The delay, in milliseconds, taken 
          between the transmission of each chunk. This is to allow manual flow control.
        - ``max_chunks_transmit`` - `Optional.` The maximum number of chunks to transmit before
          waiting on a response. The default is to transmit the entire file.

    - ``[file-transfer-service.addr]``

        - ``ip`` - Specifies the service's IP address
        - ``port`` - Specifies the port on which the service will be listening for UDP packets

For example::

    [file-transfer-service]
    storage_dir = "my/storage/directory"
    timeout = 3600
    downlink_ip = "127.0.0.1"
    downlink_port = 8080
    
    [file-transfer-service.addr]
    ip = "0.0.0.0"
    port = 8040
    
Future configuration options:

    - Maximum number of timeout-retry attempts
    - Non-default destination IP/port

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

Communicating with the File Service
-----------------------------------

Users should establish a ground-side client which implements the :doc:`file protocol <../../deep-dive/protocols/file-protocol>`
in order to transfer files over their communications device.

Kubos provides an example `file transfer client <https://github.com/kubos/kubos/tree/master/clients/kubos-file-client>`__
to allow users to learn and experiment with the file transfer service prior to the formal file
transfer client being developed.
For more information, please refer to the :doc:`file transfer <../../tutorials/file-transfer>` tutorial.