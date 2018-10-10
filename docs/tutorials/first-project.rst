Getting Started with KubOS and the Kubos SDK
============================================

This tutorial guides the user through the process of creating a new Kubos project using Python2.7
and installing and running it on a target OBC.

.. note:: 

    The iOBC does not support Python. If this is the board which you are using,
    please refer to the :doc:`Using Rust with the Kubos SDK <../sdk-docs/sdk-rust>`
    doc to get your first project built and running on the OBC.

Prerequisites
-------------

- :doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`
- Have an OBC available with both Python and SSH capabilities
  (preferably with an :doc:`installation of Kubos Linux <../installation-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

Creating your Project
---------------------

Like all good programming tutorials, we'll start with a basic "Hello, World" program.

Log into your SDK VM and then create a new folder for your project.

Within that folder, create a new file, ``hello_world.py``, which will print "Hello, World!"
and then exit.

.. code-block:: python

    print "Hello, World!"

Running the Project Locally
---------------------------

The Kubos SDK should have all resources required to run the project locally.

Our example project can be run like so::

    $ python hello_world.py
    Hello, World!
    
Logging in to KubOS
-------------------

By default, KubOS comes with a user account, ``kubos``, with the default password ``Kubos123``.

Log into your OBC using SSH and its configured IP address. Enter the password when prompted.

For example::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********

If this is your first time connecting to the board via SSH, you may be prompted to confirm
the target IP's authenticity. Enter "yes" if this occurs::

    $ ssh root@10.0.2.20
    The authenticity of host '10.0.2.20 (10.0.2.20)' can't be established.
    ECDSA key fingerprint is SHA256:ir2TC+iML+MJ5Cb3cxTReWI69aX6EtPysFQzWleKc+8.
    Are you sure you want to continue connecting (yes/no)? yes
    Warning: Permanently added '10.0.2.20' (ECDSA) to the list of known hosts.
    kubos@10.0.2.20's password: ********

Please confirm that you are able to connect to the board via SSH from the SDK before proceeding
with the next step. If you are unable to do so, please verify that your OBC's network connection
has been :ref:`successfully configured and activated <ethernet>`.

Once you are logged in to the OBC, you can use the ``exit`` command to end the SSH connection and
return to the SDK.

Transferring the Project to a Target OBC
----------------------------------------

We can now transfer the project to the ``kubos`` user home directory on the target OBC using SCP.
From the SDK's command line, run the following (be sure to replace ``10.0.2.20`` with your OBC's
IP address)::

    $ scp hello_world.py kubos@10.0.2.20:/home/kubos
    kubos@10.0.2.20's password: ********
    hello_world.py                                       100%   21     0.0KB/s   00:00
    
Running the Project on the Target OBC
-------------------------------------

Once the project has been transferred, we can log in to the OBC and run it::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # python hello_world.py
    Hello, World!

.. todo::
    
    Next Steps
    <add section header>
    
    In the :doc:`next tutorial <first-mission-app>`, we'll create and run our first mission application.