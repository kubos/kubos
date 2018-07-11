Shell Service
=============

The shell service is used to provide shell access and commanding from
mission operations to the OBC. It may also be used between a developer's
system and the OBC when in a development or testing environment.

The shell service provides shell functionality by implementing the
:doc:`shell protocol <shell-protocol>`. The shell protocol is UDP-based
which means a connection is required between the OBC and ground segment
capable of transferring UDP packets. This could be established using the
:doc:`communication service <communication>` or a standard network connection.

Currently both the shell service and shell client are implemented in Lua,
so refer to the :doc:`Lua SDK doc <../sdk-docs/sdk-lua>` for more
detailed Lua instructions.

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
The source is located in the folder ``kubos/services/file-service``
in the KubOS source repo. The service can be started like so::

    $ cd kubos/services/shell-service
    # The lit command only needs to be run once
    $ lit install
    $ PORT=8011 luvi-regular .

The service will look for the environment variable ``PORT`` to determine
which port it should listen on. If ``PORT`` is not specified then by default
it will listen on port ``6000``.

Running the Shell Client from Source
------------------------------------

The shell client is located in the folder ``kubos/clients/shell-client`` in the
`KubOS repo <https://github.com/kubos/kubos>`_. The shell client can be used
to connect to the shell service and provides a fairly full-featured
terminal emulator. It is appropriate for development usage, but would not
operate well under flight conditions.

The shell client does not take any arguments. It is run like so::

    $ cd kubos/clients/shell-client
    # The lit command only needs to be run once
    $ lit install
    $ PORT=8011 luvi-regular .

The shell client will look for the environment variable ``PORT`` to determine
which port it should listen on. If ``PORT`` is not specified then by default
it will listen on port ``6000``.

Once started the shell client will query the file service for current
sessions and present the user with the option to start a new session
or continue an existing session.

Shell client prompt with no existing sessions::

    Choose an option:
    Press enter to start a new sh shell.
    Press Control-D to exit
    Or enter session ID to take over an existing session.
    >

Shell client prompt with an existing session::

    Choose an option:
    Press enter to start a new sh shell.
    Press Control-D to exit
    Or enter session ID to take over an existing session.
    39624	{ path = 'sh', pid = 19232 }
    > 

