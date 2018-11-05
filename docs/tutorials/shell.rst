Establishing a Shell Connection with an OBC
===========================================

The :doc:`shell service <../services/shell>` is used to provide shell access and commanding from
mission operations to the OBC.

Pre-Requisites
--------------

- :doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`
- Have an OBC available with ethernet capabilities
  (preferably with an :doc:`installation of Kubos Linux <../installation-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- Have the file transfer service running on a target OBC (this happens by default when running KubOS)
- Windows users: :ref:`Make sure Windows is setup to allow UDP packets from the OBC <windows-udp>`

Syntax
------

The shell client can be run from inside of the Kubos SDK with the following command::

    ``$ kubos-shell-client``

The shell client has the following command syntax::

    (start | list | join | kill) [options]

Required arguments:

    - Operation to perform

        - ``start`` - Start a new shell session
        - ``list`` - List current shell sessions
        - ``join`` - Join existing shell session
        - ``kill`` - Kill existing shell session

Optional arguments:

    - ``-i {remote IP}`` - Default: `0.0.0.0`. IP address of the shell service to connect to.
    - ``-p {remote port}`` - Default: `6000`. UDP port of the shell service to connect to.


Starting a New Shell Session
----------------------------

We'll start by creating a new shell session on the OBC.

Our command should look like this::

   $ kubos-shell-client start

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Starting shell session -> 672612
   Press enter to send input to the shell session.
   Press Control-D to detach from the session.
   $ 

Listing Existing Shell Sessions
-------------------------------

Our command should look like this::

   $ kubos-shell-client list

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Listing shell sessions
   672612	{ path = '/bin/bash', pid = 24939 }

Joining an Existing Shell Session
---------------------------------

Our command should look like this::

   $ kubos-shell-client join -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Joining existing shell session 672612

Killing Existing Shell Session
------------------------------

Our command should look like this::

   $ kubos-shell-client kill -c 672612

The output from the client should look like this:

.. code-block:: none

   Starting shell client -> 0.0.0.0:6000
   Killing existing shell session -c 672712
