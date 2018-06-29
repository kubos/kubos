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

Currently both the file service and file client are implement in Lua, so refer
to the :doc:`Lua SDK doc <../sdk-docs/sdk-lua>` for more detailed
Lua instructions.

Running The Service From KubOS
------------------------------

The Kubos Linux distribution (as of v1.3.0) ships with the file transfer
service installed and configured to run on boot. This can be verified by
booting the KubOS system, running the ``ps`` command and looking for the
``file-service`` process. If the service is not running then it can
be started like so::

    $ /etc/init.d/S90file-service start

Running The Service From Source
-------------------------------

The file transfer service can also be run from source if required.
The source is located in the folder ``kubos/services/file-service``
in the KubOS source repo. The service can be started like so::

    $ cd kubos/services/file-service
    $ lit install
    $ PORT=8010 luvi-regular .

The service will look for the environment variable ``PORT`` to determine
which port it should listen on. If ``PORT`` is not specified then by default
it will listen on port ``7000``.

Running The File Client From Source
-----------------------------------

The file client is located in the folder ``kubos/clients/file-client`` in the
`KubOS repo <https://github.com/kubos/kubos>`_. The client has two use cases:
`upload` and `dowload`.

Uploading a file is the act of is taking a file local to the client and sending
it to the file service. A file upload is done like so::

    $ cd kubos/clients/file-client
    # The lit command only needs to be run once
    $ lit install
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
