Kubos ECP APIs
==============

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

.. toctree::
  :hidden:
  :caption: Low Level ECP API
  :name: ecp-apis

  evented-control


.. toctree::
  :hidden: 
  :caption: ECP Message APIs
  :name: ecp-msg-apis

  ecp-messages
  power

