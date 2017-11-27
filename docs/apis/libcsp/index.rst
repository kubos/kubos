Cubesat Space Protocol
=======================

Cubesat Space Protocol (CSP) is a small protocol stack written in C. CSP is designed
to ease communication between distributed embedded systems in smaller networks, 
such as Cubesats. The design follows the TCP/IP model and includes a transport 
protocol, a routing protocol and several MAC-layer interfaces. The core of libcsp 
includes a router, a socket buffer pool and a connection oriented socket API.

.. toctree::
    :caption: CSP Docs
    :maxdepth: 1

    Overview <csp_docs/overview>
    History <csp_docs/history>
    Library Structure <csp_docs/structure>
    CSP Interfaces <csp_docs/interfaces>
    How CSP uses memory <csp_docs/memory>
    The Protocol Stack <csp_docs/protocolstack>
    Network Topology <csp_docs/topology>
    Maximum Transfer Unit <csp_docs/mtu>
    Examples <csp_docs/example>

.. toctree::
    :caption: Core

    buffer
    cmp
    csp
    crc32
    debug
    endian
    error
    interfacelist
    interface
    platform
    routingtable
    types

.. toctree::
    :caption: Architecture Dependent

    clock
    malloc
    queue
    semaphore
    system
    thread
    time

.. toctree::
    :caption: Interfaces

    caninterface
    i2cinterface
    kissinterface
    lointerface
    socketinterface

.. toctree::
    :caption: Drivers

    candriver
    i2cdriver
    usartdriver
    socketdriver