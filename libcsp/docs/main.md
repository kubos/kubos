#  Cubesat Space Protocol
## {#csp-main}

Cubesat Space Protocol (CSP) is a small protocol stack written in C. CSP is designed to ease communication between distributed embedded systems in smaller networks, such as Cubesats. The design follows the TCP/IP model and includes a transport protocol, a routing protocol and several MAC-layer interfaces. The core of libcsp includes a router, a socket buffer pool and a connection oriented socket API.

### Docs:

 - [Overview](docs/overview.md)
 - [CSP Interfaces](doc/interfaces.md)
 - [How CSP uses memory](doc/memory.md)
 - [Maximum Transfer Unit](doc/mtu.md)
 - [The Protocol Stack](doc/protocolstack.md)
 - [Network Topology](doc/topology.md)

### API Modules:

### Core
 - @subpage Buffer
 - @subpage CMP
 - @subpage CSP
 - @subpage CRC32
 - @subpage Debug
 - @subpage Endian
 - @subpage Error
 - @subpage InterfaceList
 - @subpage Interface
 - @subpage Platform
 - @subpage RoutingTable
 - @subpage Types

### Architecture Dependent
 - @subpage Clock
 - @subpage Malloc
 - @subpage Queue
 - @subpage Semaphore
 - @subpage System
 - @subpage Thread
 - @subpage Time

### Interfaces
 - @subpage CANInterface
 - @subpage I2CInterface
 - @subpage KISSInterface
 - @subpage LOInterface
 - @subpage SocketInterface

### Drivers
 - @subpage CANDriver
 - @subpage I2CDriver
 - @subpage USARTDriver
 - @subpage SocketDriver
