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
    node "OBC - KubOS RT\n\nkubos-proj1" as RTOBC
    
    User -> proj1
    proj1 -> RTOBC
    proj2 -> application
    User -> proj2
    
    @enduml

Kubos users develop custom flight software for their OBCs using the Kubos SDK. This software is created and distributed as "Kubos projects".

When the target OBC is using KubOS RT, the output of a Kubos project is a full system image, containing both the mission software and the underlying operating system.

When the target OBC is using KubOS Linux, the output is a custom application binary which can then be run from the Linux userspace.

.. toctree::
    :maxdepth: 1
    
    Installing the Kubos SDK <sdk-installing>
    KubOS RT Quickstart Tutorial Video <rt-quickstart>
    Creating Your First KubOS RT Project <first-rt-project>
    Creating Your First KubOS Linux Project <first-linux-project>