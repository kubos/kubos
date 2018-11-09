Shell Service
=============

The shell service is used to provide shell access and commanding from
mission operations to the OBC. It may also be used between a developer's
system and the OBC when in a development or testing environment.

The shell service provides shell functionality by implementing the
:doc:`shell protocol <../../apis/kubos-libs/shell-protocol>`. The shell protocol is UDP-based
which means a connection is required between the OBC and ground segment
capable of transferring UDP packets. This should be established using
a standard network connection.

Configuration
-------------

The shell service has a couple configuration options which may be
defined in the system's ``config.toml`` file:
          
    - ``[shell-service.addr]``
    
        - ``ip`` - Specifies the service's IP address
        - ``port`` - Specifies the port on which the service will be listening for UDP packets
        
For example::

    [shell-service.addr]
    ip = "0.0.0.0"
    port = 8010


Running the Service from KubOS
------------------------------

The Kubos Linux distribution (as of v1.3.0) ships with the shell 
service installed and configured to run on boot. This can be verified by
booting the KubOS system, running the ``ps`` command and looking for the
``shell-service`` process. If the service is not running then it can
be started like so::

    $ /etc/init.d/S90shell-service start

Running the Service from Source
-------------------------------

The shell service can also be run from source if required.
The source is located in the folder ``kubos/services/shell-service``
in the KubOS source repo. The service can be started like so::

    $ cd kubos/services/shell-service
    $ cargo run -- -c config.toml

The service will look for the given ``config.toml`` file in order to get the
needed configuration options.
