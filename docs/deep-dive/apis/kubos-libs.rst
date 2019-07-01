Kubos Libraries
===============

Kubos provides the following libraries to assist with core functionality:

- :doc:`Applications API <../../ecosystem/apps/app-guide>` - Simplifies the process of setting up a
  mission application

    - :doc:`Python <../../ecosystem/apps/python-app-api>`
    - |rust-app-api|

- :doc:`Kubos Service Library <../../ecosystem/services/service-dev>` - Provides a base framework for
  KubOS services

    - `Python <https://github.com/kubos/kubos/tree/master/libs/kubos-service>`__
    - |rust-service|

- |rust-system| - Provides helper functions for things like reading configuration files and setting
  U-Boot environment variables
- |comms-service| - Provides a base framework to assist with implementing the system's
  communications service
- |file-protocol| - Used by the file service in order to process simultaneous, asynchronous file
  transfer requests
- |shell-protocol| - Used by the shell service in order to handle simultaneous, asynchronous
  remote shell connections
- |cbor-protocol| - Constructs and processes the CBOR packets which are used for sending data to and
  from the file and shell services
- |channel-protocol| - Sends and receives CBOR packets for the file and shell services, maintaining
  ownership and separation between simultaneous client connections

 .. |rust-app-api| raw:: html

    <a href="../../rust-docs/kubos_app/index.html" target="_blank">Rust</a>

 .. |rust-service| raw:: html

    <a href="../../rust-docs/kubos_service/index.html" target="_blank">Rust</a>

 .. |rust-system| raw:: html

    <a href="../../rust-docs/kubos_system/index.html" target="_blank">Kubos System</a>

 .. |comms-service| raw:: html

    <a href="../../rust-docs/comms_service/index.html" target="_blank">Communications Service Framework</a>

 .. |file-protocol| raw:: html

    <a href="../../rust-docs/file_protocol/index.html" target="_blank">File Protocol</a>

 .. |shell-protocol| raw:: html

    <a href="../../rust-docs/shell_protocol/index.html" target="_blank">Shell Protocol</a>

 .. |cbor-protocol| raw:: html

    <a href="../../rust-docs/cbor_protocol/index.html" target="_blank">CBOR Protocol</a>

 .. |channel-protocol| raw:: html

    <a href="../../rust-docs/channel_protocol/index.html" target="_blank">Channel Protocol</a>