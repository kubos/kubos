#include <ipc/pubsub.h>
#include <cmocka.h>

bool __wrap_subscriber_read(pubsub_conn conn, void * buffer, int buffer_size, uint8_t port)
{
    check_expected(conn.conn_handle);
    check_expected(buffer);

    return mock_type(bool);
}

bool __wrap_subscriber_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
{
    check_expected(conn);
    if (conn != NULL)
    {
        conn->conn_handle = mock_type(csp_conn_t *);
    }
    return mock_type(bool);
}

bool __wrap_send_csp(pubsub_conn conn, void * data, uint16_t length)
{
    check_expected(conn.conn_handle);
    check_expected(data);
    
    return mock_type(bool);
}