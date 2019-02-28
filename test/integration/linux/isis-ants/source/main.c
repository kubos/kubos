/*
 * Kubos Linux
 * Copyright (C) 2018 Kubos Corporation
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
 * ANTS Integration Test for the ISIS iMTQ
 */

#include <ants-api.h>
#include <errno.h>
#include <getopt.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>

FILE * fp;

#define BUS             "/dev/i2c-0"
#define PRIMARY_ADDR    0x31
#define SECONDARY_ADDR  0x32
#define ANT_COUNT       4
#define TIMEOUT         10

KANTSStatus reset()
{
    KANTSStatus status = ANTS_OK;

    status = k_ants_reset();
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Reset Test] Failed to reset ANTS: %d\n", status);
        fprintf(stderr, "[Reset Test] Failed to reset ANTS: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Reset Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus configure()
{
    KANTSStatus status;

    status = k_ants_configure(SECONDARY);
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Configure Test] Failed to configure ANTS: %d\n", status);
        fprintf(stderr, "[Configure Test] Failed to configure ANTS: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Configure Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus deploy(KANTSAnt antenna, bool override, uint8_t time)
{
    KANTSStatus status;

    /* Get the data */
    status = k_ants_deploy(antenna, override, time);
    if (status != ANTS_OK)
    {
        fprintf(fp,
                "[Deploy %d %sTest] Error/s occurred while deploying antenna/s: %d\n",
                antenna, override ? "Override " : "", status);
        fprintf(stderr,
                "[Deploy %d %sTest] Error/s occurred while deploying antenna/s: %d\n",
                antenna, override ? "Override " : "", status);
        return ANTS_ERROR;
    }

    /* Give it a moment to run so we have meaningful data later when we get the telemetry */
    sleep(1);

    fprintf(fp, "[Deploy %d %sTest] Test completed successfully\n", antenna, override ? "Override " : "");

    return ANTS_OK;
}

KANTSStatus auto_deploy()
{
    KANTSStatus status;

    uint8_t time = 3;

    status = k_ants_auto_deploy(time);
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Auto-Deploy Test] Failed to auto-deploy antennas: %d\n", status);
        fprintf(stderr, "[Auto-Deploy Test] Failed to auto-deploy antennas: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Auto-Deploy Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus cancel_deploy()
{
    KANTSStatus status;

    status = k_ants_cancel_deploy();
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Cancel Deploy Test] Failed to cancel AntS deployment: %d\n", status);
        fprintf(stderr, "[Cancel Deploy Test] Failed to cancel AntS deployment: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Cancel Deploy Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus arm()
{
    KANTSStatus status = ANTS_OK;

    status = k_ants_arm();
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Arm Test] Failed to arm ANTS: %d\n", status);
        fprintf(stderr, "[Arm Test] Failed to arm ANTS: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Arm Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus disarm()
{
    KANTSStatus status = ANTS_OK;

    status = k_ants_disarm();
    if (status != ANTS_OK)
    {
        fprintf(fp, "[Disarm Test] Failed to disarm ANTS: %d\n", status);
        fprintf(stderr, "[Disarm Test] Failed to disarm ANTS: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Disarm Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus get_uptime()
{
    KANTSStatus status;

    uint32_t uptime;

    /* Get the data */
    status = k_ants_get_uptime(&uptime);
    if (status != ANTS_OK)
    {
        fprintf(fp,
                "[Uptime Test] Error/s occurred while getting AntS uptime: %d\n", status);
        fprintf(stderr,
                "[Uptime Test] Error/s occurred while getting AntS uptime: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "System Uptime: %d\n", uptime);

    fprintf(fp, "[Uptime Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus get_deploy()
{
    KANTSStatus status;

    uint16_t deploy;

    status = k_ants_get_deploy_status(&deploy);
    if (status != ANTS_OK)
    {
        fprintf(fp,
                "[Deploy Status Test] Error/s occurred while getting AntS deploy status: %d\n", status);
        fprintf(stderr,
                "[Deploy Status Test] Error/s occurred while getting AntS deploy status: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "Deploy Status: %#X\n", deploy);

    fprintf(fp, "[Deploy Status Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus passthrough()
{
    KANTSStatus status;
    uint8_t    cmd = GET_STATUS;
    uint16_t resp;

    status = k_ants_passthrough(&cmd, 1, (uint8_t *) &resp, 2);
    if (status != ANTS_OK)
    {
        fprintf(fp, "Failed to read AntS deployment status: %d\n", status);
        fprintf(stderr, "Failed to read AntS deployment status: %d\n", status);
        return ANTS_ERROR;
    }

    fprintf(fp, "[Passthrough Test] Result: %#x\n", resp);

    return ANTS_OK;
}

KANTSStatus get_system_telemetry()
{
    KANTSStatus status;

    ants_telemetry telem;

    /* Get the data */
    status = k_ants_get_system_telemetry(&telem);
    if (status != ANTS_OK)
    {
        fprintf(fp,
                "[System Telemetry Test] Error/s occurred while getting AntS system telemetry: %d\n",
                status);
        fprintf(stderr,
                "[System Telemetry Test] Error/s occurred while getting AntS system telemetry: %d\n",
                status);
        return ANTS_ERROR;
    }

    fprintf(fp, "System Temp (raw): %d\n", telem.raw_temp);
    fprintf(fp, "Deploy Status: %#x\n", telem.deploy_status);
    fprintf(fp, "System Uptime: %d\n", telem.uptime);

    fprintf(fp, "[System Telemetry Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus get_activation_counts()
{
    KANTSStatus status;

    uint8_t count;

    for(int i = 0; i < ANT_COUNT; i++)
    {
        /* Get the data */
        status = k_ants_get_activation_count(i, &count);
        if (status != ANTS_OK)
        {
            fprintf(fp,
                    "[Activation Counts Test] Error/s occurred while getting antenna %d activation count: %d\n",
                    (i + 1), status);
            fprintf(stderr,
                    "[Activation Counts Test] Error/s occurred while getting antenna %d activation count: %d\n",
                    (i + 1), status);
            status = ANTS_ERROR;
            continue;
        }

        fprintf(fp, "Antenna %d activation count: %d\n", (i + 1), count);
    }
    fprintf(fp, "[Activation Counts Test] Test completed successfully\n");

    return ANTS_OK;
}

KANTSStatus get_activation_times()
{
    KANTSStatus status;

    uint16_t time;

    for(int i = 0; i < ANT_COUNT; i++)
    {
        /* Get the data */
        status = k_ants_get_activation_time(i, &time);
        if (status != ANTS_OK)
        {
            fprintf(fp,
                    "[Activation Times Test] Error/s occurred while getting antenna %d activation time: %d\n",
                    (i + 1), status);
            fprintf(stderr,
                    "[Activation Times Test] Error/s occurred while getting antenna %d activation time: %d\n",
                    (i + 1), status);
            status = ANTS_ERROR;
            continue;
        }

        fprintf(fp, "Antenna %d activation time: %d\n", (i + 1), time);
    }
    fprintf(fp, "[Activation Times Test] Test completed successfully\n");

    return ANTS_OK;
}

int main(int argc, char * argv[])
{

    KANTSStatus status;

    status = k_ants_init(BUS, PRIMARY_ADDR, SECONDARY_ADDR, ANT_COUNT, TIMEOUT);
    if (status != ANTS_OK)
    {
        fprintf(stderr, "k_ants_init failed: %d\n", status);
        exit(-1);
    }

    fp = fopen("ants-results.txt", "w");
    if (fp == NULL)
    {
        perror("Failed to open ants-results.txt");
        k_ants_watchdog_stop();
        k_ants_terminate();
        return -1;
    }

    fprintf(fp, "ISIS AntS Integration Test\n"
                "--------------------------\n\n");

    status = k_ants_watchdog_start();

    status |= arm();
    status |= disarm();
    status |= configure();
    status |= arm(); /* System needs to be armed to run deploy commands successfully */
    status |= deploy(ANT_3, false, 1);
    status |= deploy(ANT_1, true, 1);
    status |= auto_deploy();
    status |= cancel_deploy();
    status |= passthrough();
    status |= get_deploy();
    status |= get_system_telemetry();
    status |= get_activation_counts();
    status |= get_activation_times();
    status |= get_uptime();
    status |= reset();

    k_ants_terminate();

    fprintf(fp, "\nISIS AntS Integration Tests Complete\n"
                  "------------------------------------\n\n");

    if (status == ANTS_OK)
    {
        printf("AntS tests completed successfully\n");
        fprintf(fp, "AntS tests completed successfully\n");
    }
    else
    {
        fprintf(stderr, "One or more AntS tests have failed. See ant-results.txt for info\n");
        fprintf(fp, "One or more AntS tests have failed\n");
    }

    fclose(fp);

    return 0;
}
