#include <ipc/pubsub.h>
#include <cmocka.h>

bool __wrap_kprv_subscriber_read(pubsub_conn * conn, void * buffer, int buffer_size, uint8_t port)
{
    check_expected(conn->conn_handle);
    check_expected(buffer);

    return mock_type(bool);
}

bool __wrap_kprv_subscriber_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
{
    if (conn != NULL)
    {
        conn->conn_handle = mock_type(csp_conn_t *);
    }
    return mock_type(bool);
}

bool __wrap_kprv_send_csp(pubsub_conn * conn, void * data, uint16_t length)
{
    check_expected(conn->conn_handle);
    check_expected(data);
    
    return mock_type(bool);
}

bool __wrap_kprv_server_accept(csp_socket_t * socket, pubsub_conn * conn)
{
    // Intentionally not expecting anything on socket
    // because it is a private variable and is initialized in a init function
    // that is not desirable to run around our tests
    check_expected(conn);
    conn->conn_handle = mock_type(csp_conn_t *);

    return mock_type(bool);
}

bool __wrap_kprv_publisher_read(pubsub_conn * conn, void * buffer, int buffer_size, uint8_t port)
{
    check_expected(conn->conn_handle);
    check_expected(buffer);

    return mock_type(bool);
}