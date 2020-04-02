Kubos SDK Docs
==============

While the majority of KubOS development can be done locally on a host machine, we also provide a
standalone SDK which can be used to assist with the development process.

The "Kubos SDK" is a term used to describe all of the components used
to build KubOS projects:

-  `Vagrant <https://www.vagrantup.com/>`__ box - The VM that contains the "ready to run" Kubos development
   environment
-  Kubos source modules - The underlying libraries on which KubOS projects
   are built

Internally, we use this SDK in order to build KubOS releases and to host our CI tests.

Externally, the SDK is most useful for:

- Windows users
- Users who do not want to muddle their host systems with all of the dependencies required to build
  and execute KubOS projects
- Users who want to build a :ref:`custom KubOS image <custom-klb>`

.. uml::

    @startuml
    left to right direction

    actor User

    node "Kubos SDK" as sdk{
        () "init" as init
        folder "kubos-proj" as proj {
            folder source {
            }
            () "build" as build
            [binary]
        }
        folder "Kubos Source" as k_source {
        }
    }

    () "transfer" as flash

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

    - :doc:`sdk-installing` - How to install the SDK onto your host machine
    - :doc:`sdk-c` - A guide to using C with the Kubos SDK
    - :doc:`sdk-rust` - How to develop and run Kubos projects using Rust
    - :doc:`sdk-python` - How to develop and run Kubos projects using Python
    - :doc:`sdk-upgrading` - How to upgrade to the latest version of the Kubos SDK
    - :doc:`sdk-advanced-cross-compiling` - How to cross compile unsupported targets with Rust

.. toctree::
    :hidden:

    sdk-installing
    sdk-rust
    sdk-python
    sdk-c
    Upgrading the Kubos SDK <sdk-upgrading>
    sdk-advanced-cross-compiling
