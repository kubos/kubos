Transferring Files to an OBC
============================

Once a satellite is in orbit, the :doc:`file transfer service <../ecosystem/services/file>` can be used to
transfer files both to and from the ground.

Pre-Requisites
--------------

- :doc:`Install the Kubos SDK <../sdk-docs/sdk-installing>` or set up the dependencies
  required for a :doc:`local dev environment <../getting-started/local-setup>`
- Have an OBC available with ethernet capabilities
  (preferably with an :doc:`installation of Kubos Linux <../obc-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- Have the file transfer service running on a target OBC (this happens by default when running KubOS)
- Windows users: :ref:`Make sure Windows is setup to allow UDP packets from the OBC <windows-udp>`

We'll be using the `file transfer client <https://github.com/kubos/kubos/tree/master/clients/kubos-file-client>`__
in order to communicate with the file transfer service on our OBC, which is automatically included
with the Kubos SDK (as of v1.8.0).

If you are using a local development environment, instead of an instance of the SDK, you'll need to
clone the repo and navigate to the `clients/kubos-file-client` folder.
You'll then run the program with ``cargo run -- {command args}``.

Syntax
------

The file transfer client has the following command syntax::

    kubos-file-client [options] (upload | download | cleanup) source-file [target-file]
    
Required arguments:

    - Operation to perform

        - ``upload`` - Transfer ``source-file`` on the local host to ``target-file`` location
          on the remote target
        - ``download`` - Transfer ``source-file`` on the remote target to ``target-file`` location
          on the local host
        - ``cleanup`` - Cleanup the endpoint service's temporary storage directory

    - ``source-file`` - The file to be transferred. May be a relative or absolute path.

Optional arguments:

    - ``target-file`` - Final destination path for the transferred file.
      If not specified, the root file name from ``source-file`` will be used and the file will be
      placed in the current directory of the destination.
    - ``-h {host IP}`` - Default: `0.0.0.0`. IP address of the local host to use.
    - ``-r {remote IP}`` - Default: `0.0.0.0`. IP address of the file transfer service to connect to.
    - ``-p {remote port}`` - Default: `8040`. UDP port of the file transfer service to connect to.
    - ``-P {host_port}`` - Default: `8080`. The UDP port that the file transfer service will send responses to.
    - ``-s {storage_prefix}`` - Default: `file-storage`. Name of the directory which should be used
      for temporary file transfer storage.
    - ``-c {transfer_chunk_size}`` - Default: `1024`. Size, in bytes, of the individual chunks the 
      file should be broken into before transfer.
    - ``-t {hold_count}`` - Default: `6`. The number of times the client should fail to receive data
      from the endpoint service before giving up and exiting.
    - ``-d {inter_chunk_delay}`` - Default: `1`. The delay in milliseconds between
      each chunk transmission.
    - ``-m {max_chunks_transmit}`` - Default: None. The maximum number of chunks to transmit 
      before waiting for a response. The default is to transmit the whole file.
    - ``--hash_chunk_size`` - Default: `2048`: The chunk size, in bytes, to be used when 
      generating the file's hash.

Sending a File to an OBC
------------------------

We'll start by transferring a file to our OBC.
For this tutorial, we'll be transferring the application file that was created as part of the
:doc:`mission application on an OBC <first-obc-project>` tutorial to the ``kubos`` user's home directory on the
OBC.

We'll need to specify the OBC's IP address and the port that the file transfer service is listening
on. By default, this is port 8040.

We will also need to specify the port that the file transfer service will be sending data
back on. By default, the file transfer service sends responses to port 8080.

Our transfer command should look like this::

    $ kubos-file-client -r 10.0.2.20 -p 8040 -P 8080 upload /home/vagrant/my-app/my-mission-app.py /home/kubos/my-mission-app.py
    
Or, from your local dev environment::

    $ cargo run -- -r 10.0.2.20 -p 8040 -P 8080 upload /home/vagrant/my-app/my-mission-app.py /home/kubos/my-mission-app.py
    
The output from the client should look like this:

.. code-block:: none

    16:55:56 [INFO] Starting file transfer client
    16:55:56 [INFO] Uploading local:/home/vagrant/new-user/my-mission-app.py to remote:/home/kubos/my-mission-app.py
    16:55:56 [INFO] -> { 768720, 62c3491309b0bf9af5b367bea18471b8, 1 }
    16:55:56 [INFO] -> { 768720, export, 62c3491309b0bf9af5b367bea18471b8, /home/kubos/my-mission-app.py, 33277 }
    16:55:56 [INFO] <- { 768720, 62c3491309b0bf9af5b367bea18471b8, false, [(0, 1)] }
    16:55:56 [INFO] -> { 768720, 62c3491309b0bf9af5b367bea18471b8, 0, chunk_data }
    16:55:58 [INFO] <- { 62c3491309b0bf9af5b367bea18471b8, true }
    16:55:58 [INFO] <- { 768720, true }
    16:55:58 [INFO] Operation successful

The file transfer service maintains a temporary storage directory with the data from transferred files.
As a result, if you run the upload command again, you should see a slightly truncated output:

.. code-block:: none

    16:15:08 [INFO] Starting file transfer client
    16:15:08 [INFO] Uploading local:/home/vagrant/new-user/my-mission-app.py to remote:/home/kubos/my-mission-app.py
    16:15:08 [INFO] -> { 184278, 62c3491309b0bf9af5b367bea18471b8, 1 }
    16:15:08 [INFO] -> { 184278, export, 62c3491309b0bf9af5b367bea18471b8, /home/kubos/my-mission-app.py, 33277 }
    16:15:08 [INFO] <- { 62c3491309b0bf9af5b367bea18471b8, true }
    16:15:08 [INFO] <- { 184278, true }
    16:15:08 [INFO] Operation successful

Receiving a File from an OBC
----------------------------

Next, we'll request that the OBC send us the application debug log file::

    $ kubos-file-client -r 10.0.2.20 -p 8040 -P 8081 download /var/log/app-debug.log
    
We're not specifying a destination file, which will result in the transferred file being saved as
`app-debug.log` in our current directory.

The output from the client should look like this:

.. code-block:: none

    17:56:27 [INFO] Starting file transfer client
    17:56:27 [INFO] Downloading remote: /var/log/app-debug.log to local: app-debug.log
    17:56:27 [INFO] -> { import, /var/log/app-debug.log }
    17:56:27 [INFO] <- { 796611, true, 1a564e8da7b83c2d6a2a44d447855f6d, 1, 33188 }
    17:56:27 [INFO] -> { 796611, 1a564e8da7b83c2d6a2a44d447855f6d, false, [0, 1] }
    17:56:27 [INFO] <- { 796611, 1a564e8da7b83c2d6a2a44d447855f6d, 0, chunk_data }
    17:56:29 [INFO] -> { 796611, 1a564e8da7b83c2d6a2a44d447855f6d, true, None }
    17:56:29 [INFO] -> { 796611, true }
    17:56:29 [INFO] Operation successful

We can then check the contents of the transferred file::

    $ cat /var/log/app-debug.log
    1970-01-01T03:23:13.246358+00:00 Kubos my-mission-app:<info> Current available memory: 497060 kB
    1970-01-01T03:23:13.867534+00:00 Kubos my-mission-app:<info> Telemetry insert completed successfully
