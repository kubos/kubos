Shell Protocol
==============

The shell protocol is implemented by both the
:doc:`shell service <../../../ecosystem/services/shell>` and any clients interfacing
with the service.

All messages in the shell protocol are encoded as `CBOR` arrays
and are sent in UDP packets. The first value in the encoded list
is the ``channel_id``. The second value will be the ``command``
and any additional values will be parameters for the ``command``.

    ``{ channel_id, command, parameters.. }``

APIs
----

- |shell-protocol|
- |cbor-protocol|
- |channel-protocol|

 .. |shell-protocol| raw:: html

    <a href="../../rust-docs/shell_protocol/index.html" target="_blank">Shell Protocol</a>

 .. |cbor-protocol| raw:: html

    <a href="../../rust-docs/cbor_protocol/index.html" target="_blank">CBOR Protocol</a>

 .. |channel-protocol| raw:: html

    <a href="../../rust-docs/channel_protocol/index.html" target="_blank">Channel Protocol</a>

Messages
--------

The primary purpose of the shell protocol is to allow the
spawning and controlling of remote processes. These messages
are used by shell clients to direct the shell service in
this work of manipulating processes.

Spawn Process
~~~~~~~~~~~~~

This message is sent to the shell service to request a child
process to be spawned. It contains a channel ID, the string
'spawn', a command, and spawn options. The command can be an
absolute path to a binary or something in the system ``$PATH``.

    ``{ channel_id, 'spawn', command, options.. }``

There is currently only one available option for the ``options``
argument:

    - ``args`` - An array of arguments to pass to the child process

Example of starting a shell:

    ``{ 1, 'spawn', 'sh', { args = { '-l' } } }``

Write to Stdin
~~~~~~~~~~~~~~

This message is sent to the shell service to write data
to the stdin of a child process. It contains a channel ID,
the string 'stdin', and a data string. The data string
will be written directly to the stdin of the child process.

    ``{ channel_id, 'stdin', data }``

Close Stdin
~~~~~~~~~~~

This message is sent to the shell service to close the
stdin of a child process. It contains a channel ID and
the string 'stdin'.

After this message is received the shell service will close
the stdin pipe for the specified child process. Any future
messages attempting to write to stdin for this process will
result in an error.

    ``{ channel_id, 'stdin' }``

Send Signal
~~~~~~~~~~~

This message is sent to the shell service to signal a
child process. It contains a channel ID, the string 'kill',
and optionally a signal number. If the signal number is
omitted then `SIGTERM` will be sent.

    ``{ channel_id, 'kill', signal }``

A list of available signals can be found
`here <http://man7.org/linux/man-pages/man7/signal.7.html>`_.

Example usages:

Send `SIGTERM` to a child process:

    ``{ channel_id, 'kill' }``

Send `SIGKILL` to a child process:

    ``{ channel_id, 'kill', 9 }``

Process Created
~~~~~~~~~~~~~~~

This message is sent from the shell service when a process
has been created. It contains the channel ID, the string 'pid'
and a decimal number which is the pid.

    ``{ channel_id, 'pid', pid }``

Example message - A process has been created with a pid of 10:

    ``{ 1, 'pid', 10 }``

Stdout Data
~~~~~~~~~~~

This message is sent from the shell service when a process
has produced data via `stdout`. It contains the channel ID,
the string 'stdout', and a string of the stdout data.

    ``{ channel_id, 'stdout', data }``

Example message - ``ls`` producing directory output of `kubos-shell-client`:

    ``{ 12, 'stdout', 'Cargo.toml\nsrc\n' }``

Stdout Closed
~~~~~~~~~~~~~

This message is sent from the shell service when a process's
stdout pipe has been closed. It contains the channel ID and
the string 'stdout'.

    ``{ channel_id, 'stdout' }``

Stderr Data
~~~~~~~~~~~

This message is sent from the shell service when a process
has produced data via `stderr`. It contains the channel ID,
the string `stderr`, and a string of the stderr data.

    ``{ channel_id, 'stderr', data }``

Example message - The result of running ``ls`` with an invalid argument:

    ``{ 13, 'stderr', "Try 'ls --help' for more information.\n" }``

Stderr Closed
~~~~~~~~~~~~~

This message is sent from the shell service when a process's
stderr pipe has been closed. It contains the channel ID and
the string 'stderr'.

    ``{ channel_id, 'stderr' }``

Process Exited
~~~~~~~~~~~~~~

This message is sent from the shell service when a process
has exited. It contains the channel ID, the string 'exit',
the exit signal and the exit code.

    ``{ channel_id, 'exit', code, signal }``

Example messages

The result of a process exiting normally:

    ``{ 14, 'exit', 0, 0 }``

The result of sending a SIGKILL to a process:

    ``{ 14, 'exit', 0, 9 }``

Request List of Processes
~~~~~~~~~~~~~~~~~~~~~~~~~

This message is sent to the shell service to request a list
of the current processes running in the shell service. It
contains the channel ID and the string 'list'.

    ``{ channel_id, 'list' }``

List of Processes
~~~~~~~~~~~~~~~~~

This message is sent from the shell service when a list
of processes is requested. It contains the channel ID,
the string 'list', and a list of objects containing
process information (channel_id, path and pid). The
channel ID can be used to communicate with the corresponding
process in the list.

    ``{ channel_id, 'list', { [channel_id] = { path, pid } } }``

Example list of processes:

    ``{ 16, 'list', { [12] = { path = 'sh', pid = 45 }, [14] = { path = 'sh', pid = 50 } } }``


Example Usages
--------------

Running a Short-Lived Process
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The goal here is to run ``uname -a`` on a remote machine
via the shell service and see the output. The shell client
randomly chooses ``35`` as its ``channel_id`` and sends a
``spawn`` command with the arguments.

::

    Client: { 35, 'spawn', 'uname', { args = {'-a'} } }

The service sends back multiple messages in quick
succession because this is a short-lived process.

::

    Server: { 35, 'pid', 26191 }
    Server: { 35, 'stdout', 'Linux vagrant 4.4.0-128-generic #154-Ubuntu SMP Fri May 25 14:15:18 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux' }
    Server: { 35, 'stdout' }
    Server: { 35, 'stderr' }
    Server: { 35, 'exit', 0, 0 }

Running a Long-Lived Process
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The goal here is to open a ``bash`` shell on a remote
machine via the shell service and use that shell to
execute commands.

Starting the Process
^^^^^^^^^^^^^^^^^^^^

The shell client randomly chooses ``55`` as its ``channel_id``
and sends a ``spawn`` command with the arguments.

::

    Client: { 55, 'spawn', 'sh', { detached = true, pty = true, args = { '-l' } } }

The service responds back with the ``pid`` of the newly
created process.

::

    Server: { 55, 'pid', 26825 }
    Server: { 55, 'stdout', '\027kvagrant@vagrant:/home/vagrant\027\\' }
    Server: { 55, 'stdout', '[vagrant@vagrant vagrant]$ ' }


Finding the Process
^^^^^^^^^^^^^^^^^^^

The shell client can send the ``list`` command over a new ``channel_id``
to find this process and its information.

::

    Client: { 65, 'list' }

The service responds with the list of current processes.

::

    Server: { 65, 'list', { [55] = { path = '/bin/sh', pid = 26825 } } }

Sending Data to the Process
^^^^^^^^^^^^^^^^^^^^^^^^^^^

The shell client can use the ``channel_id`` to send data to the
``stdin`` of the process.

::

    Client: { 55, 'stdin', 'echo hello\n' }

The server will write this data to the ``stdin`` of the process
and send back any data received over ``stdout``.

::

    Server: { 55, 'stdout', 'echo hello\r\n' }
    Server: { 55, 'stdout', 'hello\r\n\027kvagrant@vagrant:/home/vagrant\027\\' }
    Server: { 55, 'stdout', '[vagrant@vagrant vagrant]$ ' }

Killing the Process
^^^^^^^^^^^^^^^^^^^

Once the shell client is finished it can use the ``kill`` command
to terminate the process.

::

    Client: { 55, 'kill' }

The service will terminate the process, respond with any data which was
sent via ``stdout`` or ``stderr`` and send the ``exit`` message.

::

    Server: { 55, 'stdout', 'logout\r\n' }
    Server: { 55, 'exit', 0, 0 }

Future Messages
---------------

These messages may be implemented in the shell protocol in the future,
but are not implemented as of KubOS release v1.8.0.

Spawn Process
~~~~~~~~~~~~~

The spawn process is currently implemented, however the following
optional arguments are not currently implemented:

    - ``pty`` - A boolean specifying whether a new pty is needed
    - ``env`` - An array of environment variable entries in the form ``"KEY=val"``
    - ``cwd`` - The current working directory of the child process
    - ``uid`` - The uid of the process
    - ``gid`` - The gid of the process
    - ``detached`` - Determines if the child process should be detached from the service

Resize Terminal
~~~~~~~~~~~~~~~

This message is sent to the shell service to resize the pseudo
terminal of a child process, if one exists. It contains a
channel ID, the string 'resize', the desired number of columns
and the desired number of rows.

    ``{ channel_id, 'resize', columns, rows }``

Example message - Resizing a pseudo terminal to 10x10:

    ``{ 1, 'resize', 10, 10 }``
