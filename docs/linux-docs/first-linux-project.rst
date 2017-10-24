Getting Started with KubOS Linux and the Kubos SDK
==================================================

This is intended to be a quick guide to creating a new KubOS Linux project 
using the Kubos SDK.

Prerequisites
-------------

:doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`

Creating your Project
---------------------

Method 1: Kubos Init
~~~~~~~~~~~~~~~~~~~~

The simplest way to create a new KubOS Linux project is by using the Kubos CLI.
The ``kubos init --linux`` command takes a project name and creates the project
files and folders.

**Note:** Inside of the build system there are several reserved words, which
cannot be used as the name of the project. The most common of these are
``test``, ``source`` and ``include``.

**Note:** Yotta, the build system the Kubos CLI is based upon, requires project
names to be hyphen-delimited or underscore-delimited. CamelCased project names
will cause warnings.

::

        $ kubos init -l myproject

The ``init`` command creates a new directory with the
`kubos-linux-example <https://github.com/kubostech/kubos/tree/master/examples/kubos-linux-example>`__
included so you can get started right away. The ``-l`` or ``--linux`` command
tells the CLI that a KubOS Linux project should be created, rather than a KubOS
RT project.

Method 2: Cloning a Project
~~~~~~~~~~~~~~~~~~~~~~~~~~~

If you would like to copy an existing Kubos project from the internet, you will
need to clone and link the necessary files. For example:

::

        $ git clone https://github.com/kubostech/kubos myproject
        $ cd myproject/examples/kubos-linux-example
        $ kubos link --all

**Note:** It is unnecessary to run the ``kubos init`` command in this case

Editing the project
-------------------

Whether you have cloned your Kubos project or created it with the Kubos CLI, the
default source code entry point is at ``{project directory}/source/main.c``.

There may be additional source files in the ``{project directory}/source``
directory, depending on the specific project that you are working with. Each of
our example applications have a main.c source file as the entry point of the
project.

Choosing a Target
-----------------

Once you have created a project you will need to select a target. The target
defines which hardware your project will run on and how the peripherals are
configured.

You can see a list of available projects by running the following command:

::

        $ kubos target --list

For this example we will set the x86-native-linux target:

::

        $ kubos target x86-linux-native

For more information, see our documentation on :ref:`selecting-a-target`

Building and Flashing
---------------------

Now that the target is set you can begin building. This command will build the
current project:

::

        $ kubos build

You should see the ``Build Succeeded`` message! You are now ready to run your
project:

::

        $ kubos flash
        
Using the `x86-linux-native` target will cause the project to execute within your
Kubos Vagrant image. All output will be routed to your console.

The output should look like this:

::

    Initializing CSP
    Starting example tasks
    Ping result 44 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 8 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 86 [ms]
    Packet received on MY_PORT: Hello World

Press `CTRL+C` to stop execution

Congratulations! You have just created, built, and run a basic Kubos project.

Using Hardware
--------------

If you would like to run this project on a physical board, you'll follow this same process,
except you'll select a different hardware target and the target board will need to be 
connected to your computer and powered before running the ``kubos flash`` command.

More information about the available targets can be found in the :ref:`SDK Cheatsheet <selecting-a-target>`.

.. note::

    If you build a project and then change its target, you will need to rebuild the project
    for the new target with the ``kubos build`` command in order to create a new compatible
    binary to use with ``kubos flash``