Under the Hood of KubOS
=======================

TODO: Flesh this out. There are more things to dive into than just the APIs

These docs give a more detailed examination of the inner workings of KubOS

TODO: The shell and file protocol are buried within the API docs. Maybe pull them up to this top level

APIs
----

Kubos provides a variety of APIs to help with the development of mission software:

  - :doc:`Device Interfaces <apis/device-api/index>` - APIs for external devices (ex. radio), built on top of the Kubos HAL
  - :doc:`OBC APIs <apis/obc-api/index>` - APIs for features which are internal to a particular OBC
  - :doc:`Kubos HAL <apis/kubos-hal/index>` - Hardware interface abstractions (I2C, SPI, etc)
  - :doc:`Kubos Libraries <apis/kubos-libs/index>` - Non-hardware libraries

.. toctree::
    :caption: APIs
    :hidden:
    
    apis/device-api-guide
    Device Interfaces <apis/device-api/index>
    OBC APIs <apis/obc-api/index>
    Kubos HAL <apis/kubos-hal/index>
    Kubos Libraries <apis/kubos-libs/index>

.. _sysadmin:

Kubos Linux
-----------

While Kubos does create and distribute official releases of Kubos Linux, users are free to create
their own builds.
This allows KubOS to be easily customized on a per-mission basis.

Users will most likely want to create their own builds when they create new hardware services which
should be included the OS' root file system.

.. toctree::
    :maxdepth: 1
    
    klb/configuring-kubos
    klb/kubos-linux-on-bbb
    klb/kubos-linux-on-iobc
    klb/kubos-linux-on-mbm2