Installing the Kubos SDK
========================

Prerequisites
-------------

System Requirements
~~~~~~~~~~~~~~~~~~~

The Kubos SDK has several hardware requirements, including:

-  64-bit processor with AMD-V or Intel VT-x virtualization support
-  Mac OS (10.9 +), Windows 7 SP1 (or more recent), or a mainstream
   Linux distribution (see the `full
   list <https://www.virtualbox.org/manual/ch01.html#hostossupport>`__
   of supported host OSes)
-  8 GB RAM
-  10 GB of free HDD space

.. _powershell:

Install Windows PowerShell v3+ (Windows 7 SP1 Only)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

If you are running Windows 7 SP1, you **must** upgrade your version of
Windows PowerShell to atleast v3.0.

You can verify your current version by opening Windows Command Prompt
and running the following commands::

    $>powershell
    Windows PowerShell
    Copyright (C) 2016 Microsoft Corporation. All rights reserved.
    
    PS $> $PSVersionTable.PSVersion
    
    Major  Minor  Build  Revision
    -----  -----  -----  --------
    2      0      -1     -1
  
    
    PS $> exit

The ``Major`` field should have a value of atleast ``3``.

`Click here for instructions about upgrading PowerShell <https://docs.microsoft.com/en-us/powershell/scripting/setup/installing-windows-powershell?view=powershell-5.1>`__

Install VirtualBox
~~~~~~~~~~~~~~~~~~

Vagrant requires a virtualization "provider". Currently the only
provider that Kubos officially supports is VirtualBox.

-  `Download VirtualBox <https://www.virtualbox.org/wiki/Downloads>`__

-  `Download <https://www.virtualbox.org/wiki/Downloads>`__ the
   VirtualBox Extension Pack to enable passing USB devices into a
   virtual machine.

If you're using Linux as your host operating system you will need to add
yourself to the ``vboxusers`` group with the following command:

::

        sudo usermod -aG vboxusers <username>

You will need to logout and log back in to your host computer, otherwise
passing USB devices through to your development environment will not work correctly.

Install Vagrant
~~~~~~~~~~~~~~~

If you don't already have Vagrant installed see the Vagrant
`installation
documentation. <https://www.vagrantup.com/docs/installation>`__

If your Vagrant installation is set up correctly, running the following
command should print something similar to the following output:

::

        $ vagrant --version
        Vagrant 2.0.0

Setup
-----

Create your Kubos SDK Instance:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To create an instance of the SDK box follow these steps:

::

       $ vagrant init kubos/kubos-dev
       $ vagrant up

This will create a Vagrantfile in your current directory. Vagrantfiles
are important as they contain the configuration details for specific
boxes. Additionally, Vagrant environments are dependant on the directory
they were created in. To interact with this box in the future you will
need to navigate back to this directory.

If the output of ``vagrant up`` mentions there's a new version of the
kubos-dev box available you can upgrade your box with the following
command:

.. warning:: THIS WILL OVERWRITE ALL FILES IN YOUR EXISTING BOX

::

        $ vagrant box update
        
.. _mount-directory:

Mounting a Host Directory
~~~~~~~~~~~~~~~~~~~~~~~~~

In the context of these documents, as well as virtual machines in
general, the physical "main" computer is referred to as the "host". The
virtual machine inside of the host is referred to as the "guest".

It is strongly recommended that you create your project in a directory
on your host that is shared with your box.
By keeping your project on your host it will protect them in the event
your box is destroyed or re-built.

.. note::
    
    Windows does not support Linux symlinks. If, for some reason, you need to create symlinks in
    your project, you will need to do so in a directory which lives entirely within the VM.
    
    KubOS does not currently leverage any symlinks, so this should not be an issue for the average
    developer's workflow.

To mount a specific directory from your host, open the Vagrantfile
located in the directory from the previous step and look for the
following lines:

::

        # Share an additional folder to the guest VM. The first argument is
        # the path on the host to the actual folder. The second argument is
        # the path on the guest to mount the folder. And the optional third
        # argument is a set of non-required options.
        # config.vm.synced_folder "../data", "/vagrant_data"

.. Note::
  The default home directory in the Kubos Vagrant boxes is ``/home/vagrant``

Uncomment the last line in this block and change the paths to match your
host directory and a desired mount point in the box.

.. Note::
  The path in the box must be an absolute path

After a volume is mounted into the box all of the data from the host
path will be available at the path specified for the box. In the above
example the host path (``../data``) would be exposed at
``/vagrant_data`` inside of the box. This allows you to use the text
editor of your choosing to edit the project files from your host machine
at the host directory path.

.. Note::
  If you make changes to the Vagrantfile after the box has been
  started you will need to run ``vagrant reload`` for these changes to
  take effect in the box.

--------------

For more information on mounting volumes see the following `guide <https://www.vagrantup.com/docs/synced-folders/basic_usage.html>`__

--------------

.. _sdk-port-forward:

Exposing Network Ports
~~~~~~~~~~~~~~~~~~~~~~

If you would like to interact with Kubos services running inside an SDK instance from your host
environment, you will need to update your Vagrantfile to expose either a single port, or your
entire SDK as with a private network address.

We recommend that you set up a `private network <https://www.vagrantup.com/docs/networking/private_network.html>`__
connection, since you may want to interact with multiple different network ports while developing
with KubOS.

To do so, enable the following line in your Vagrantfile::

    config.vm.network "private_network", ip: "192.168.33.10"

Start the Vagrant Box
~~~~~~~~~~~~~~~~~~~~~

To start the box, run:

::

        $ vagrant up

After the box has started you need to "ssh" into the machine to work
with your projects.

::

        $ vagrant ssh

This will start an SSH session in the Vagrant box with the Kubos CLI and
all of the required dependencies installed.

That's it! From here see more on:

  - :doc:`Creating your first KubOS project <../tutorials/first-mission-app>`

After a little bit of usage you may want to look at :doc:`how to upgrade the
Kubos SDK <../sdk-docs/sdk-upgrading>`
