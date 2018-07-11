File Protocol
=============

The file protocol is implemented by both the
:doc:`file transfer service <file>` and clients interacting
with the service. This protocol uses a content-addressable
methodology similar to git for storing and chunking files.
This document covers the content-addressable storage, all
messages used in the protocol, and includes diagrams
of common use cases.

Content-Addressable Storage
---------------------------

The file protocol uses a content-addressable system to store file data.
All files are broken up into 4KB chunks prior to sending. This chunking
is initiated either by an ``export`` or ``import`` message. A local
folder called ``storage`` is created by the file service and client
for storing the content-addressable information. Inside of ``storage``
each file has it's own folder. The folder name is a 16-bit
`blake2 hash <https://blake2.net/>`_ of the file's
contents. This folder is created as part of the import/export process.

Inside of each file's folder there is a ``meta`` file and chunk files.
Each ``meta`` file contains metadata about the file in a CBOR list.
Currently we only store the number of chunks in the ``meta`` file.
Each chunk file is also stored in this folder. Chunk files are named
after the hex representation of their chunk number. Each chunk file
contains the raw contents of that chunk.

Here is an example content-addressable storage structure containing
an eleven chunk file::

    storage/
    └── 852f1630f4ed2c0bc934d71ada618974/ <- blake2 hash of file
        ├── 0 <- Each of these are file chunks
        ├── 1
        ├── 2
        ├── 3
        ├── 4
        ├── 5
        ├── 6
        ├── 7
        ├── 8
        ├── 9
        ├── A
        └── meta <- Contains {"num_chunks" : 11 } in CBOR

Messages
--------

All messages in the file protocol are encoded as `CBOR` arrays and are sent
in UDP packets. The first value in the encoded list is the ``channel_id``
for request/response type messages and the ``hash`` for content-addressable
messages.

The ``hash`` parameter is the blake2 hash for the corresponding file
which is being transferred.

The ``channel_id`` parameter corresponds to an in-memory array of coroutines
created as part of the file transfer process.

Send Chunk
~~~~~~~~~~

This message is sent as part of the file ``import`` or ``export`` process.
It contains the file hash, chunk index and raw chunk data.

Each raw chunk is 4KB in size. Individual chunk messages will not get
an immediate reply. However if no chunks are received within the
timeout window then an ``ack`` or ``nak`` will be sent depending
on whether all the chunks have been received or not.

    ``{ hash, chunk_index, data }``

Sync
~~~~

This message is sent to query the message receiver on the status
of a file. It contains the file's hash and the expected number
of chunks for the file.

If the file does not exist on the receiver's side then the receiver
will send a ``nak`` requesting all chunks and create the appropriate
file storage structure. If the file does exist then the receiver will
send back an ``ack``. The ``num_chunks`` is optional, however it
should be sent in the first ``sync`` of an ``import`` or ``export``
to ensure the expected number of chunks is known.

    ``{ hash, num_chunks }``

Acknowledge (Ack)
~~~~~~~~~~~~~~~~~

This message is sent to inform the message receiver that the
message sender has all chunks for a given file. It contains the
file's hash, the boolean value true, and the number of
chunks in the file.

    ``{ hash, true, num_chunks }``

Negative Acknowledge (Nak)
~~~~~~~~~~~~~~~~~~~~~~~~~~

This message is sent to inform the message receiver that the
message sender does not have all chunks for a given file. It
contains the file's hash, the boolean value false, and a list
of missing chunks. The list of missing chunks is made up of
pairs of ranges where the first number is inclusive and the
second is exclusive. For example ``0, 2`` means the first
two chunks are missing.

A ``nak`` may be sent in response to a ``sync`` or after a
timeout during a file ``import`` or ``export``. The message
sender should expect the message receiver to send
the missing file chunks upon receipt of a ``nak``.

    ``{ hash, false 1, 4, 6, 7 }``

The above example ``nak`` indicates that chunks 1-3 and 6
are missing.

Export
~~~~~~

This message is sent to initiate the process of transferring
a file from the message sender to the message receiver. It
contains the channel id, the string "export", the file's hash,
the target path for the file and file's permissions mode.

The message receiver will begin waiting for file chunks after
receiving this message. Once the timeout triggers it will
attempt to export the file locally. If the file is incomplete then
the receiver will request any missing chunks. Upon receiving
all chunks it will attempt to verify and export the file to
the local filesystem. This message is sent after the
``sync`` command as part of the export process.

    ``{ channel_id, "export", hash, path, mode }``


Import
~~~~~~

This message is sent to initiate the process of transferring
a file to the message sender from the message receiver. It
contains the channel ID, the string "import", and the requested
file's path.

Upon receiving, the message receiver will import the requested
file into the managed content-addressable storage and send a
``success`` message to the sender. This ``success`` message
will contain the file`s hash and allow the original message
sender to determine which file chunks are required.

    ``{ channel_id, "import", path }``

Success
~~~~~~~

This message is sent as part of the ``import`` or ``export``
processes. It contains the channel ID, the boolean value true
and potentially other values depending on the situation.

This message is primarily sent in two different situations:
at the end of an ``export`` and near the beginning of an ``import``.
The message sender would send a ``success`` if an ``export``
has completed successfully. The ``success`` is also used
during an ``import`` to indicate a file is ready for sending
and to communicate the file's hash.

Extra values in this command appear as extra items in the list.

    ``{ channel_id, true, ..values }``

Failure
~~~~~~~

This message is sent if there as an error in the ``import`` or
``export`` process. It contains the channel ID, the boolean false
and the error message.

    ``{ channel_id, false, error_message }``

Common Protocol Usages
----------------------

Uploading a single chunk file from a ground station to an OBC:

.. uml::

    @startuml

    participant "Ground Station" as ground
    participant "OBC" as obc

    ground -> obc : Sync 
    ground -> obc : Export 
    obc -> ground : Nak
    ground -> obc : Send Chunk
    obc -> ground : Ack
    obc -> ground : Success

    @enduml

Downloading a single chunk file from an OBC to a ground station:

.. uml::

    @startuml

    participant "Ground Station" as ground
    participant "OBC" as obc

    ground -> obc : Import 
    obc -> ground : Success 
    ground -> obc : Nak
    obc -> ground : Send Chunk
    ground -> obc : Ack

    @enduml

Uploading a three chunk file from ground station with a chunk re-request:

.. uml::

    @startuml

    participant "Ground Station" as ground
    participant "OBC" as obc

    ground -> obc : Sync 
    ground -> obc : Export 
    obc -> ground : Nak
    ground -> obc : Send Chunk
    ground -> obc : Send Chunk
    obc -> ground : Nak
    ground -> obc : Send Chunk
    obc -> ground : Ack
    obc -> ground : Success

    @enduml
