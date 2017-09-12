#include <dbus/dbus.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
static void check_and_abort(DBusError *error);
static DBusHandlerResult tutorial_messages(DBusConnection *connection, DBusMessage *message, void *user_data);
static void respond_to_introspect(DBusConnection *connection, DBusMessage *request);
static void respond_to_sum(DBusConnection *connection, DBusMessage *request);
 
static int count = 0;

int main() {
    DBusConnection *connection;
    DBusError error;
    DBusObjectPathVTable vtable;
    DBusMessage * msg;
 
    dbus_error_init(&error);
    connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
    check_and_abort(&error);
 
    dbus_bus_request_name(connection, "org.KubOS.PowerManager", 0, &error);
    check_and_abort(&error);
 
    vtable.message_function = tutorial_messages;
    vtable.unregister_function = NULL;
     
    dbus_connection_try_register_object_path(connection,
                         "/org/KubOS/PowerManager",
                         &vtable,
                         NULL,
                         &error);
    check_and_abort(&error);
 
    char * st = "powerrr";
    DBusMessageIter args;
    dbus_uint32_t serial = 0;
    while(1) {
        
        msg = dbus_message_new_signal("/org/KubOS/PowerManager/PowerStatus",
                                       "org.KubOS.PowerManager.PowerStatus",
                                        "power");
        if (NULL == msg)
        {
            printf("msg null\n");
        }
        else
        {
            dbus_message_iter_init_append(msg, &args);
            if (!dbus_message_iter_append_basic(&args, DBUS_TYPE_STRING, &st))
            {
                printf("No memory\n");
            }
            else
            {
                if (!dbus_connection_send(connection, msg, &serial))
                {
                    printf("failed to send\n");
                }
                else
                {
                    printf("Sent!\n");
                    sleep(5);
                }   
            }
        }
        

        dbus_connection_read_write_dispatch(connection, 1000);
    }
     
    return 0;
}

static void check_and_abort(DBusError *error) {
    if (dbus_error_is_set(error)) {
        puts(error->message);
        abort();
    }
}


static DBusHandlerResult tutorial_messages(DBusConnection *connection, DBusMessage *message, void *user_data) {
    const char *interface_name = dbus_message_get_interface(message);
    const char *member_name = dbus_message_get_member(message);
     
    if (0==strcmp("org.KubOS.PowerManager", interface_name) &&
           0==strcmp("EnableRail", member_name)) {
         
        respond_to_sum(connection, message);
        return DBUS_HANDLER_RESULT_HANDLED;
    } else {
        return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
    }
}


static void respond_to_sum(DBusConnection *connection, DBusMessage *request) {
    DBusMessage *reply;
 
    const char *introspection_data =
        "hello there";
     
    reply = dbus_message_new_method_return(request);
    dbus_message_append_args(reply,
                 DBUS_TYPE_STRING, &introspection_data,
                 DBUS_TYPE_INVALID);
    dbus_connection_send(connection, reply, NULL);
    dbus_message_unref(reply);
}
