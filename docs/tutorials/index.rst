New User Tutorials
==================

Kubos users develop custom flight software for their OBCs using the :doc:`Kubos SDK <../sdk-docs/index>`.

This custom software is then used to communicate with the various :doc:`core <../services/core-services>` and
:doc:`hardware <../services/hardware-services>` services in order to interact with the system and execute
mission logic.

.. uml::

    @startuml
    left to right direction
    
    actor User
    
    folder "Kubos SDK" {
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

Mission Development Tutorials
-----------------------------

After the completion of these tutorials, the user will have created a mission application capable of querying their hardware for telemetry
data and then storing that data into the telemetry database.

.. toctree::
    :maxdepth: 1
    
    Installing the Kubos SDK <../installation-docs/sdk-installing>
    Creating Your First KubOS Project <first-project>
    
.. todo::

    Creating Your First Mission Application <first-mission-app>
    Communicating with Hardware Services <querying-hardware>
    Storing Telemetry <storing-telemetry>
    
System Interaction Tutorials
----------------------------

These tutorials cover all of the ways a user might directly interact with a system from the ground.

There is no coding required for these actions, however some scripting may used for example purposes.

.. toctree::
    :maxdepth: 1
    
    Registering an Application <app-register>
    
.. todo::
    Querying the System <querying-telemetry>
    Transferring Files <file-transfer>
    Creating a Shell Connection <shell>
    Communicating Over a Radio <comms>
    
Other Resources
---------------

These other docs give more information about various components of KubOS and the Kubos SDK:

    - :doc:`KubOS Architecture Overview <../architecture-overview>`
    - :doc:`Using Rust with the Kubos SDK <../sdk-docs/sdk-rust>`
    - :doc:`Kubos Services <../services/index>`
    - :doc:`Mission Application Development Guide <../app-docs/app-guide>`
    - :doc:`Deployment Application Guide <../app-docs/deployment>`