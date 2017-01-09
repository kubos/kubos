#include <csp/csp.h>

csp_packet_t * __wrap_csp_read(csp_conn_t * conn, uint32_t timeout)
{
    if (conn == NULL)
    {
        return NULL;
    }
    return malloc(sizeof(csp_packet_t *));
}

int __wrap_csp_conn_dport(csp_conn_t * conn)
{
    return 12;
}

int __wrap_csp_conn_sport(csp_conn_t * conn)
{
    return 12;
}

void __wrap_csp_service_handler(csp_conn_t * conn, csp_packet_t * packet)
{
    if (packet != NULL) free(packet);
}

csp_conn_t * __wrap_csp_accept(csp_socket_t * socket, int timeout)
{
    return malloc(sizeof(csp_conn_t *));
}

csp_socket_t * __wrap_csp_socket(uint32_t opts)
{
    return malloc(sizeof(csp_socket_t *));
}

int __wrap_csp_bind(csp_socket_t *socket, uint8_t port)
{
    return CSP_ERR_NONE;
}

int __wrap_csp_listen(csp_socket_t *socket, size_t conn_queue_length)
{
    return CSP_ERR_NONE;
}

csp_conn_t * __wrap_csp_connect(uint8_t prio, uint8_t dest, uint8_t dport, uint32_t timeout, uint32_t opts)
{
    return malloc(sizeof(csp_conn_t *));
}

int __wrap_csp_send(csp_conn_t *conn, csp_packet_t *packet, uint32_t timeout)
{
    return 1;
}

void * __wrap_csp_buffer_get(size)
{
    return malloc(size);
}

void __wrap_csp_buffer_free(void * buffer)
{
    if (buffer != NULL) free(buffer);
}