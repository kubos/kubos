KubOS Mission Applications Development Guide
============================================

Overview
--------

In order to be compatible with the applications service, mission applications must comply with the applications framework:

- The application should have separate handler functions for each supported run level
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
    
These APIs abstract the run level definitions and provide helper functions for use when querying other system and hardware services.

Run Levels
----------

Run levels allow users the option to define differing behaviors depending on when and how their application is started.

Each application should have a definition for each of the available run levels:

    - OnBoot
    - OnCommand

When the application is first called, the run level will be fetched,
and then the corresponding run level function will be called.

It is acceptable to only have a single set of logic no matter which run level is specified.
In this case, each of the run level options should simply call the common logic function.

On Command
~~~~~~~~~~

The ``OnCommand`` run level defines logic which should be executed when the :ref:`application is started manually <start-app>`.

For example, a user might want a custom batch of telemetry to be gathered and returned occassionally.
Rather than sending individual telemetry requests, they could code their application to take care of the work,
so then they only have to send a single query in order to trigger the process.

On Boot
~~~~~~~

The ``OnBoot`` run level defines logic which should be executed when the applications service is started at system boot time.

This run level is frequently used for setting up continuous fetching and processing of data from the other system services and hardware.
For instance, an application might be set up to fetch the current time from a GPS device and then pass that information through to the ADCS device.

.. todo::

    On Deploy
    //~~~~~~~~~
    
    The ``on-deploy`` run level defines the deployment logic for the system.
    
    This is a special level which will be run when the system is started if deployment has not completed successfully yet.
    
    Two U-Boot variables help keep track of this: `deployed` - boolean indicating whether deployment has been completed or not,
    and `deploy_start` - a timestamp that's generated the first time deployment is started. This is used to keep track of the
    delay required between initial launch and when deployment is allowed to begin.
    
Additional Arguments
--------------------

Additional command line arguments may be used by the application. They will be automatically passed through to the application by the applications service.

.. _app-manifest:

Application Manifest
--------------------

In order for the applications service to properly maintain versioning information, each application should be registered along with a manifest file, `manifest.toml`.

This file must have the following key values:

- ``name`` - The name of the application which will be called for execution
- ``version`` - The version number of the application
- ``author`` - The author of the application

For example::

    name = "mission-app"
    version = "1.1"
    author = "Me"

.. note::

    The ``name`` parameter must exactly match the name of the file which should be called for
    execution

Additional Resources
--------------------

Reference Docs:

    - :doc:`Applications Service Guide <app-service>` - Goes over the basic capabilities of the
      applications service and how it can be customized
    - :doc:`Deployment Application Guide <deployment>` - Goes over the basic architecture required
      to execute a mission's deployment logic

Tutorials:

    - :doc:`../tutorials/first-mission-app` - Walks the user through the process of creating their
      first mission application which is capable of interacting with Kubos services
    - :doc:`../tutorials/app-register` - Walks the user through the process of registering a
      mission application with the applications service and then starting, updating, and verifying
      the application

Example applications:

    - `Basic application written in Rust <https://github.com/kubos/kubos/tree/master/examples/rust-mission-app>`__ -
      Demonstrates the basic application framework and how passthrough arguments can be used
    - `Framework application written in Python <https://github.com/kubos/kubos/blob/master/examples/python-mission-app/python-mission-app.py>`__ -
      Can be used as a starting template when creating Python applications
    - `Basic application wrtting in Python <https://github.com/kubos/kubos/tree/master/examples/python-mission-application>`__ -
      Demonstrates the basic application framework and how to communicate with Kubos services