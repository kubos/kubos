Getting Started with KubOS RT and the Kubos SDK
===============================================

This is intended to be a quick guide to creating a new KubOS RT project
using the Kubos SDK.

Prerequisites
-------------

:doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`

Creating your Project
---------------------

Method 1: Kubos Init
~~~~~~~~~~~~~~~~~~~~

The simplest way to create a new KubOS RT project is by using the Kubos
CLI. The ``kubos init`` command takes a project name and creates the
project files and folders.

**Note:** Inside of the build system there are several reserved words,
which cannot be used as the name of the project. The most common of
these are ``test``, ``source`` and ``include``.

**Note:** Yotta, the build system the Kubos CLI is based upon, requires
project names to be hyphen-delimited or underscore-delimited. CamelCased
project names will cause warnings.

::

        $ kubos init myproject

The ``init`` command creates a new directory with the
`kubos-rt-example <https://github.com/kubostech/kubos/tree/master/kubos-rt-example>`__
included so you can get started right away.

Method 2: Cloning a Project
~~~~~~~~~~~~~~~~~~~~~~~~~~~

We have also created several different example Kubos projects which can
be used as starting points.

-  `Example showing basic FreeRTOS tasks and
   CSP <https://github.com/kubostech/kubos/tree/master/examples/kubos-rt-example>`__
-  `Example showing the I2C HAL and
   sensors <https://github.com/kubostech/kubos/tree/master/examples/kubos-i2c-example>`__
-  `Example showing the SPI HAL and
   sensors <https://github.com/kubostech/kubos/tree/master/examples/kubos-spi-example>`__
-  `Example showing the sensor
   interface <https://github.com/kubostech/kubos/tree/master/examples/kubos-sensor-example>`__
-  `Example showing CSP over
   UART <https://github.com/kubostech/kubos/tree/master/examples/kubos-csp-example>`__

If you would like to use one of our projects, you will need to clone the main repo and
then link the necessary files. For example:

::

        $ git clone https://github.com/kubostech/kubos myproject
        $ cd myproject/examples/kubos-spi-example
        $ kubos link --all

**Note:** It is unnecessary to run the ``kubos init`` command in this
case

Editing the project
-------------------

Whether you have cloned your Kubos project or created it with the
kubos-cli, the default source code entry point is at
``{project directory}/source/main.c``.

There may be additional source files in the
``{project directory}/source`` directory, depending on the specific
project that you are working with. Each of our example applications have
a main.c source file as the entry point of the project.

Choosing a Target
-----------------

Once you have created a project you will need to select a target. The
target defines which hardware your project will run on and how the
peripherals are configured.

You can see a list of available projects by running the following
command:

::

        $ kubos target --list

For this example we will set the msp430f5529 target:

::

        $ kubos target msp430f5529-gcc

For more information, see our documentation on :ref:`selecting-a-target`

Building and Flashing
---------------------

Now that the target is set you can begin building. This command will
build the current project:

::

        $ kubos build

You should see the ``Build Succeeded`` message! You are now ready to
load your software on some hardware. Connect your hardware to your
computer and run the following flash command:

::

        $ kubos flash

Congratulations! You have just created a basic Kubos project, built it
and (hopefully) flashed it onto some hardware.
