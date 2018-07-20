KubOS Mission Applications Development Guide
============================================

Overview
--------

In order to be compatible with the applications service, mission applications must comply with the applications framework:

- The application should have separate handler functions for each supported run level
- The application must be packaged with a manifest file which details the name, version, and author information for the binary
    
Once an application has been built, it should be transferred to the system, along with its manifest, and then :ref:`registered with the applications service <register-app>`.

APIs
----

User's may write applications in the language of their choice.
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

The ``OnCommand`` run level defines logic which should be executed when the application is started manually.

For example, a user might want a custom batch of telemetry to be gathered and returned occassionally.
Rather than sending individual telemetry requests, they could code their application to take care of the work,
so then they only have to send a single query in order to trigger the process.

On Boot
~~~~~~~

The ``OnBoot`` run level defines logic which should be executed each time the system is started.

This run level is frequently used for setting up continuous fetching and processing of data from the other system services and hardware.
For instance, an application might be set up to fetch the current time from a GPS device and then pass that information through to the ADCS device.

.. todo::

    On Deploy
    //~~~~~~~~~
    
    The ``on-deploy`` run level defines the deployment logic for the system.
    
    This is a special level which will be run when the system is started if and only if this is the first time deployment has been attempted.
    
    TODO: Is it the first time deployment is attempted, or is it if deployment has not yet completed successfully? Also how do we keep track of that? Is it built into the API?
    Need waaaay more details on this, since it's crazy mission critical.

.. _app-manifest:

Application Manifest
--------------------

In order for the applications service to properly maintain versioning information, each application should be registered along with a manifest file.

This file should contain the application name, version number, and author information.

For Rust, this is the `Cargo.toml` file which is automatically generated when you create a new project.

.. todo::

    For Python... we need to do some stuff...

    TODO: What should users name the file? What fields are allowed and/or must be present?
    
.. todo::

    Example Walkthrough
    //-------------------
    
    Here's how we go about coding an actual mission application...
    
    PREREQ: Need an example app to actually walk through...