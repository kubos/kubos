First Time Users
================

.. uml::

    @startuml
    left to right direction
    
    actor User
    
    folder "Kubos SDK" {
        [kubos-proj1] as proj1
        [kubos-proj2] as proj2
    }
    
    node "OBC - KubOS Linux" {
        cloud "kubos-proj2" as application
        cloud "App1"
        cloud "App2"
    } 
    
    User -> proj1
    proj2 -> application
    User -> proj2
    
    @enduml

Kubos users develop custom flight software for their OBCs using the Kubos SDK. This software is created and distributed as "Kubos projects".

The output of a Kubos project is a custom application binary which can then be run from the Linux user space.

.. toctree::
    :maxdepth: 1
    
    Installing the Kubos SDK <installation-docs/sdk-installing>
    Creating Your First KubOS Linux Project <os-docs/first-linux-project>