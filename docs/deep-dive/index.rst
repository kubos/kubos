Under the Hood of KubOS
=======================

These docs give a more detailed examination of the inner workings of KubOS

APIs
----

Kubos provides a variety of APIs to help with the development of mission software:

  - :doc:`Device Interfaces <apis/device-api/index>` - APIs for external devices (ex. radio), built on top of the Kubos HAL
  - :doc:`OBC APIs <apis/obc-api/index>` - APIs for features which are internal to a particular OBC
  - :doc:`Kubos HAL <apis/kubos-hal/index>` - Hardware interface abstractions (I2C, SPI, etc)
  - :doc:`Kubos Libraries <apis/kubos-libs>` - Non-hardware libraries

.. toctree::
    :caption: APIs
    :hidden:
    
    apis/device-api-guide
    Device Interfaces <apis/device-api/index>
    OBC APIs <apis/obc-api/index>
    Kubos HAL <apis/kubos-hal/index>
    Kubos Libraries <apis/kubos-libs>
    
Protocols
---------

Intra-satellite communication is generally handled using HTTP over TCP/IP, with the packet payloads
being structured as :doc:`GraphQL <../ecosystem/services/graphql>` requests or JSON responses.

For procedures which require space-ground communication, special care has been taken to craft
protocols which can handle higher rates of packet loss and function well in the more asynchronous
comms environment.

.. note::

    We are currently working on a dedicated Space Ground Interface Control Document to highlight
    this expected link behavior.

.. toctree::
    :maxdepth: 1

    File Protocol Overview <protocols/file-protocol>
    Shell Protocol Overview <protocols/shell-protocol>

.. _custom-klb:

Kubos Linux
-----------

While Kubos does create and distribute official releases of Kubos Linux, users are free to create
their own builds.
This allows KubOS to be easily customized on a per-mission basis.

Users will most likely want to create their own builds when they create new hardware services which
should be included in the OS' root file system.

.. toctree::
    :maxdepth: 1
    
    klb/configuring-kubos
    klb/kubos-linux-on-bbb
    klb/kubos-linux-on-iobc
    klb/kubos-linux-on-mbm2
    
Design Decisions
----------------

When developing new features for KubOS, we frequently have to make a choice between two or more
tools/libraries/frameworks which will most closely give us our desired characteristics.

The design decisions doc give a quick summary of how we have decided on our particular toolset.

.. toctree::
    :maxdepth: 2
    
    design-decisions