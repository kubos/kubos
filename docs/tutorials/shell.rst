Establishing a Shell Connection with an OBC
===========================================

The :doc:`shell service <../ecosystem/services/shell>` is used to provide shell access and commanding from
mission operations or a development environment to the OBC.

Pre-Requisites
--------------

- :doc:`Install the Kubos SDK <../sdk-docs/sdk-installing>` or set up the dependencies
  required for a :doc:`local dev environment <../getting-started/local-setup>`
- Have an OBC available with ethernet capabilities
  (preferably with an :doc:`installation of Kubos Linux <../obc-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- Have the shell service running on a target OBC (this happens by default when running KubOS)
- Windows users: :ref:`Make sure Windows is setup to allow UDP packets from the OBC <windows-udp>`

We'll be using the `shell client <https://github.com/kubos/kubos/tree/master/clients/kubos-shell-client>`__
in order to communicate with the shell service on our OBC, which is automatically included
with the Kubos SDK (as of v1.8.0).

If you are using a local development environment, instead of an instance of the SDK, you'll need to
clone the repo and navigate to the `clients/kubos-shell-client` folder.
You'll then run the program with ``cargo run -- {command args}``.

Syntax
------

The shell client has the following command syntax::

  kubos-shell-client [options] (start | run | list | join | kill)

Required arguments:

    - Operation to perform

        - ``start`` - Start a new shell session
        - ``run`` - Run single remote command
        - ``list`` - List current shell sessions
        - ``join`` - Join an existing shell session
        - ``kill`` - Kill an existing shell session
        - ``help`` - Display the help message

Optional arguments:

    - ``-i {remote IP}`` - Default: `0.0.0.0`. IP address of the shell service to connect to.
    - ``-p {remote port}`` - Default: `8050`. UDP port of the shell service to connect to.

Starting a New Shell Session
----------------------------

We'll start by creating a new shell session between our dev environment and the OBC.

Our command should look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8050 start

Or, from your local dev environment::

    $ cargo run --bin kubos-shell-client -- -i 10.0.2.20 -p 8050 start

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Starting shell session -> 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $

The shell service has spawned an instance of ``/bin/bash`` on the
remote system. Any lines on input given to the shell client will be
sent to the shell service and executed by the ``bash`` instance.

A simple shell session would look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Starting shell session -> 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $ cd /home/kubos
   $ pwd
   /home/kubos
   $ whoami
   kubos

You can enter the ``exit`` command to quit this ``bash`` session,
or you can hit Control-D to detach from the session.

Listing Existing Shell Sessions
-------------------------------

Next we will look at listing the existing shell sessions on the OBC.

Our command should look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8050 list

The output from the client will look like this because we just
started a session in the previous step:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Fetching existing shell sessions:
       672612	{ path = '/bin/bash', pid = 24939 }


The entries in the sessions list are structured like so:

.. code-block:: none

   [channel-id] { path = [process-path], pid = [process-id] }

The channel ID is the unique identifier which can be used with the shell
client's ``join`` and ``kill`` commands.
The process path is the path to the executable running in the session.
The process ID is the PID of the running executable on the remote system.

If no sessions exist, then the output from the client will look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Fetching existing shell sessions:
       No active sessions found

Joining an Existing Shell Session
---------------------------------

If sessions already exist on the OBC then we are able to join them using
the ``join`` command.

The ``join`` command has the following syntax::

   kubos-shell-client join -c <channel_id>

The channel ID should belong to a shell session which was previously started.

To join the session started earlier, our command will look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8050 join -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Joining existing shell session 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $

Killing an Existing Shell Session
---------------------------------

If sessions already exist on the OBC then we are also able to end them
using the ``kill`` command. Shell sessions will not end unless the
process exits or the ``kill`` command is used.

The kill command has the following syntax::

   kubos-shell-client kill -c <channel_id> [-s signal]

The kill command requires a channel ID to know which session to kill.
Optionally, a signal number may also be passed in. If no signal is
specified, then ``SIGKILL`` will be sent.

Our command should look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8050 kill -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8050
   Killing existing shell session -c 672712

Running a Single Remote Command
-------------------------------

Sometimes only a single command needs to be run. In these cases it is
not necessary to start a whole shell session. The run command will
handle starting the shell session, running the remote command,
retrieving the output, and terminating the shell session.

The run command has the following syntax::

   kubos-shell-client run -c "<command>"

The run command requires a command string to know what to run.
This command string must include the base command as well as
any required arguments. The command string **must** be enclosed in `"`s.

A good use case for this command is determining the contents of a directory.
We will look at the contents of the `/home` directory. Our command should
look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8050 run -c "ls /home"

The output from the client should look like this:

.. code-block:: none

   Starting shell client: -> 10.0.2.20:8050
   Running remote command 'ls -l /home'

   kubos
   system