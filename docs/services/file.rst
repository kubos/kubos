File Service
============

The file service is a process implementing the :doc:`file protocol <file-protocol>`
meant to run on an OBC in either a development or flight environment. The file
protocol is UDP-based which means a connection is required between the OBC
and ground segment capable of transferring UDP packets. This could be established
using the :doc:`communication service <communication>` or a standard network connection.

Currently both the file service and file client are implement in Lua, so refer
to the :doc:`Lua SDK instructions <../sdk-docs/sdk-lua>` for more detailed
Lua instructions.

Running the service from source
-------------------------------

The file service is located in the folder ``kubos/services/file-service`` in the
KubOS source repo. The service can be started like so::

    $ cd kubos/services/file-service
    $ PORT=8010 luvi-regular .

The file service will look for the environment variable ``PORT`` to determine
which port it should listen on. If ``PORT`` is not specified then by default
it will listen on port ``7000``.

Running the file client from source
-----------------------------------

The file client is located in the folder ``kubos/clients/file-client`` in the
KubOS source repo. The client has two use cases: `upload` and `dowload`.

Uploading a file is the act is taking a file local to the client and sending
it to the file service. A file upload is done like so::

    $ cd kubos/clients/file-client
    $ PORT=8010 luvi-regular . -- upload local/file/path [remote/file/path]

All uploads must specify a local file to upload. Optionally they may specify
a remote path for the file. If a remote path is not specified then the file
will be saved in the folder the service is running out of.

Downloading a file is the act is retrieving a file which is local to the file
service and saving it in a location local to the file client. A file download
is done like so::

    $ cd kubos/clients/file-client
    $ PORT=8010 luvi-regular . -- download remote/file/path [local/file/path]

All downloads must specify a remote file to download. Optionally they may specify
a local path for the file. If a local path is not specified then the file will
be saved in the folder the file client is running out of.

The file client will look for the environment variable ``PORT`` to determine
which port it should listen on. If ``PORT`` is not specified then by default
it will listen on port ``7000``.
