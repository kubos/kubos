KubOS Mission Applications Development Guide
============================================

Overview
--------

The things that actually do the stuff. TODO: Make this sentence actually useful

Using the applications framework, users can code their application to have differing mission logic depending on when and how the application is run.

The current unique run levels are:

    - When the system boots
    - When the system boots for the first time (deployment logic)
    - On demand
    
Once an application has been built, it should be transferred to the system, along with its manifest, and then :ref:`registered with the applications service <register-app>`.

APIs
----

User's may write applications in the language of their choice. Something something first-class citizens.

Something something we have provided application APIs for each language to reduce the complexity of code required...

Our supported languages for mission applications are:

.. toctree::
    :maxdepth: 1
    
    Python <python-app-api>
    Rust <rust-app-api>
    
Run Levels
----------

Run levels allow users the option to define differing behaviors depending on when and how their application is started.

Each application should have a definition for each of the available run levels:

    - on-boot
    - on-command
    - TODO: what's the one for deployment? also how to we detect that?

When the application is first called, the run level {will be fetched? why isn't it just a command line arg} will be fetched,
and then the corresponding run level function will be called.

It is acceptable to only have a single set of logic no matter which run level is specified.
In this case, each of the run level options should simply call the common logic function.

On Command
~~~~~~~~~~

The ``on-command`` run level defines logic which should be executed when the application is started manually.

TODO: An example of the kind of logic you'd want to do on-demand rather than automatically

On Boot
~~~~~~~

The ``on-boot`` run level defines logic which should be executed each time the system is started.

This run level is frequently used for setting up continuous fetching and processing of data from the other system services and hardware.
For instance, an application might be set up to fetch the current time from a GPS device and then pass that information through to the ADCS device.

On Deploy
~~~~~~~~~

The ``on-deploy`` run level defines the deployment logic for the system.

This is a special level which will be run when the system is started if and only if this is the first time deployment has been attempted.

TODO: Is it the first time deployment is attempted, or is it if deployment has not yet completed successfully? Also how do we keep track of that? Is it built into the API?
Need waaaay more details on this, since it's crazy mission critical.

.. _app-manifest:

Application Manifest (TODO: What do we actually want to call it?)
-----------------------------------------------------------------

In order for the applications service to properly maintain versioning information, each application should be registered along with a manifest file.

This file should contain the application name, version number, and author information.

For Rust, this is the `Cargo.toml` file which is automatically generated when you create a new project.

TODO: What should users name the file? What fields are allowed and/or must be present?

Example Walkthrough
-------------------

Here's how we go about coding an actual mission application...

PREREQ: Need an example app to actually walk through...