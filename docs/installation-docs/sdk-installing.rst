Installing the Kubos SDK
========================

What is the Kubos SDK?
----------------------

The Kubos SDK is a term used to describe all of the components used
to build and run a Kubos project on a target device:

-  Kubos source modules - the individual components of the operating
   systems, hardware abstraction layers, and APIs
-  Kubos CLI - The command-line tool used to create, configure, build
   and debug KubOS projects
-  Vagrant box - A command-line based VM that contains a "ready to run"
   kubos development environment

How Does The SDK Work?
----------------------

The Kubos SDK is distributed through a Vagrant box. A Vagrant box
(referred to simply as a "box") is a command-line based virtual machine.
This virtual machine contains all of the Kubos source code, compiler
toolchains, debugging utilities and miscellaneous tools the Kubos CLI.
The box, when started, is already pre-configured with all of the
required tools for the CLI you will need. This minimizes the set-up
process so you can work on your project rather than setting up tooling.

`Vagrant <https://www.vagrantup.com/>`__ is a command-line based
tool that abstracts the virtualization provider into a simple-to-use
interface. Vagrant supports a variety of providers (VirtualBox, VmWare,
Parallels, etc.) but right now the Kubos SDK only supports VirtualBox.

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
passing usb devices through to your vagrant development environment will not work correctly.

Install Vagrant
~~~~~~~~~~~~~~~

If you don't already have Vagrant installed see the Vagrant
`installation
documentation. <https://www.vagrantup.com/docs/installation>`__

If your vagrant installation is set up correctly, running the following
command should print something similar to the following output:

::

        $ vagrant --version
        Vagrant 2.0.0

Setup
-----

Create your Kubos SDK Vagrant Box:
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

WARNING: THIS WILL OVERWRITE ALL FILES IN YOUR EXISTING BOX
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

::

        $ vagrant box update
        
.. _mount-directory:

Mounting a Host Directory
~~~~~~~~~~~~~~~~~~~~~~~~~

In the context of these documents, as well as virtual machines in
general, the physical "main" computer is referred to as the "host". The
virtual machine inside of the host is referred to as the "guest".

.. Note:: There is not a supported method of this for Windows hosts at
  this time, as Windows does not support Linux symlinks. There is an 
  alternate method for editing files on the SDK listed :doc:`here. <../sdk-docs/windows-dev-environment>`

It is strongly recommended that you create your project in a directory
on your host that is shared with your box when using a Linux or Mac OS
host. By keeping your project on your host it will protect them in the
event your box is destroyed or re-built.

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

Start the Vagrant Box
~~~~~~~~~~~~~~~~~~~~~

To start the box, run:

::

        $ vagrant up

After the box has started you need to "ssh" into the machine to work
with your projects.

::

        $ vagrant ssh

This will start an ssh session in the vagrant box with the Kubos CLI and
all of the required dependencies installed.

That's it! From here see more on:

  - :doc:`Creating your first KubOS RT project <../rt-docs/first-rt-project>`
  - :doc:`Creating your first KubOS Linux project <../linux-docs/first-linux-project>`

After a little bit of usage you may want to look at :doc:`how to upgrade the
Kubos SDK <../sdk-docs/sdk-upgrading>`
