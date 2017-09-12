#include <stdbool.h>
#include <stdlib.h>              
#include <dbus/dbus-glib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <dbus-1.0/dbus/dbus.h>
#include <math.h>
void reply_to_method_call(DBusMessage* msg, DBusConnection* conn)
{
   DBusMessage* reply;
   DBusMessageIter rootIter;
   dbus_uint32_t serial = 0;
   dbus_uint32_t a;
   dbus_uint32_t b;
   dbus_uint32_t sum;

   // read the arguments
   if (!dbus_message_iter_init(msg,&rootIter))
      fprintf(stderr, "Message has no arguments!\n");
    dbus_message_iter_get_basic(&rootIter, &a);
    printf("Method called with %d\n", a);
    if(dbus_message_iter_has_next(&rootIter))
    {
        dbus_message_iter_next(&rootIter);
        dbus_message_iter_get_basic(&rootIter, &b);
        printf("Method called with %d\n", b);
    }

   if ( dbus_message_is_method_call( msg, "test.method.Type", "Method" ) ) 
{
    sum=a+b; 
} 
  // create a reply from the message
   reply = dbus_message_new_method_return(msg);

   // add the arguments to the reply
   dbus_message_iter_init_append(reply, &rootIter);

   if (!dbus_message_iter_append_basic(&rootIter, DBUS_TYPE_INT32, &sum)) { 
      fprintf(stderr, "Out Of Memory!\n"); 
      exit(1);

   }

   // send the reply && flush the connection
   if (!dbus_connection_send(conn, reply, &sum)) {
      fprintf(stderr, "Out Of Memory!\n"); 
      exit(1);
   }
   dbus_connection_flush(conn);

   // free the reply
   dbus_message_unref(reply);
}

int main()
{
 DBusMessage* msg;
   DBusMessage* reply;
   DBusMessageIter args;
   DBusConnection* conn;
   DBusError err;
   int ret;
   char* param;

   printf("Listening for method calls\n");

   // initialise the error
   dbus_error_init(&err);

   // connect to the bus and check for errors
   conn = dbus_bus_get(DBUS_BUS_SESSION, &err);
   if (dbus_error_is_set(&err)) { 
      fprintf(stderr, "Connection Error (%s)\n", err.message); 
      dbus_error_free(&err); 
   }
   if (NULL == conn) {
      fprintf(stderr, "Connection Null\n"); 
      exit(1); 
   }

   // request our name on the bus and check for errors
   ret = dbus_bus_request_name(conn,"test.server.source", DBUS_NAME_FLAG_REPLACE_EXISTING , &err);
   if (dbus_error_is_set(&err)) { 
      fprintf(stderr, "Name Error (%s)\n", err.message); 
      dbus_error_free(&err);
   }


   // loop, testing for new messages
   while (true) {
      // non blocking read of the next available message
      dbus_connection_read_write(conn, 0);
      msg = dbus_connection_pop_message(conn);

      // loop again if we haven't got a message
      if (NULL == msg) { 
         sleep(1); 
         continue; 
      }

      if ( dbus_message_has_interface(msg, "test.method.Type") )
        reply_to_method_call( msg, conn );
      // free the message
      dbus_message_unref(msg);
   }

   // close the connection
   dbus_connection_close(conn);
}