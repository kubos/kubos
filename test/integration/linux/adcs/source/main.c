/*
 * Kubos Linux
 * Copyright (C) 2017 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * ADCS Integration Test for the ISIS iMTQ
 */

#include <imtq-api/imtq.h>
#include <errno.h>
#include <getopt.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

FILE * fp;

KADCSStatus noop()
{
    KADCSStatus status = ADCS_OK;

    status = k_adcs_noop();
    if (status != ADCS_OK)
    {
        fprintf(fp, "[No-op Test] Failed to run no-op command: %d\n", status);
        fprintf(stderr, "[No-op Test] Failed to run no-op command: %d\n", status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[No-op Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus reset()
{
    KADCSStatus status = ADCS_OK;

    status = k_adcs_reset(0);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Reset Test] Failed to reset ADCS: %d\n", status);
        fprintf(stderr, "[Reset Test] Failed to reset ADCS: %d\n", status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[Reset Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus mode()
{
    KADCSStatus status;
    adcs_mode_param param = 0;

    /* Detumble duration */
    param = 20;

    status = k_adcs_set_mode(DETUMBLE, &param);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Mode Test] Failed to enter detumble mode: %d\n",
                status);
        fprintf(stderr, "[Mode Test] Failed to enter detumble mode: %d\n",
                status);
        return ADCS_ERROR;
    }

    const struct timespec DELAY = {.tv_sec = 1, .tv_nsec = 0 };

    nanosleep(&DELAY, NULL);

    ADCSMode mode;

    status = k_adcs_get_mode(&mode);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Mode Test] Failed to get ADCS mode: %d\n", status);
        fprintf(stderr, "[Mode Test] Failed to get ADCS mode: %d\n", status);
        return ADCS_ERROR;
    }

    if (mode != DETUMBLE)
    {
        fprintf(
            fp,
            "[Mode Test] Failed to enter detumble mode. Current mode: %d\n",
            mode);
        fprintf(
            stderr,
            "[Mode Test] Failed to enter detumble mode. Current mode: %d\n",
            mode);
        return ADCS_ERROR;
    }

    /* Return to idle mode for other tests */
    status = k_adcs_set_mode(IDLE, NULL);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Mode Test] Failed to enter idle mode: %d\n", status);
        fprintf(stderr, "[Mode Test] Failed to enter idle mode: %d\n", status);
        return ADCS_ERROR;
    }

    nanosleep(&DELAY, NULL);

    fprintf(fp, "[Mode Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus run_test(ADCSTestType type)
{
    KADCSStatus status;

    adcs_test_results test = json_mkobject();

    status = k_adcs_run_test(type, test);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[%s Self-Test Test] Failed to run self-test: %d\n",
                type == 0 ? "All-Axes" : "Single-Axis", status);
        fprintf(stderr, "[%s Self-Test Test] Failed to run self-test: %d\n",
                type == 0 ? "All-Axes" : "Single-Axis", status);
        json_delete(test);
        return ADCS_ERROR;
    }

    /* Print the results to the file */
    char * temp = json_stringify(test, " ");

    fprintf(fp, "%s Self-Test Results: \n", type == 0 ? "All-Axes" : "Single-Axis");
    fputs(temp, fp);
    fprintf(fp, "\n");

    free(temp);
    json_delete(test);

    const struct timespec DELAY = {.tv_sec = 0, .tv_nsec = 300000001 };

    nanosleep(&DELAY, NULL);

    fprintf(fp, "[%s Self-Test Test] Test completed successfully\n", type == 0 ? "All-Axes" : "Single-Axis");

    return ADCS_OK;
}

KADCSStatus configure()
{
    KADCSStatus status;

    JsonNode * config = json_decode("{\"0x2003\": 1,   \"0x2004\": 2}");

    /* Test configuration */
    status = k_adcs_configure(config);
    json_delete(config);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Configure Test] Failed to configure ADCS: %d\n", status);
        fprintf(stderr, "[Configure Test] Failed to configure ADCS: %d\n", status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[Configure Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus passthrough()
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_TEMPS;

    imtq_coil_temp data = { 0 };

    status = k_adcs_passthrough(&cmd, 1, (uint8_t *) &data,
                                sizeof(imtq_coil_temp), NULL);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Passthrough Test] Failed to get iMTQ coil temperatures: %d\n", status);
        fprintf(stderr, "[Passthrough Test] Failed to get iMTQ coil temperatures: %d\n", status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[Passthrough test] Coil temps - X: %d, Y: %d, Z: %d\n",
                data.data.x, data.data.y, data.data.z);
    fprintf(fp, "[Passthrough Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus get_power()
{
    KADCSStatus       status;
    adcs_power_status uptime = 0;

    status = k_adcs_get_power_status(&uptime);
    if (status != ADCS_OK)
    {
        fprintf(fp, "[Power Test] Failed to get ADCS power status: %d\n", status);
        fprintf(stderr, "[Power Test] Failed to get ADCS power status: %d\n", status);
        return ADCS_ERROR;
    }

    if (uptime == 0)
    {
        fprintf(fp, "[Power Test] Test failed. ADCS appears to be offline\n");
        fprintf(stderr, "[Power Test] Test failed. ADCS appears to be offline\n");
        return ADCS_ERROR;
    }

    fprintf(fp, "[Power Test] Current uptime: %d\n", uptime);
    fprintf(fp, "[Power Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus get_orientation()
{
    KADCSStatus status;
    adcs_orient data;

    status = k_adcs_get_orientation(&data);
    if (status != ADCS_ERROR_NOT_IMPLEMENTED)
    {
        fprintf(fp, "[Orientation Test] Received unexpected ADCS orientation RC: %d\n",
                status);
        fprintf(stderr, "[Orientation Test] Received unexpected ADCS orientation RC: %d\n",
                status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[Orientation Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus get_spin()
{
    KADCSStatus status;
    adcs_spin data;

    status = k_adcs_get_spin(&data);
    if (status != ADCS_ERROR_NOT_IMPLEMENTED)
    {
        fprintf(fp, "[Spin Test] Received unexpected ADCS spin RC: %d\n", status);
        fprintf(stderr, "[Spin Test] Received unexpected ADCS spin RC: %d\n", status);
        return ADCS_ERROR;
    }

    fprintf(fp, "[Spin Test] Test completed successfully\n");

    return ADCS_OK;
}

KADCSStatus get_telemetry(ADCSTelemType type)
{
    KADCSStatus status;

    /* Make parent object */
    JsonNode * telem = json_mkobject();

    /* Get the data */
    status = k_adcs_get_telemetry(type, telem);
    if (status != ADCS_OK)
    {
        fprintf(fp,
                "[%s Telemetry Test] Error/s occurred while getting ADCS telemetry: %d\n",
                type == NOMINAL ? "Nominal" : "Debug", status);
        fprintf(stderr,
                "[%s Telemetry Test] Error/s occurred while getting ADCS telemetry: %d\n",
                type == NOMINAL ? "Nominal" : "Debug", status);
        return ADCS_ERROR;
    }

    /* Print the results to the file */
    char * temp = json_stringify(telem, " ");

    fprintf(fp, "%s Telemetry Results: \n", type == NOMINAL ? "Nominal" : "Debug");
    fputs(temp, fp);
    fprintf(fp, "\n");

    free(temp);
    json_delete(telem);

    fprintf(fp, "[%s Telemetry Test] Test completed successfully\n", type == NOMINAL ? "Nominal" : "Debug");

    return ADCS_OK;
}

int main(int argc, char * argv[])
{
    KADCSStatus status;

    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "k_adcs_init failed: %d\n", status);
        exit(-1);
    }

    fp = fopen("results.txt", "w");
    if (fp == NULL)
    {
        perror("Failed to open results.txt");
        /* Print the results to stdout instead */
        k_adcs_terminate();
        return -1;
    }

    fprintf(fp, "iMTQ ADCS Integration Test\n"
                "--------------------------\n\n");

    /* Start of integration tests */

    status = noop();

    status |= passthrough();

    status |= configure();

    status |= get_power();

    status |= mode();

    status |= run_test(TEST_ALL);

    status |= run_test(TEST_Z_POS);

    status |= get_telemetry(DEBUG);

    status |= get_telemetry(NOMINAL);

    status |= get_spin();

    status |= get_orientation();

    status |= reset();

    /* End of tests */

    k_adcs_terminate();

    fprintf(fp, "\niMTQ ADCS Integration Tests Complete\n"
                  "------------------------------------\n\n");

    if (status == ADCS_OK)
    {
        printf("ADCS tests completed successfully\n");
        fprintf(fp, "ADCS tests completed successfully\n");
    }
    else
    {
        fprintf(stderr, "One or more ADCS tests have failed. See results.txt for info\n");
        fprintf(fp, "One or more ADCS tests have failed\n");
    }

    fclose(fp);

    return 0;
}
