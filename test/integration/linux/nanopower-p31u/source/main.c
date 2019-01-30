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
 * Integration Test for the Gomspace P31u
 */

#include <gomspace-p31u-api.h>
#include <errno.h>
#include <getopt.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>
#include <endian.h>
#include <time.h>

FILE * fp;

KEPSStatus ping()
{
    KEPSStatus status = EPS_OK;

    status = k_eps_ping();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Ping Test] Failed to ping EPS: %d\n", status);
        fprintf(stderr, "[Ping Test] Failed to ping EPS: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Ping Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus get_heater()
{
    KEPSStatus status = EPS_OK;

    uint8_t bp4 = 0;
    uint8_t onboard = 0;

    status = k_eps_get_heater(&bp4, &onboard);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Get Heater Test] Failed to get EPS heaters status: %d\n", status);
        fprintf(stderr, "[Get Heater Test] Failed to get EPS heaters status: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "\nHeaters:\nbp4 - %s\nonboard - %s\n", bp4 ? "on" : "off", onboard ? "on" : "off");

    fprintf(fp, "[Get Heater Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus get_config()
{
    KEPSStatus status;

    /* System config */
    eps_system_config_t sys_config = {0};

    status = k_eps_get_system_config(&sys_config);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Get Config Test] Error/s occurred while getting EPS system config: %d\n", status);
        fprintf(stderr,
                "[Get Config Test] Error/s occurred while getting EPS system config: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "\nSystem Config:\n");
    fprintf(fp, "ppt_mode - %s\n", sys_config.ppt_mode > 1 ? "Fixed" : "Auto");
    fprintf(fp, "Heater mode - %s\n", sys_config.battheater_mode ? "Auto" : "Manual");
    fprintf(fp, "Heater low threshold - %d C\n", sys_config.battheater_low);
    fprintf(fp, "Heater high threshold - %d C\n", sys_config.battheater_high);
    fprintf(fp, "Normal mode outputs - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", sys_config.output_normal_value[i]);
    }
    fprintf(fp, "\nSafe mode outputs - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", sys_config.output_safe_value[i]);
    }
    fprintf(fp, "\nOutput on delays - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", sys_config.output_initial_on_delay[i]);
    }

    fprintf(fp, "\nOutput off delays - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", sys_config.output_initial_off_delay[i]);
    }
    fprintf(fp, "\nVBoost - %d %d %d\n", sys_config.vboost[0], sys_config.vboost[1], sys_config.vboost[2]);

    /* Battery config */
    eps_battery_config_t batt_config = {0};

    status = k_eps_get_battery_config(&batt_config);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Get Config Test] Error/s occurred while getting EPS battery config: %d\n", status);
        fprintf(stderr,
                "[Get Config Test] Error/s occurred while getting EPS battery config: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "\nBattery Config:\n");
    fprintf(fp, "Max voltage - %d\n", batt_config.batt_maxvoltage);
    fprintf(fp, "Normal voltage - %d\n", batt_config.batt_normalvoltage);
    fprintf(fp, "Safe voltage - %d\n", batt_config.batt_safevoltage);
    fprintf(fp, "Critical voltage - %d\n", batt_config.batt_criticalvoltage);

    fprintf(fp, "[Get Config Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus get_housekeeping()
{
    KEPSStatus status;

    eps_hk_t hk = {0};

    status = k_eps_get_housekeeping(&hk);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        fprintf(stderr,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "\nHousekeeping Data (HK2):\n");
    fprintf(fp, "VBoost - %d %d %d\n", hk.vboost[0], hk.vboost[1], hk.vboost[2]);
    fprintf(fp, "Battery voltage - %dmV\n", hk.vbatt);
    fprintf(fp, "Input currents - %d %d %d\n", hk.curin[0], hk.curin[1], hk.curin[2]);
    fprintf(fp, "Boost current - %d\n", hk.cursun);
    fprintf(fp, "Battery current - %d\n", hk.cursys);
    fprintf(fp, "Output currents - ");
    for (int i = 0; i < 6; i++)
    {
        fprintf(fp, "%d ", hk.curout[i]);
    }
    fprintf(fp, "\nOutput status - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", hk.output[i]);
    }
    fprintf(fp, "\nOutput time until on - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", hk.output_on_delta[i]);
    }
    fprintf(fp, "\nOutput time until off - ");
    for (int i = 0; i < 8; i++)
    {
        fprintf(fp, "%d ", hk.output_off_delta[i]);
    }
    fprintf(fp, "\nLatchups - ");
    for (int i = 0; i < 6; i++)
    {
        fprintf(fp, "%d ", hk.latchup[i]);
    }
    fprintf(fp, "\nWatchdog time left (I2C) - %d\n", hk.wdt_i2c_time_left);
    fprintf(fp, "Watchdog time left (GND) - %d\n", hk.wdt_gnd_time_left);
    fprintf(fp, "Watchdog pings left (CSP) - %d %d\n", hk.wdt_csp_pings_left[0], hk.wdt_csp_pings_left[1]);
    fprintf(fp, "Watchdog reboots (I2C) - %d\n", hk.counter_wdt_i2c);
    fprintf(fp, "Watchdog reboots (GND) - %d\n", hk.counter_wdt_gnd);
    fprintf(fp, "Watchdog reboots (CSP) - %d %d\n", hk.counter_wdt_csp[0], hk.counter_wdt_csp[1]);
    fprintf(fp, "System reboots (total) - %d\n", hk.counter_boot);
    fprintf(fp, "Temperatures - ");
    for (int i = 0; i < 6; i++)
    {
        fprintf(fp, "%d ", hk.temp[i]);
    }
    fprintf(fp, "\nCause of last reset - %d\n", hk.boot_cause);
    fprintf(fp, "Battery mode - %d\n", hk.batt_mode);
    fprintf(fp, "PPT mode - %d\n", hk.ppt_mode);

    fprintf(fp, "[Housekeeping Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus reboot()
{
    KEPSStatus status = EPS_OK;

    status = k_eps_reboot();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Reboot Test] Failed to reboot EPS: %d\n", status);
        fprintf(stderr, "[Reboot Test] Failed to reboot EPS: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Reboot Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus system_config()
{
    KEPSStatus status;

    eps_system_config_t current_config = {0};

    /* Reset config */
    status = k_eps_reset_system_config();
    if (status != EPS_OK)
    {
        fprintf(stderr, "[System Config Test] Failed to reset EPS system config: %d\n", status);
        fprintf(stderr, "[System Config Test] Failed to reset EPS system config: %d\n", status);
        return EPS_ERROR;
    }

    /* Set new config */
    eps_system_config_t new_config = {
            .ppt_mode = 0,
            .battheater_mode = 0,
            .battheater_low = -110,
            .battheater_high = 1,
            .output_normal_value = {1, 0, 1, 0, 1, 0, 1, 0},
            .output_safe_value = {0, 1, 0, 1, 0, 1, 0, 1},
            .output_initial_on_delay = {1,2,3,4,5,6,7,8},
            .output_initial_off_delay = {21,22,23,24,25,26,27,28},
            .vboost = {3600, 3600, 3600}
    };

    status = k_eps_configure_system(&new_config);
    if (status != EPS_OK)
    {
        fprintf(fp, "[System Config Test] Failed to configure EPS: %d\n", status);
        fprintf(stderr, "[System Config Test] Failed to configure EPS: %d\n", status);
        return EPS_ERROR;
    }

    /* Get current config */
    status = k_eps_get_system_config(&current_config);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[System Config Test] Error/s occurred while getting EPS system config: %d\n", status);
        fprintf(stderr,
                "[System Config Test] Error/s occurred while getting EPS system config: %d\n", status);
        return EPS_ERROR;
    }

    /* Make sure it matches what we set */
    if (memcmp(&current_config, &new_config, sizeof(current_config)) != 0)
    {
        fprintf(fp,
                "[System Config Test] Current system config doesn't match desired config: %d\n", status);
        fprintf(stderr,
                "[System Config Test] Current system config doesn't match desired config: %d\n", status);
        return EPS_ERROR;
    }

    return EPS_OK;
}

KEPSStatus battery_config()
{
    KEPSStatus status;

    /* Reset to default config */
    status = k_eps_reset_battery_config();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Battery Config Test] Failed to reset EPS battery config: %d\n", status);
        fprintf(stderr, "[Battery Config Test] Failed to reset EPS battery config: %d\n", status);
        return EPS_ERROR;
    }

    /* Set new config */
    eps_battery_config_t new_config = {
            .batt_maxvoltage = 8200,
            .batt_safevoltage = 7100,
            .batt_criticalvoltage = 6400,
            .batt_normalvoltage = 7300,
    };

    status = k_eps_configure_battery(&new_config);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Battery Config Test] Failed to configure EPS: %d\n", status);
        fprintf(stderr, "[Battery Config Test] Failed to configure EPS: %d\n", status);
        return EPS_ERROR;
    }

    /*
     * Wait for the new config to get saved into wherever in RAM
     */
    const struct timespec TRANSFER_DELAY
        = {.tv_sec = 0, .tv_nsec = 600000000 };

    nanosleep(&TRANSFER_DELAY, NULL);

    /* Save config to EEPROM */
    status = k_eps_save_battery_config();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Battery Config Test] Failed to save EPS configuration: %d\n", status);
        fprintf(stderr, "[Battery Config Test] Failed to save EPS configuration: %d\n", status);
        return EPS_ERROR;
    }

    /* Reboot the system to verify the save worked */
    status = k_eps_reboot();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Battery Config Test] Failed to reboot EPS: %d\n", status);
        fprintf(stderr, "[Battery Config Test] Failed to reboot EPS: %d\n", status);
        return EPS_ERROR;
    }

    /* Give it a sec to come back up */
    nanosleep(&TRANSFER_DELAY, NULL);

    /* Get current config */
    eps_battery_config_t current_config = { 0 };
    status = k_eps_get_battery_config(&current_config);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Battery Config Test] Error/s occurred while getting EPS battery config: %d\n", status);
        fprintf(stderr,
                "[Battery Config Test] Error/s occurred while getting EPS battery config: %d\n", status);
        return EPS_ERROR;
    }

    /* Make sure it matches what we set */
    if (memcmp(&current_config, &new_config, sizeof(current_config)) != 0)
    {
        fprintf(fp,
                "[Battery Config Test] Current battery config doesn't match desired config: %d\n", status);
        fprintf(stderr,
                "[Battery Config Test] Current battery config doesn't match desired config: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Battery Config Test] Test completed successfully\n");

    return EPS_OK;
}


KEPSStatus set_input()
{
    KEPSStatus status = EPS_OK;

    status = k_eps_set_input_mode(2);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Set Input Test] Failed to set EPS input mode: %d\n", status);
        fprintf(stderr, "[Set Input Test] Failed to set EPS input mode: %d\n", status);
        return EPS_ERROR;
    }

    const struct timespec TRANSFER_DELAY
        = {.tv_sec = 0, .tv_nsec = 600000000 };

    nanosleep(&TRANSFER_DELAY, NULL);

    status = k_eps_set_input_value(3000, 3000, 3000);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Set Input Test] Failed to set EPS input values: %d\n", status);
        fprintf(stderr, "[Set Input Test] Failed to set EPS input values: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Set Input Values Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus set_output()
{
    KEPSStatus status = EPS_OK;

    status = k_eps_set_output(0x44);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Set Outputs Test] Failed to set EPS outputs: %d\n", status);
        fprintf(stderr, "[Set Outputs Test] Failed to set EPS outputs: %d\n", status);
        return EPS_ERROR;
    }

    const struct timespec DELAY
        = {.tv_sec = 0, .tv_nsec = 500000000 };
    nanosleep(&DELAY, NULL);

    eps_hk_t hk = {0};

    status = k_eps_get_housekeeping(&hk);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        fprintf(stderr,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        return EPS_ERROR;
    }

    if (hk.output[2] != 1 || hk.output[5] != 1)
    {
        fprintf(fp, "[Set Outputs Test] Output check failed\n");
        fprintf(stderr, "[Set Outputs Test] Output check failed\n");
        return EPS_ERROR;
    }

    fprintf(fp, "[Set Outputs Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus single_output()
{
    KEPSStatus status = EPS_OK;

    status = k_eps_set_single_output(6, 1, 0);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Set Single Output Test] Failed to set single EPS output: %d\n", status);
        fprintf(stderr, "[Set Single Output Test] Failed to set single EPS output: %d\n", status);
        return EPS_ERROR;
    }

    const struct timespec DELAY
        = {.tv_sec = 0, .tv_nsec = 500000000 };
    nanosleep(&DELAY, NULL);

    eps_hk_t hk = {0};

    status = k_eps_get_housekeeping(&hk);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        fprintf(stderr,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        return EPS_ERROR;
    }

    if (hk.output[6] != 1)
    {
        fprintf(fp, "[Set Single Output Test] Output check failed\n");
        fprintf(stderr, "[Set Single Output Test] Output check failed\n");
        return EPS_ERROR;
    }

    fprintf(fp, "[Set Single Output Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus set_heater()
{
    KEPSStatus status = EPS_OK;

    /* cmd (0), heater {BP4, onboard, both}, on|off */
    status = k_eps_set_heater(0, 1, 1);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Set Heater Test] Failed to set EPS heater: %d\n", status);
        fprintf(stderr, "[Set Heater Test] Failed to set EPS heater: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Set Heater Test] Test completed successfully\n");

    return EPS_OK;
}

KEPSStatus reset_counters()
{
    KEPSStatus status = EPS_OK;

    eps_hk_t hk = {0};

    status = k_eps_get_housekeeping(&hk);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        fprintf(stderr,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        return EPS_ERROR;
    }

    uint32_t current_count = hk.counter_boot;

    const struct timespec DELAY
        = {.tv_sec = 0, .tv_nsec = 500000000 };
    nanosleep(&DELAY, NULL);

    status = k_eps_reset_counters();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Reset Counters Test] Failed to reset EPS counters: %d\n", status);
        fprintf(stderr, "[Reset Counters Test] Failed to reset EPS counters: %d\n", status);
        return EPS_ERROR;
    }

    nanosleep(&DELAY, NULL);

    status = k_eps_reboot();
    if (status != EPS_OK)
    {
        fprintf(fp, "[Reboot Test] Failed to reboot EPS: %d\n", status);
        fprintf(stderr, "[Reboot Test] Failed to reboot EPS: %d\n", status);
        return EPS_ERROR;
    }

    nanosleep(&DELAY, NULL);

    status = k_eps_get_housekeeping(&hk);
    if (status != EPS_OK)
    {
        fprintf(fp,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        fprintf(stderr,
                "[Housekeeping Test] Error/s occurred while getting EPS deploy status: %d\n", status);
        return EPS_ERROR;
    }

    uint32_t new_count = hk.counter_boot;

    if (new_count >= current_count)
    {
        fprintf(fp, "[Reset Counters Test] Failed to reset EPS counters\n");
        fprintf(stderr, "[Reset Counters Test] Failed to reset EPS counters\n");
        return EPS_ERROR;
    }

    fprintf(fp, "[Reset Counters Test] Test completed successfully\n");

    return EPS_OK;
}


KEPSStatus passthrough()
{
    uint8_t resp;

    KEPSStatus status;
    uint8_t    cmd = PING;

    status = k_eps_passthrough(&cmd, 1, &resp, 1);
    if (status != EPS_OK)
    {
        fprintf(fp, "[Passthrough Test] Failed to send EPS passthrough packet: %d\n", status);
        fprintf(stderr, "[Passthrough Test] Failed to send EPS passthrough packet: %d\n", status);
        return EPS_ERROR;
    }

    fprintf(fp, "[Passthrough Test] Test completed successfully\n");

    return EPS_OK;
}

int main(int argc, char * argv[])
{

    KEPSStatus status;
    KEPSConf config = {
            .bus = "/dev/i2c1-0",
            .addr = 0x02
    };

    status = k_eps_init(config);
    if (status != EPS_OK)
    {
        fprintf(stderr, "k_eps_init failed: %d\n", status);
        exit(-1);
    }

    fp = fopen("nanopower-results.txt", "w");
    if (fp == NULL)
    {
        perror("Failed to open nanopower-results.txt");
        k_eps_watchdog_stop();
        k_eps_terminate();
        return -1;
    }

    fprintf(fp, "GOMspace NanoPower P31u Integration Test\n"
                "----------------------------------------\n\n");

    status = k_eps_watchdog_start(3600);
    status |= ping();
    status |= get_config();
    status |= get_housekeeping();
    status |= get_heater();
    status |= set_heater();

    // status |= test_battery_config();

    const struct timespec INTERTEST_DELAY
        = {.tv_sec = 0, .tv_nsec = 20000000 };
    nanosleep(&INTERTEST_DELAY, NULL);

    // status |= test_system_config();

    nanosleep(&INTERTEST_DELAY, NULL);

    status |= passthrough();
    status |= reset_counters();

    nanosleep(&INTERTEST_DELAY, NULL);

    status |= set_input();
    status |= set_output();

    status |= single_output();

    status |= reboot();

    k_eps_watchdog_stop();
    k_eps_terminate();

    fprintf(fp, "\nGOMspace NanoPower P31u Integration Tests Complete\n"
                  "--------------------------------------------------\n\n");

    if (status == EPS_OK)
    {
        printf("NanoPower tests completed successfully\n");
        fprintf(fp, "NanoPower tests completed successfully\n");
    }
    else
    {
        fprintf(stderr, "One or more NanoPower tests have failed. See nanopower-results.txt for info\n");
        fprintf(fp, "One or more NanoPower tests have failed\n");
    }

    fclose(fp);

    return 0;
}
