Evented Control Plane
=====================

Abstract
--------

This documentation is for the Evented Control Plane (ECP)
middleware. Its role is to pass "control oriented" flight domain messages
between software components. The ECP implements an interprocess queuing
system that allows flight software components to listen for messages on
"n of m" message broadcast channels.

The ECP is not a generic messaging system. It is used to communicate
domain-specific predefined messages related to common flight tasks. It
is intended to link sensor elements (GPS, IMU, Star Tracker, etc.) with
logic and control elements (Orientation, Camera Control, Downlink
Control, etc.) to allow the latter to be programmed in an event oriented
fashion.

This specific document mainly deals with the lower level workings of
the ECP API. This document is intended for those maintaining the ECP
API and creating the higher level abstractions around it. Application
developers should only have to interact with application message
specific abstractions which use the ECP API, not the lower level functions.

Introduction
-------------

Consider the typical technique for programming spacecraft orientation
control. Typically, we would have a single process which
reads GPS, IMU and possibly Star Tracker data to determine its current
position. It then proceeds through a series of tests to determine its
current state, what task has priority at the moment and whether it is
more important to orient the spacecraft towards the sun (batteries are
low), towards a downlink station (storage is low) or to orient a sensor
or camera towards a particular location on the planet below.

This style of programming requires the developer to model the entire
state of the spacecraft's orientation logic in a single process.

In the evented control model supported by the ECP, developers are able
to decouple the logic for power, downlink and targeting. Application
logic is decomposed into its constituent components and implemented as
separate, but communicating processes. Instead of a monolithic
application implementing all control logic, we could create separate
software processes for power, downlink and targeting control. Each of
these logic components independently determine what actions they
think the spacecraft should take. A prioritization algorithm could
then be run in a separate process. At any given time, the spacecraft
would perform the highest priority task.

This style of programming requires developers to think clearly about
process priorities, but it allows them to greatly simplify the logic
of other components.

How does it work?
-----------------

The ECP middleware is currently built on top of
`D-Bus <https://www.freedesktop.org/wiki/Software/dbus/>`_.
D-Bus provides a messaging bus which is stock to many Linux systems,
provides abstraction of the transport layer and also provides
the functionality for Pub Sub and RPC style communication.

Ideally the ECP will completely hide away the details of how D-Bus works
and its internals. Future implementations may not use D-Bus, therefore
we should not be tightly coupled to how it behaves.

The ECP API is split into two halves: the low level ECP API and the
high level message API. The low level ECP API is an abstraction of
underlying messaging layers and patterns. It does not actually define
any messages but rather defines the tools used to create, send
and receive those messages. This is where the D-Bus abstraction lives.
The higher level message API is an application specific abstraction around
the ECP API. It defines domain specific messages using the ECP API and
provides simple functions for sending and receiving those messages.

Basic flow of ECP usage by Subscriber/Client
--------------------------------------------

.. uml::

   @startuml
   Client --> ECP: Connection Request (ecp_init)
   ECP --> DBus: Initiate Connection (dbus_bus_get)
   ECP --> DBus: Register Name (dbus_request_name)
   ECP --> DBus: Add Message Filter (dbus_connection_add_filter)
   Client --> ECP: Listen Request (ecp_listen)
   ECP --> DBus: Subscribe (dbus_bus_add_match)
   Client --> ECP: Loop (ecp_loop)
   ECP --> DBus: Loop (dbus_connection_read_write_dispatch)
   DBus --> ECP: Data (callback to _ecp_message_handler)
   ECP --> Client: Published Data (callback to message specifc handler)
   @enduml

Basic flow of ECP usage by Publisher/Server
-------------------------------------------

.. uml::

   @startuml
   Server --> ECP: Connection Request (ecp_init)
   ECP --> DBus: Initiate Connection (dbus_bus_get)
   ECP --> DBus: Register Name (dbus_request_name)
   ECP --> DBus: Add Message Filter (dbus_connection_add_filter)
   Server --> ECP: Publish data (ecp_send)
   ECP --> DBus: Publish data (dbus_connection_send)
   @enduml

What subsystems are included?
-----------------------------

The ECP middleware will support the following systems:

  - SYS - System Status & Infrastructure Information
  - RIO - Radio Control & Status
  - EPS - Power Supply Control & Status
  - GPS - Global Positioning System (GPS)
  - IMU - Intertial Measurement Unit
  - DWN - Downlink Control
  - STO - Storage Control

Currently only the EPS subsystem is supported.

Under the hood, each system is represented by a "channel" that carries
messages specific to the system's functionality. Processes (clients) use the ECP
to send many-cast messages between themselves. The ECP middleware API is
"broker agnostic" in that it does not itself require a broker, but the
ECP implementation may be based on a brokered model. Consumers of the
ECP API should be prepared to handle error messages related to broker
failures even if they do not believe their implementation uses one.

How do I use it?
----------------

The EPC middleware exports a "Stock C" interface in ecp.h and a library
(libecp) which implements its functionality. The middleware assumes an
"Init, Use, Clean Up" pattern.

* The ecp_init() function initializes an opaque data structure and
  initializes the processes' link with the message bus.

* The ecp_listen() function allows the process to register a callback
  function for a particular message or message channel. The
  ECP_Unlisten() function de-registers a callback registered with the
  ECP_Listen() function.

* The ecp_broadcast() function is used to broadcast a message to all
  processes listening on a particular channel.

* The ecp_loop() function iterates through the event loop
  for a fixed amount of time or until the execution of a listen callback.
  The event loop is an internal ECP function which abstracts away the
  work of waiting for new messages and handing them off to the
  appropriate message handlers.

* The ecp_destroy() function unregisters callbacks, deallocates memory
  and disassociates the client from any message subscriptions
  associated with it.

The ECP middleware makes no assumptions about the number of processes
subscribing to or publishing to a particular channel. Messages received
over the ECP_Listen() interface are not directly acknowledged. It is the
responsibility of the subscriber to acknowledge any messages received,
if that is appropriate for the message.

Clients of the ECP middleware are expected to produce or consume messages
broadcast on a channel; clients may both send and receive messages.
Sending a message is simple, and requires a single call to the
ecp_broadcast() function. Receiving messages requires the client to
register a callback with the ecp_listen() function.

The ECP middleware functions each return an integer status code, cast as
a KECPStatus type. They all return a zero upon successful completion. For
example, this is a typical call sequence:

.. code-block:: c

    KECPStatus err = ECP_OK;
    const char * my_interface = "org.KubOS.Client";

    if(ECP_OK != (err = ecp_init(&context, my_interface)))
    {
        /* Perform error recovery here */
        printf( "Error %d when calling ecp_init()\n", err );
        break;
    }

    /* Continue execution here */

The general pattern of use is init-listen-loop/send-destroy. Don't call
ecp_listen() or ecp_broadcast() before calling ecp_init() or after
calling ecp_destroy(). Though the operating system will likely deallocate
memory allocated by the ecp_init() function, there's no guarantee the
underlying messaging system will properly disconnect from an message
endpoint without the ecp_destroy() call. Always call the ecp_destroy()
function before exiting a process.

Practically speaking calls to ecp_listen, ecp_call and ecp_broadcast
will be hidden behind higher level, service specific messaging APIs.
The lower level ECP functions will be used to create these higher
level APIs, but most likely they will not be used directly
in user applications.

Here is an example of the init-listen-loop/send-destroy pattern:

.. code-block:: c

   #include <eps-api/eps.h>
   #include <evented-control/ecp.h>
   #include <evented-control/messages.h>
   #include <stdio.h>
   #include <stdlib.h>

   KECPStatus status_handler(EPSPowerState status);

   #define MY_NAME "org.KubOS.client"

   int main(int argc, char * argv[])
   {
       KECPStatus   err;
       ECPContext context;

       do
       {

           if (ECP_OK != (err = ecp_init(&context, MY_NAME)))
           {
               printf("Error calling ecp_init(): %d\n", err);
               break;
           }
           printf("Successfully called ecp_init()\n");

           /**
            * Hidden behind on_power_status is code which creates a
            * message handler for the Power Status message and
            * calls ECP_Listen with the Power Status interface.
            */
           if (ECP_Ok != (err = on_power_status(&context, &status_handler)))
           {
               printf("Error calling on_power_status\n");
               break;
           }

           /**
            * Hidden behind enable_line is code which creates a new
            * enable line message and sends it over ECP_Call.
            */
           if (ECP_OK != (err = enable_line(&context, 1)))
           {
               printf("Error calling enable line\n");
               break;
           }

           printf("Successfully enabled line 1\n");

           for (int i = 0; i < 10; i++)
           {
               ecp_loop(&context, 100);
           }
       } while (0);

       if (ECP_OK != (err = ecp_destroy(&context)))
       {
           printf("Error calling ecp_destroy(): %d\n", err);
       }

       return (err);
   }

   KECPStatus status_handler(EPSPowerState status)
   {
       printf("Got status %d:%d\n", status.line_one, status.line_two);
       return ECP_OK;
   }


ECP APIs
--------

.. toctree::
  :caption: Low Level ECP API
  :name: ecp-apis

  evented-control


.. toctree::
  :caption: ECP Message APIs
  :name: ecp-msg-apis

  ecp-messages
  power
