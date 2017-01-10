#include <cmocka/cmocka.h>
#include <csp/csp.h>

csp_packet_t * __wrap_csp_read(csp_conn_t * conn, uint32_t timeout)
{
    check_expected(conn);
    return csp_buffer_get(sizeof(csp_packet_t *));
}

int __wrap_csp_conn_dport(csp_conn_t * conn)
{
    return mock_type(int);
}

int __wrap_csp_conn_sport(csp_conn_t * conn)
{
    return mock_type(int);
}

void __wrap_csp_service_handler(csp_conn_t * conn, csp_packet_t * packet)
{
    if (packet != NULL) csp_buffer_free(packet);
}

csp_conn_t * __wrap_csp_accept(csp_socket_t * socket, int timeout)
{
    check_expected(socket);
    return mock_type(csp_conn_t *);
}

csp_socket_t * __wrap_csp_socket(uint32_t opts)
{
    return mock_type(csp_socket_t *);
}

int __wrap_csp_bind(csp_socket_t *socket, uint8_t port)
{
    check_expected(socket);
    return mock_type(int);
}

int __wrap_csp_listen(csp_socket_t *socket, size_t conn_queue_length)
{
    check_expected(socket);
    return mock_type(int);
}

csp_conn_t * __wrap_csp_connect(uint8_t prio, uint8_t dest, uint8_t dport, uint32_t timeout, uint32_t opts)
{
    return mock_type(csp_conn_t *);
}

int __wrap_csp_send(csp_conn_t *conn, csp_packet_t *packet, uint32_t timeout)
{
    int ret;
    check_expected(conn);
    check_expected(packet);
    ret = mock_type(int);
    if (ret)
    {
        csp_buffer_free(packet);
    }
    return ret;
}

void * __wrap_csp_buffer_get(size)
{
    return test_malloc(size + CSP_BUFFER_PACKET_OVERHEAD);
}

void __wrap_csp_buffer_free(void * buffer)
{
    if(buffer != NULL)
    {
        return test_free(buffer);
    }
}