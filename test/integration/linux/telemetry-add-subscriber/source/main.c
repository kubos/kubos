#include <telemetry/telemetry.h>
#include <csp/arch/csp_time.h>
#include <stdio.h>

#define QA_TEST_TOPIC 101

int main(void)
{
    telemetry_packet data = {
        .source.topic_id = QA_TEST_TOPIC,
        .source.subsystem_id = 0,
        .source.data_type = TELEMETRY_TYPE_INT,
        .data.i = 100,
        .timestamp = csp_get_ms()
    };

    socket_conn conn;

    if (!telemetry_connect(&conn))
    {
        printf("Telemetry connect failed\n");
        return 1;
    }

    if (!telemetry_subscribe(&conn, QA_TEST_TOPIC))
    {
        printf("Telemetry subscribe failed\n");
        return 1;
    }

    for (uint8_t i = 0; i < 15; i++)
    {
        telemetry_packet packet;
        if (telemetry_read(&conn, &packet))
        {
            printf("Telemetry subscribe success\n");
            telemetry_unsubscribe(&conn, QA_TEST_TOPIC);
            telemetry_disconnect(&conn);
            return 1;
        }
        sleep(1);
    }

    printf("Telemetry subscribe failed\n");
    telemetry_unsubscribe(&conn, QA_TEST_TOPIC);
    telemetry_disconnect(&conn);

    return 0;
}