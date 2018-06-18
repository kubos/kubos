KubOS Mission Applications Development Guide
============================================

Overview
--------

The things that actually do the stuff.

Using the applications framework, users can code their application to have differing mission logic depending on when and how the application is run.

The current unique run levels are:

    - When the system boots
    - When the system boots for the first time (deployment logic)
    - On demand

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

On Command
~~~~~~~~~~

On Boot
~~~~~~~

On Deploy
~~~~~~~~~

Example Walkthrough
-------------------

Here's how we go about coding an actual mission application...

PREREQ: Need an example app to actually walk through...