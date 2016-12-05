#include "telemetry/telemetry.h"
#include "telemetry/config.h"

#define TRUE 1
#define FALSE 0

static csp_socket_t * socket;

static void conn_connect_csp(telemetry_conn * conn);


void conn_send_request(telemetry_conn conn, telemetry_request_t request);
static uint8_t conn_send_request_csp(telemetry_conn conn, telemetry_request_t request);

// void conn_read_request(telemetry_conn conn, telemetry_request_t request);
static telemetry_request_t conn_read_request_csp(telemetry_conn conn);

void telemetry_conn_init()
{
    csp_buffer_init(10, 300);

    /* Init CSP with address MY_ADDRESS */
    csp_init(TELEMETRY_CSP_ADDRESS);

    /* Start router task with 500 word stack, OS task priority 1 */
    csp_route_start_task(500, 1);

    TELEMETRY_THREADS;
}

uint8_t conn_server_setup()
{
    if ((socket = csp_socket(CSP_SO_NONE)) == NULL)
    {
        return FALSE;
    }

    if (csp_bind(socket, TELEMETRY_CSP_PORT) != CSP_ERR_NONE)
    {
        return FALSE;
    }

    if (csp_listen(socket, TELEMETRY_NUM_DESTINATIONS) != CSP_ERR_NONE)
    {
        return FALSE;
    }

    return TRUE;
}

uint8_t conn_server_accept(telemetry_conn * conn)
{
    csp_conn_t * csp_conn;
    if ((csp_conn = csp_accept(socket, 1000)) != NULL)
    {
        conn->conn_handle = csp_conn;
        return TRUE;
    }
    return FALSE;
}

telemetry_conn conn_connect(uint8_t sources)
{
    telemetry_conn conn = {
        .conn_handle = NULL
    };
    
    conn_connect_csp(&conn);
    if (conn.conn_handle != NULL)
    {
        telemetry_request_t request = {
            .sources = sources
        };
        conn_send_request_csp(conn, request);
    }

    return conn;
}

uint8_t conn_accept(telemetry_conn * conn)
{
    csp_conn_t * csp_conn = NULL;
    if (conn != NULL)
    {

    }
}


static void conn_connect_csp(telemetry_conn * conn)
{
    csp_conn_t * csp_conn = NULL;

    csp_ping(TELEMETRY_CSP_ADDRESS, 100, 100, CSP_O_NONE);
    csp_conn = csp_connect(CSP_PRIO_NORM, TELEMETRY_CSP_ADDRESS, TELEMETRY_CSP_PORT, 1000, CSP_O_NONE);
    conn->conn_handle = csp_conn;
}

uint8_t conn_send_packet(telemetry_conn conn, telemetry_packet packet)
{
    csp_packet_t * csp_packet;
    csp_conn_t * csp_conn = conn.conn_handle;
    if (csp_conn != NULL)
    {
        csp_packet = csp_buffer_get(20);
        if (csp_packet != NULL)
        {
            memcpy(csp_packet->data, &packet, sizeof(packet));
            csp_packet->length = sizeof(packet);
            if (!csp_send(csp_conn, csp_packet, 1000))
            {
                return FALSE;
            }
            else
            {
                return TRUE;
            }
        }
    }
}

static uint8_t conn_send_request_csp(telemetry_conn conn, telemetry_request_t request)
{
    csp_packet_t * csp_packet;
    csp_conn_t * csp_conn = conn.conn_handle;
    if (csp_conn != NULL)
    {
        csp_packet = csp_buffer_get(20);
        if (csp_packet != NULL)
        {
            memcpy(csp_packet->data, &request, sizeof(request));
            csp_packet->length = sizeof(request);
            if (!csp_send(csp_conn, csp_packet, 1000))
            {
                return FALSE;
            }
            else
            {
                return TRUE;
            }
        }
    }
}

telemetry_request_t conn_server_read_request(telemetry_conn conn)
{
    csp_packet_t * csp_packet;
    telemetry_request_t request;
    csp_conn_t * csp_conn = conn.conn_handle;
    if (csp_conn != NULL)
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            switch(csp_conn_dport(csp_conn))
            {
                case TELEMETRY_CSP_PORT:
                    memcpy(&request, (telemetry_request_t*)csp_packet->data, sizeof(request));
                    csp_buffer_free(csp_packet);
                    return request;
                default:
                    csp_service_handler(csp_conn, csp_packet);
            }
        }
    }
    return request;
}

uint8_t conn_client_read_packet(telemetry_conn conn, telemetry_packet * packet)
{
    csp_packet_t * csp_packet;
    csp_conn_t * csp_conn = conn.conn_handle;
    if (csp_conn != NULL)
    {
        if ((csp_packet = csp_read(csp_conn, 1000)) != NULL)
        {
            switch(csp_conn_sport(csp_conn))
            {
                case TELEMETRY_CSP_PORT:
                    memcpy(packet, (telemetry_packet*)csp_packet->data, sizeof(telemetry_packet));
                    csp_buffer_free(csp_packet);
                    return TRUE;
                default:
                    csp_service_handler(csp_conn, csp_packet);
                    break;
            }
        }
    }
    return FALSE;
}