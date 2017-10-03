ABSTRACT

This repository contains the code for the Evented Control Plane (ECP)
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

INTRODUCTION

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

WHAT SUBSYSTEMS ARE INCLUDED?

THe ECP middleware currently supports the following systems:

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

HOW DO I USE IT?

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
of the ECP API may use SYS messages to discover the address of endpoints responsible for sending messages.

Clients of the ECP middleware are expected to produce or consume messages
broadcast on a channel; clients may both send and receive messages.
Sending a message is simple, and requires a single call to the
ECP_Broadcast() function. Receiving messages requires the client to
register a callback with the ECP_Listen() function.

The ECP middleware functions each return an integer status code, cast as
a ECPStatus type. They all return a zero upon successful completion. For
example, this is a typical call sequence:

  ECPStatus err = ECP_E_NOERR;

  if( ECP_E_NOERR != ( err = ECP_Init( & context ) ) ) {
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

Here is an example of the init-listen-loop/send-destroy pattern:

  #include "ecp.h"

  ECPStatus func( ECPContext * context, tECP_Channel channel,
                   tECP_Message * message );

  int main() {
    ECPContext context;
    ECPStatus   err = ECP_E_NOERR;
    tECP_Message msg;
    int          i;
    int          initialized = 0;

    do {
      if( ECP_E_NOERR != ( err = ECP_Init( & context ) ) ) {
        printf( "Error %d calling ECP_Init()\n", err );
        break;
      }

      initialized = 1;

      /* Send a message before calling loop. perfectly acceptable */
      msg.messageid = ECP_M_SYS_BEGIN;
      msg.begin.id  = 0xCAFEB0EF;

      if( ECP_E_NOERR !=
          ( err = ECP_Broadcast( &context, ECP_C_SYS, &msg ) ) ) {
        printf( "Error %d calling ECP_Broadcast()\n", err );
        break;
      }

      if( ECP_E_NOERR !=
          ( err = ECP_Listen( & context, ECP_C_SYS, func ) ) ) {
        printf( "Error %d calling ECP_Listen()\n", err );
        break;
      }

      /* Now loop for (at most) 15 seconds, looking for a heartbeat */
      for( i = 0; ( i < 15 ) && ( err == ECP_E_NOERR ) ; i++ ) {
        err = ECP_Loop( & context, 1000 );
      }

      if( err != ECP_E_NOERR ) {
        printf( "Error %d calling ECP_Loop()\n", err );
        break;
      }
    } while( 0 );

    if( 1 == initialized ) {
      if( ECP_E_NOERR != ( err = ECP_Destroy( & context ) ) ) {
        printf( "Error %d calling ECP_Destroy()\n", err );
      }
    }

    if( ECP_E_NOERR == err ) {
      return( 0 );
    } else {
      return( 2 );
    }
  }

  ECPStatus func( ECPContext * context, tECP_Channel channel,
                   tECP_Message * message ) {
    ECPStatus   err = TCP_E_NOERR;
    tECP_Message msg;

    if( ECP_C_SYS != channel ) {
      printf( "That's weird, we received a message on channel %d\n",
              channel );
      err = TCP_E_GENERIC;
    } else {
      switch( message.messageid ) {
        case ECP_M_SYS_HEARTBEAT:
          printf( "Listen to my heartbeat\n" );

          /* You can send a message from w/i a message handler */
          msg.messageid = ECP_M_SYS_NULL;
          err = ECP_Broadcast( &context, ECP_C_SYS, &msg );
          break;
      }
    }

    return( err );
  }

MESSAGES

SYS - System Status & Infrastructure Information

MESSAGE ID 0 : NULL

MESSAGE ID 1 : BEGIN

MESSAGE ID 2 : HEART


GPS - Global Positioning System (GPS)

MESSAGE ID 0 : INFO

MESSAGE ID 1 : POSITION

MESSAGE ID 2 :

IMU - Intertial Measurement Unit

MESSAGE ID 0 : INFO

MESSAGE ID 1 : ORIENTATION


DWN - Downlink Control

MESSAGE ID 0 : INFO

MESSAGE ID 1 : LOCK

MESSAGE ID 2 : STATUS

MESSAGE ID 3 : UNLOCK


STO - Storage Control

MESSAGE ID 0 : INFO

MESSAGE ID 1 : RECEIVED


POW - Power and Battery

MESSAGE ID 0 : INFO

MESSAGE ID 1 : BATTERY

MESSAGE ID 2 : PANEL

MESSAGE ID 3 : RTG
