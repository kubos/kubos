#include <ipc/pubsub.h>

bool __wrap_subscriber_read(pubsub_conn conn, void * buffer, int buffer_size, uint8_t port)
{
    if ((conn.conn_handle != NULL) && (buffer != NULL))
        return true;
    return false;
}

bool __wrap_subscriber_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
{
    if (conn == NULL)
    {
        return false;
    }
    return true;
}

bool __wrap_send_csp(pubsub_conn conn, void * data, uint16_t length)
{
    csp_packet_t * csp_packet = NULL;
    csp_conn_t * csp_conn = conn.conn_handle;
    if ((data != NULL) && (length > 0) && (csp_conn != NULL))
    {
        return true;
    }
    return false;
}