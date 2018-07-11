Kubos SDK Docs
==============

The "Kubos SDK" is a term used to describe all of the components used
to build Kubos projects:

-  Vagrant box - The VM that contains the "ready to run" Kubos development
   environment
-  Kubos CLI - The command-line tool used to create, configure, build,
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

    node "OBC - Kubos Linux" {
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
    - :doc:`sdk-examples` - A list of example projects using the SDK
    - :doc:`sdk-project-config` - How to configure a Kubos project's settings, dependencies, and underlying hardware information
    - :doc:`sdk-cheatsheet` - A quick guide for the most common SDK tasks
    - :doc:`sdk-reference` - The full list of Kubos CLI commands
    - :doc:`sdk-rust` - How to develop and run Kubos projects using Rust
    - :doc:`sdk-python` - How to develop and run Kubos projects using Python
    - :doc:`sdk-lua` - How to use Kubos projects written in Lua
    - :doc:`windows-dev-environment` - How to develop Kubos projects from a Windows-based host computer
    - :doc:`sdk-upgrading` - How to upgrade to the latest version of the Kubos SDK

.. toctree::
    :hidden:

    ../installation-docs/sdk-installing
    sdk-examples
    sdk-project-config
    sdk-cheatsheet
    sdk-reference
    sdk-rust
    sdk-python
    sdk-lua
    windows-dev-environment
    Upgrading Kubos SDK <sdk-upgrading>
