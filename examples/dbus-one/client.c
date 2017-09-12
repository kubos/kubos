#include <dbus/dbus.h>
#include <stdio.h>
#include <stdlib.h>
 
static void check_and_abort(DBusError *error);
static DBusHandlerResult handle_messages(DBusConnection *connection, DBusMessage *message, void *user_data);
 
int main() {
    DBusConnection *connection = NULL;
    DBusError error;
    DBusMessage *msgQuery = NULL;
    DBusMessage *msgReply = NULL;
    const char *interfaceName = NULL;
    const char *versionValue = NULL;
 
    dbus_error_init(&error);
    connection = dbus_bus_get(DBUS_BUS_SESSION, &error);
    check_and_abort(&error);
 
    interfaceName = "org.KubOS.PowerManager";

    dbus_connection_add_filter(connection, handle_messages, NULL, NULL);

    dbus_bus_add_match(connection, "type='signal',interface='org.KubOS.PowerManager.PowerStatus'", NULL);
    dbus_connection_flush(connection);

    DBusObjectPathVTable vtable;
    vtable.message_function = handle_messages;
    vtable.unregister_function = NULL;
    dbus_connection_try_register_object_path(connection,
                        "/org/KubOS/client",
                        &vtable,
                        NULL,
                        &error);

    while (1)
    {
        dbus_connection_read_write_dispatch(connection, 1000);
    }

    return 0;
}

static DBusHandlerResult handle_messages(DBusConnection *connection, DBusMessage *message, void *user_data) {
    const char *interface_name = dbus_message_get_interface(message);
    const char *member_name = dbus_message_get_member(message);

    printf("Got Message\n%s\n%s\n", interface_name, member_name);

    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}
 
static void check_and_abort(DBusError *error) {
    if (!dbus_error_is_set(error)) return;
    puts(error->message);
    abort();
}