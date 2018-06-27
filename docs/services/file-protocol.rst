File Protocol
=============

The file protocol is implemented by both the :doc:`file service <file>` and
and clients interacting with the service. This protocol uses a content
addressable methodology similar to git for storing and chunking files.
This document covers the content addressable storage, all messages
used in the protocol and includes diagrams of common use cases.

Content Addressable Storage
---------------------------

The file protocol uses a content addressable system to store file data.
All files are broken up into 4kB chunks prior to sending. This chunking
is initiated either by an ``export`` or ``import`` message. A local
folder called ``storage`` is created by the file service and client
for storing the content addressable information. Inside of ``storage``
each file has it's own folder. The folder name is a 16-bit
`blake2 hash <https://blake2.net/>`_ of the file's
contents. This folder is created as part of the import/export process.

Inside of each file's folder there is a ``meta`` file and chunk files.
Each ``meta`` file contains meta data about the file in a cbor list.
Currently we only store the number of chunks in the ``meta`` file.
Each chunk file is also stored in this folder. Chunk files are named
after the hex representation of their chunk number. Each chunk file
contains the raw contents of that chunk.

Here is an example content addressable storage structure containing
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
for request/response type messages and the ``hash`` for content-addressable messages.

Send Chunk
~~~~~~~~~~

Files are sent to the remote side in 4kB chunks. This is the payload
message containing the chunks indexed by file hash and chunk index.

    ``{ hash, chunk_index, data }``

The protocol typically won't reply to these unless there is a long
pause in sending (such as after sending the last chunk). In this case
the receive will send either an ack or nak depending on if
all the chunks have been received or not.

Sync
~~~~

To query whether the other side has all the chunks for a file, send a
sync message. The ``num_chunks`` is optional, but it should be sent for
new files so the other side knows how many chunks to expect.

    ``{ hash, num_chunks }``

An acknowledge or negative acknowledge message will be sent in response.

Acknowledge (Ack)
~~~~~~~~~~~~~~~~~

This message tells the other side that the sender has all chunks
for a given hash file.

    ``{ hash, true, num_chunks }``

Negative Acknowledge (Nak)
~~~~~~~~~~~~~~~~~~~~~~~~~~

This message tells the other side that the sender is missing chunks.
The numbers after ``false`` are pairs of ranges where the first number is
inclusive and the second is exclusive. For example ``0, 2`` means the
first two chunks are missing.

    ``{ hash, false 1, 4, 6 ,7 }``

Export
~~~~~~

The sender wishes that the other side exports a file from content addressable
storage to somewhere on the normal file system. Typically this is the first
command when uploading a file to a remote service.

    ``{ channel_id, "export", hash, path, mode }``

If the chunks aren't there yet, then other side will request them. The export
command will wait to execute until all chunks are received.

Import
~~~~~~

Import is used to tell a remote side to import a file from the normal file system
into the managed content addressable storage. 

    ``{ channel_id, "import", path }``

Success
~~~~~~~

When an import or export command finishes with success, this will be received
with the results.

    ``{ channel_id, true, ..values }``

Failure
~~~~~~~

If there is an error in the import or export, this will be received with an
error message.

    ``{ channel_id, false, error_message }``

Common Protocol Usages
----------------------

Uploading a single chunk file from a ground station to an OBC:

.. uml::

    @startuml

    participant "OBC" as obc
    participant "Ground Station" as ground

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

    participant "OBC" as obc
    participant "Ground Station" as ground

    ground -> obc : Import 
    obc -> ground : Success 
    ground -> obc : Nak
    obc -> ground : Send Chunk
    ground -> obc : Ack

    @enduml

Uploading a three chunk file from ground station with a chunk re-request:

.. uml::

    @startuml

    participant "OBC" as obc
    participant "Ground Station" as ground

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

