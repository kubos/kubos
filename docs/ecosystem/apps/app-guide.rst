KubOS Mission Applications Development Guide
============================================

Overview
--------

In order to be compatible with the applications service, mission applications must comply with the applications framework:

- The application should provide a ``-h`` command option for query the command line arguments
- The application should not define a ``-c`` command option (reserved for configuration setup)
- The application must be packaged with a manifest file which details the name, version, and author information for the binary

Once an application has been built, it should be transferred to the system, along with its manifest, and then :ref:`registered with the applications service <register-app>`.

For projects written in languages like Python, where there is no executable file, multiple files
may be used for the application.

APIs
----

Users may write applications in the language of their choice.
However, Kubos provides APIs to assist and simplify with application development for use with one of our preferred languages.

Our supported languages for mission applications are:

.. toctree::
    :maxdepth: 1
    
    Python <python-app-api>
    Rust <rust-app-api>
    
These APIs abstract initial logging setup and provide helper functions for use when querying other system and hardware services.

Additional Arguments
--------------------

Additional command line arguments may be used by the application. They will be automatically passed through to the application by the applications service.

Under the covers, the application would be called like so::

    mission-app --verbose --release
    
Where ``--verbose`` and ``--release`` are custom arguments for that particular application.

Examples of how to define and structure additional arguments can be seen in the `Rust <https://github.com/kubos/kubos/blob/master/examples/rust-mission-app/src/main.rs>`__
and `Python <https://github.com/kubos/kubos/blob/master/examples/python-mission-application/mission-app.py>`__
example applications.

.. note:: The ``-c`` option is pre-defined by the system to allow a custom config file to be used.

.. _app-manifest:

Application Manifest
--------------------

In order for the applications service to properly maintain versioning information, each application should be registered along with a manifest file, `manifest.toml`.

This file must have the following key values:

- ``name`` - The name of the application which will be called for execution
- ``version`` - The version number of the application
- ``author`` - The author of the application

Optionally, the file may also specify the following values:

The ``executable`` key value allows you to specify which file should be called in order to begin
execution of the application.
If it's omitted, the value of ``name`` will be used.
This is particularly useful for Python applications, where the name of the application might not
match the name of the file to be called.

The ``config`` key value allows you to specify a custom file which the application should use in
order to read :doc:`service configuration <../services/service-config>` information.
If it is omitted, the default location ``/etc/kubos-config.toml`` will be used.

For example::

    name = "mission-app"
    executable = "app.py"
    version = "1.1"
    author = "Me"
    config = "/custom/config.toml"
    
Local Execution
---------------

If you would like to test your application locally and if it will communicate with any
:doc`locally running services <../../getting-started/local-services>`, then you will need to
include the ``-c path/to/config.toml`` argument when starting your application.

For example::

    $ ./app.py -c ../tools/default_config.toml


Additional Resources
--------------------

Reference Docs:

    - :doc:`Applications Service Guide <../services/app-service>` - Goes over the basic capabilities
      of the applications service and how it can be customized
    - :doc:`Deployment Application Guide <../../mission-dev/deployment>` - Goes over the basic
      architecture required to execute a mission's deployment logic

Tutorials:

    - :doc:`../../tutorials/first-mission-app` - Walks the user through the process of creating their
      first mission application which is capable of interacting with Kubos services
    - :doc:`../../tutorials/app-register` - Walks the user through the process of registering a
      mission application with the applications service and then starting, updating, and verifying
      the application

Example applications:

    - `Basic application written in Rust <https://github.com/kubos/kubos/tree/master/examples/rust-mission-app>`__ -
      Demonstrates the basic application framework and how passthrough arguments can be used
    - `Framework application written in Python <https://github.com/kubos/kubos/blob/master/examples/python-mission-framework/python-mission-app.py>`__ -
      Can be used as a starting template when creating Python applications
    - `Basic application wrtting in Python <https://github.com/kubos/kubos/tree/master/examples/python-mission-application>`__ -
      Demonstrates the basic application framework and how to communicate with Kubos services
