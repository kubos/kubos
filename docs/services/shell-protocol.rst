Shell Protocol
==============

The shell protocol is implemented by both the
:doc:`shell service <shell>` and any clients interfacing
with the service.

All messages in the shell protocol are encoded as `CBOR` arrays
and are sent in UDP packets. The first value in the encoded list
is the ``channel_id``. The second value will be the ``command``
and any additional values will be parameters for the ``command``.

    ``{ channel_id, command, parameters.. }``

Spawn Messages
--------------

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

The possible values in the ``options``
argument are as follows:

    - ``args`` - An array of arguments to pass to the child process
    - ``pty`` - A boolean specifying whether a new pty is needed
    - ``env`` - An array of environment variable entries in the form ``"KEY=val"``
    - ``cwd`` - The current working directory of the child process
    - ``uid`` - The uid of the process
    - ``gid`` - The gid of the process
    - ``detached`` - Determines if the child process should be detached from the service

Example of starting a long running shell::

    ``{ 1, 'spawn', 'sh', { detached = true, pty = true, args = { '-l' } } }``

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

Example usages:

Send `SIGTERM` to a child process:

    ``{ channel_id, 'kill' }``

Send `SIGKILL` to a child process:

    ``{ channel_id, 'kill', 9 }``

Resize Terminal
~~~~~~~~~~~~~~~

This message is sent to the shell service to resize the pseudo
terminal of a child process, if one exists. It contains a
channel ID, the string 'resize', the desired number of columns
and the desired number of rows.

    ``{ channel_id, 'resize', columns, rows }``

Example message - Resizing a pseudo terminal to 10x10:

    ``{ 1, 'resize', 10, 10 }``

Process Created
~~~~~~~~~~~~~~~

This message is sent from the shell service when a process
has been created. It contains the channel ID, the string 'pid'
and a number which is the pid.

    ``{ channel_id, 'pid', pid }``

Example message - A process has been created with a pid of 10:

    ``{ 1, 'pid', 10 }``

Stdout Data
~~~~~~~~~~~

This message is sent from the shell service when a process
has produced data via `stdout`. It contains the channel ID,
the string 'stdout', and a string of the stdout data.

    ``{ channel_id, 'stdout', data }``

Example message - ``ls`` producing directory output of `shell-client`:

    ``{ 12, 'stdout', 'deps\ninit.lua\nlibs\nmain.lua\npackage.lua\nREADME.md\ntests\n' }``

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
