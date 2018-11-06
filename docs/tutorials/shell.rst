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

- Have the file transfer service running on a target OBC (this happens by default when running KubOS)
- Windows users: :ref:`Make sure Windows is setup to allow UDP packets from the OBC <windows-udp>`

Setup
-----

This tutorial is written under the assumption that you are working inside of the
Kubos SDK. The shell client can be easily run from inside of the SDK with
the follow command::

   ``$ kubos-shell-client``

Syntax
------

The shell client has the following command syntax::

    (start | list | join | kill) [options]

Required arguments:

    - Operation to perform

        - ``start`` - Start a new shell session
        - ``list`` - List current shell sessions
        - ``join`` - Join an existing shell session
        - ``kill`` - Kill an existing shell session

Optional arguments:

    - ``-i {remote IP}`` - Default: `0.0.0.0`. IP address of the shell service to connect to.
    - ``-p {remote port}`` - Default: `8080`. UDP port of the shell service to connect to.


Starting a New Shell Session
----------------------------

We'll start by creating a new shell session on the OBC.

Our command should look like this::

   $ kubos-shell-client start

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:8080
   Starting shell session -> 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $

The shell service has spawned an instance of ``/bin/bash`` on the
remote system. Any lines on input given to the shell client will be
sent to the shell service and executed by the ``bash`` instance.

You can send ``exit`` to quit this ``bash`` session, or you can
hit Control-D to detach from the session.

Listing Existing Shell Sessions
-------------------------------

Next we will look at listing the existing shell sessions on the OBC.

Our command should look like this::

   $ kubos-shell-client list

The output from the client should look like this if sessions exist:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Listing shell sessions
       672612	{ path = '/bin/bash', pid = 24939 }


The entries in the sessions list are structured like so:

.. code-block:: none

   [channel-id] { path = [process-path], pid = [process-id] }

The channel id can be used to join or kill the process.
The process path is the path to the executable running in the session.
The pid is the pid of the process on the remote system.

If no sessions exist then the output from the client will look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Listing shell sessions
       No active sessions found

Joining an Existing Shell Session
---------------------------------

If sessions already exist on the OBC then we are able to join them using
the join command.

The join command has the following synatx::

   kubos-shell-client join -c <channel_id>

The channel id must be that of a session that already exists.

To join the session started earlier our command will look like this::

   $ kubos-shell-client join -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Joining existing shell session 672612
   Press enter to send input to the shell session
   Press Control-D to detach from the session
   $

Killing an Existing Shell Session
---------------------------------

We are also able to kill existing sessions on the OBC.

The kill command has the following syntax::

   kubos-shell-client kill [-s signal] -c <channel_id>

The kill command requires a channel_id to know which session to kill.
Optionally a signal number may also be passed in. If no signal is
specified then ``SIGKILL`` will be sent.

Our command should look like this::

   $ kubos-shell-client kill -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Killing existing shell session -c 672712
