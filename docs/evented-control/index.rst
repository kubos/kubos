Evented Control Plane
=====================

Abstract
--------

This documentation is for the Evented Control Plane (ECP)
middleware. It is to pass "control oriented" flight domain messages
between software components. The ECP implements an interprocess queuing
system that allows flight software components to listen for messages on
"n of m" message broadcast channels.

The ECP is not a generic messaging system. It is used to communicate
domain-specific predefined messages related to common flight tasks. It
is intended to link sensor elements (GPS, IMU, Star Tracker, etc.) with
logic and control elements (Orientataion, Camera Control, Downlink
Control, etc.) to allow the latter to be programmed in an event oriented
fashion.

Introduction
-------------

Consider the typical technique for programming spacecraft orientation
control, for instance. Typically, we would have single a process which
reads GPS, IMU and possibly Star Tracker data to determine it's current
position. It then proceeds through a series of tests to determine it's
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
these logic compnents would independently determine what actions they
think the spacecraft should take. A prioritization algorithme could
then be run in a separate process. At any given time, the spacecraft
would perform the highest priority task.

This style of programming requires developers to think clearly about
process priorities, but it allows them to greatly simplify the logic
of other components.

What subsystems are included?
-----------------------------

THe ECP middleware will support the following systems:

  SYS - System Status & Infrastructure Information
  RIO - Radio Control & Status
  EPS - Power Supply Control & Status
  GPS - Global Positioning System (GPS)
  IMU - Intertial Measurement Unit
  DWN - Downlink Control
  STO - Storage Control

Under the hood, each system is represented by a "channel" that carries
channel and message specific messages. Processes (clients) use the ECP
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

* The ECP_Init() function initializes an opaque data structure and
  initializes the processes' link with the message bus.

* The ECP_Listen() function allows the process to register a callback
  function for a particular message or message channel. The
  ECP_Unlisten() function de-registers a callback registered with the
  ECP_Listen() function.

* The ECP_Broadcast() function is used to broadcast a message to all
  processes listening on a particular channel.

* The ECP_Loop() function iterates through the event loop for a fixed
  amount of time or following the execution of a listen callback.

* The ECP_Destroy() function unregisters callbacks, deallocates memory
  and disassociates the client from any message subscriptions
  associated with it.

The ECP middleware makes no assumptions about the number of processes
subscribing to or publishing to a particular channel. Messages received
over the ECP_Listen() interface are not directly acknowledged. Consumers
of the ECP API may use SYS messages to discover the address of endpoints
responsible for sending messages.

Clients of the ECP middleware are expected to produce or consume messages
broadcast on a channel; clients may both send and receive messages.
Sending a message is simple, and requires a single call to the
ECP_Broadcast() function. Receiving messages requires the client to
register a callback with the ECP_Listen() function.

The ECP middleware functions each return an integer status code, cast as
a ECPStatus type. They all return a zero upon successful completion. For
example, this is a typical call sequence:

.. code-block:: c

    ECPStatus err = ECP_OK;
    const char * my_interface = "org.KubOS.Client";

    if(ECP_OK != (err = ECP_Init(&context, my_interface)))
    {
        /* Perform error recovery here */
        printf( "Error %d when calling ECP_Init()\n", err );
        break;
    }

    /* Continue execution here */

The general pattern of use is init-listen-loop/send-destroy. Don't call
ECP_Listen() or ECP_Broadcast() before calling ECP_Init() or after
calling ECP_Destroy(). Though the operating system will likely deallocate
memory allocated by the ECP_Init() function, there's no guarantee the
underlying messaging system will properly disconnect from an message
endpoint without the ECP_Destroy() call. Always call the ECP_Destroy()
function before exiting a process.

Practically speaking calls to ECP_Listen, ECP_Call and ECP_Broadcast
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

   ECPStatus status_handler(eps_power_status status);

   #define MY_NAME "org.KubOS.client"

   int main(int argc, char * argv[])
   {
       ECPStatus   err;
       ECPContext context;

       do
       {

           if (ECP_OK != (err = ECP_Init(&context, MY_NAME)))
           {
               printf("Error calling ECP_Init(): %d\n", err);
               break;
           }
           printf("Successfully called ECP_Init()\n");

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
               ECP_Loop(&context, 100);
           }
       } while (0);

       if (ECP_OK != (err = ECP_Destroy(&context)))
       {
           printf("Error calling ECP_Destroy(): %d\n", err);
       }

       return (err);
   }

   tECP_Error status_handler(eps_power_status status)
   {
       printf("Got status %d:%d\n", status.line_one, status.line_two);
   }



.. toctree::
  :caption: Low Level ECP API
  :name: ecp-apis

  evented-control


.. toctree::
  :caption: ECP Message APIs
  :name: ecp-msg-apis

  ecp-messages
  power
