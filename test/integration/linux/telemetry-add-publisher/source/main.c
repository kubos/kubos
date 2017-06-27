#include <telemetry/telemetry.h>
#include <csp/arch/csp_time.h>
#include <stdio.h>

#define QA_TEST_TOPIC 999

int main(void)
{
    telemetry_packet data = {
        .source.topic_id = QA_TEST_TOPIC,
        .source.subsystem_id = 0,
        .source.data_type = TELEMETRY_TYPE_INT,
        .data.i = 100,
        .timestamp = csp_get_ms()
    };

    if (telemetry_publish(data))
    {
        printf("Telemetry publish success\n");
    }
    else
    {
        printf("Telemetry publish failed\n");
    }

    return 0;
}