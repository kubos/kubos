Establishing a Shell Connection with an OBC
===========================================

The :doc:`shell service <../services/shell>` is used to provide shell access and commanding from
mission operations or a development environment to the OBC.

Pre-Requisites
--------------

- :doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`
- Have an OBC available with ethernet capabilities
  (preferably with an :doc:`installation of Kubos Linux <../installation-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- Have the shell service running on a target OBC (this happens by default when running KubOS)
- Windows users: :ref:`Make sure Windows is setup to allow UDP packets from the OBC <windows-udp>`

This tutorial is written under the assumption that you are working inside of the
Kubos SDK. The shell client can be easily run from inside of the SDK with
the follow command::

   ``$ kubos-shell-client``

Syntax
------

The shell client has the following command syntax::

  kubos-shell-client  (start | list | join | kill) [options]

Required arguments:

    - Operation to perform

        - ``start`` - Start a new shell session
        - ``list`` - List current shell sessions
        - ``join`` - Join an existing shell session
        - ``kill`` - Kill an existing shell session
        - ``help`` - Display the help message

Optional arguments:

    - ``-i {remote IP}`` - Default: `0.0.0.0`. IP address of the shell service to connect to.
    - ``-p {remote port}`` - Default: `8010`. UDP port of the shell service to connect to.


Starting a New Shell Session
----------------------------

We'll start by creating a new shell session between our SDK instance and the OBC.

Our command should look like this::

   $ kubos-shell-client -i 10.0.2.20 -p 8010 start

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8010
   Starting shell session -> 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $

The shell service has spawned an instance of ``/bin/bash`` on the
remote system. Any lines on input given to the shell client will be
sent to the shell service and executed by the ``bash`` instance.

A simple shell session would look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8010
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

   $ kubos-shell-client -i 10.0.2.20 -p 8010 list

The output from the client will look like this because we just
started a session in the previous step:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8010
   Listing shell sessions
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

   Starting shell client -> 10.0.2.20:8010
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

   $ kubos-shell-client -i 10.0.2.20 -p 8010 join -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8010
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

   $ kubos-shell-client -i 10.0.2.20 -p 8010 kill -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 10.0.2.20:8010
   Killing existing shell session -c 672712
