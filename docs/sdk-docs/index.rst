Kubos SDK Docs
==============

The "Kubos SDK" is a term used to describe all of the components used
to build Kubos projects:

-  Vagrant box - The VM that contains the "ready to run" Kubos development
   environment
-  Kubos CLI - The command-line tool used to create, configure, build
   and debug Kubos projects
-  Kubos source modules - the underlying libraries on which Kubos projects 
   are built

.. uml::

    @startuml
    left to right direction
    
    actor User
    
    node "Kubos SDK" as sdk{
        () "kubos init" as init
        folder "kubos-proj" as proj {
            folder source {
            }
            () "kubos build" as build
            [binary]
        }   
        folder "Kubos Source" as k_source {
        }
    }
    
    () "kubos flash" as flash
    
    node "OBC - KubOS Linux" {
        cloud "kubos-proj" as application
        cloud "App1"
        cloud "App2"
    } 
    
    User -> sdk : vagrant ssh
    init -> proj
    k_source -> build
    build <- source
    [binary] <- build
    [binary] -> flash
    flash -> application
    
    @enduml
   
This documentation section contains the various guides related to using the Kubos SDK:

    - :doc:`../installation-docs/sdk-installing` - How to install the SDK onto your host machine
    - :doc:`sdk-cheatsheet` - A quick guide for the most common SDK tasks
    - :doc:`sdk-reference` - The full list of Kubos CLI commands
    - :doc:`sdk-project-config` - How to configure a Kubos project's settings, dependencies, and underlying hardware information
    - :doc:`windows-dev-environment` - How to develop Kubos projects from a Windows-based host computer
    - :doc:`sdk-upgrading` - How to upgrade to the latest version of the Kubos SDK
    - :doc:`first-linux-project` - How to build your first linux project with the SDK

.. toctree::
    :hidden:
    
    Creating Your First KubOS Linux Project <first-linux-project>
    Installing the Kubos SDK <../installation-docs/sdk-installing>
    Kubos SDK Cheat Sheet <sdk-cheatsheet>
    Kubos CLI Command Reference <sdk-reference>
    Kubos Project Configuration <sdk-project-config>
    Windows Development Environment Example <windows-dev-environment>
    Upgrading Kubos SDK <sdk-upgrading>