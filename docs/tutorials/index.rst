New User Tutorials
==================

TODO: Re-evaluate this graph

.. uml::

    @startuml
    left to right direction
    
    actor User
    
    folder "Kubos SDK" {
        [kubos-proj1] as proj1
        [kubos-proj2] as proj2
    }
    
    node "OBC - Kubos Linux" {
        cloud "kubos-proj2" as application
        cloud "App1"
        cloud "App2"
    } 
    
    User -> proj1
    proj2 -> application
    User -> proj2
    
    @enduml

Kubos users develop custom flight software for their OBCs using the Kubos SDK.

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
    Creating Your First Mission Application <first-mission-app>
    Communicating with Hardware Services <querying-hardware>
    Storing Telemetry <storing-telemetry>
    
System Interation Tutorials
---------------------------

These tutorials cover all of the ways a user might directly interact with a system from the ground.

There is no coding required for these actions, however some scripting may used for example purposes.

.. toctree::
    :maxdepth: 1
    
    Registering an Application <app-register>
    Querying the System <querying-telemetry>
    Transferring Files <file-transfer>
    Creating a Shell Connection <shell>
    Communicating Over a Radio <comms>
    
Other Resources
---------------

These other docs give a more information about various components of KubOS

TODO: Links to other docs