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