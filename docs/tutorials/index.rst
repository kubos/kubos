Tutorials
=========

Kubos users develop custom flight software for their OBCs using the KubOS libraries.

This custom software is then used to communicate with the various :doc:`core <../ecosystem/services/core-services>` and
:doc:`hardware <../ecosystem/services/hardware-services>` services in order to interact with the system and execute
mission logic.

.. uml::

    @startuml
    left to right direction

    actor User

    folder "Host Machine" {
        [kubos-proj1] as proj1
        [kubos-proj2] as proj2
    }

    node "OBC - KubOS" {
        frame "App Registry" {
            cloud "kubos-proj2" as application
            cloud "App1"
            cloud "App2"
        }

        rectangle "Process Monitor Service" as monitor
        rectangle "Telemetry Database Service" as telemdb
        rectangle "Hardware Service #1" as hw1
    }

    User -> proj1
    proj2 -> application
    User -> proj2
    application ..> monitor
    application ..> telemdb
    application ..> hw1

    @enduml



This series of tutorials will walk a new user through the major steps required to program a mission and then communicate with
a system once it is in-flight.

.. note::

    You should be able to go through these tutorials from either a :doc:`local dev environment <../getting-started/local-setup>`
    or an instance of the :doc:`Kubos SDK <../sdk-docs/sdk-installing>`

.. _mission-development-tutorials:

Mission Development Tutorials
-----------------------------

After the completion of these tutorials, the user will have created a mission application capable of querying their hardware for telemetry
data and then storing that data into the telemetry database.

.. toctree::
    :maxdepth: 1


    Creating Your First Mission Application <first-mission-app>
    Running a KubOS Project on an OBC <first-obc-project>

.. _system-interaction-tutorials:

System Interaction Tutorials
----------------------------

These tutorials cover all of the ways a user might directly interact with a system from the ground.

There is no coding required for these actions, however some scripting may used for example purposes.

The first two tutorials may be run entirely locally.
The file and shell service tutorials are written to be used with an OBC.

.. toctree::
    :maxdepth: 1

    Registering an Application <app-register>
    Scheduling an Application <schedule-app>
    Querying the System <querying-telemetry>
    Transferring Files <file-transfer>
    Creating a Shell Connection <shell>

.. _advanced-tutorials:

Advanced Tutorials
------------------

Important system tutorials which should be approached after mastering basic system interaction.

.. toctree::
    :maxdepth: 1

    Creating Your Communications Service <comms-service>

Other Resources
---------------

These docs give more information about various components of KubOS:

    - :doc:`KubOS Design <../kubos-design>` - The high-level design of KubOS
    - :doc:`../getting-started/using-rust` - Instructions for using Rust with KubOS
    - :doc:`Application Dev Guides <../ecosystem/apps/app-guide>` - More in-depth instructions about
      creating mission applications
    - :ref:`Kubos Services <service-docs>` - Docs covering what services are,
      which services are availabe, and how to create your own services
    - :doc:`Mission Development Guide <../mission-dev/index>` - Docs covering the recommended
      components which should be developed for any given mission
