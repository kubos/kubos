/*
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
 */

#include <ants-api/ants-api.h>
#include <kubos-hal/i2c.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

/* Address of the antenna microcontroller commands should be issued against */
uint8_t ants_addr;

/*
 * The system can lock up if you make too many calls too quickly,
 * so we're adding a small delay for safety.
 */
const struct timespec TRANSFER_DELAY = {.tv_sec = 0, .tv_nsec = 1000001 };

KANTSStatus k_ants_init()
{
    /*
     * All I2C configuration is done at the kernel level,
     * but we still need to pass a config structure to make
     * our I2C API happy.
     */
    KI2CConf conf = k_i2c_conf_defaults();

    KI2CStatus status;
    status = k_i2c_init(ANTS_I2C_BUS, &conf);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to initialize AntS: %d\n", status);
        return ANTS_ERROR;
    }

    /* Set default I2C slave address */
    ants_addr = ANTS_PRIMARY;

    return ANTS_OK;
}

void k_ants_terminate()
{
    k_i2c_terminate(ANTS_I2C_BUS);

    return;
}

KANTSStatus k_ants_configure(KANTSController config)
{
    KANTSStatus status = ANTS_OK;

    if (config == PRIMARY)
    {
        ants_addr = ANTS_PRIMARY;
    }
    else if (config == SECONDARY)
    {
        if (ANTS_SECONDARY == 0x00)
        {
            fprintf(stderr, "AntS config failed: Secondary I2C target is not "
                            "available\n");
        }
        else
        {
            ants_addr = ANTS_SECONDARY;
        }
    }
    else
    {
        fprintf(stderr, "AntS config failed: Unknown value - %d\n", config);
        return ANTS_ERROR_CONFIG;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return status;
}

KANTSStatus k_ants_reset()
{
    KANTSStatus ret = ANTS_OK;
    KI2CStatus  status;
    uint8_t     cmd = SYSTEM_RESET;

    status = k_i2c_write(ANTS_I2C_BUS, ANTS_PRIMARY, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reset primary AntS controller: %d\n",
                status);
        ret = ANTS_ERROR;
    }

#if ANTS_SECONDARY != 0
    status = k_i2c_write(ANTS_I2C_BUS, ANTS_SECONDARY, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reset secondary AntS controller: %d\n",
                status);
        ret = ANTS_ERROR;
    }
#endif

    nanosleep(&TRANSFER_DELAY, NULL);

    return ret;
}

KANTSStatus k_ants_arm()
{
    KI2CStatus status;
    uint8_t    cmd = ARM_ANTS;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to arm AntS: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_disarm()
{
    KI2CStatus status;
    uint8_t    cmd = DISARM_ANTS;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to disarm AntS: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_deploy(KANTSAnt antenna, bool override, uint8_t timeout)
{
    KI2CStatus status    = ANTS_OK;
    char       packet[2] = { 0 };

    packet[1] = timeout;

    switch (antenna)
    {
        case ANT_1:
            if (override == true)
            {
                packet[0] = DEPLOY_1_OVERRIDE;
            }
            else
            {
                packet[0] = DEPLOY_1;
            }
            break;
#if ANT_COUNT > 1
        case ANT_2:
            if (override)
            {
                packet[0] = DEPLOY_2_OVERRIDE;
            }
            else
            {
                packet[0] = DEPLOY_2;
            }
            break;
#endif
#if ANT_COUNT > 2
        case ANT_3:
            if (override)
            {
                packet[0] = DEPLOY_3_OVERRIDE;
            }
            else
            {
                packet[0] = DEPLOY_3;
            }
            break;
#endif
#if ANT_COUNT > 3
            if (override)
            {
                packet[0] = DEPLOY_4_OVERRIDE;
            }
            else
            {
                packet[0] = DEPLOY_4;
            }
            break;
#endif
        default:
            fprintf(stderr, "Unknown AntS antenna: %d\n", antenna);
            return ANTS_ERROR_CONFIG;
    }

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to deploy antenna %d: %d\n", (antenna + 1),
                status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_auto_deploy(uint8_t timeout)
{
    KI2CStatus status    = ANTS_OK;
    char       packet[2] = { 0 };

    packet[0] = AUTO_DEPLOY;
    packet[1] = timeout;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to auto-deploy AntS: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_cancel_deploy()
{
    KI2CStatus status;
    uint8_t    cmd = CANCEL_DEPLOY;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to cancel AntS deployment: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_get_deploy_status(uint16_t * resp)
{
    if (resp == NULL)
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;
    uint8_t    cmd = GET_STATUS;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request AntS deployment status: %d\n",
                status);
        return ANTS_ERROR;
    }

    status = k_i2c_read(ANTS_I2C_BUS, ants_addr, (uint8_t *) resp, 2);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read AntS deployment status: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_get_uptime(uint32_t * uptime)
{
    if (uptime == NULL)
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;
    uint8_t    cmd = GET_UPTIME_SYS;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request AntS uptime: %d\n", status);
        return ANTS_ERROR;
    }

    status = k_i2c_read(ANTS_I2C_BUS, ants_addr, (uint8_t *) uptime, 4);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read AntS uptime: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_get_system_telemetry(ants_telemetry * telem)
{
    if (telem == NULL)
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;
    uint8_t    cmd = GET_TELEMETRY;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request AntS telemetry: %d\n", status);
        return ANTS_ERROR;
    }

    status = k_i2c_read(ANTS_I2C_BUS, ants_addr, (uint8_t *) telem,
                        sizeof(ants_telemetry));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read AntS telemetry: %d\n", status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_get_activation_count(KANTSAnt antenna, uint8_t * count)
{
    KANTSStatus ret = ANTS_OK;

    if (count == NULL)
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;
    uint8_t    cmd = GET_COUNT_1 + antenna;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request antenna %d activation count: %d\n",
                (antenna + 1), status);
        return ANTS_ERROR;
    }

    status = k_i2c_read(ANTS_I2C_BUS, ants_addr, count, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read antenna %d activation count: %d\n",
                (antenna + 1), status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_get_activation_time(KANTSAnt antenna, uint16_t * time)
{
    KANTSStatus ret = ANTS_OK;

    if (time == NULL)
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;
    uint8_t    cmd = GET_UPTIME_1 + antenna;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to request antenna %d activation times: %d\n",
                (antenna + 1), status);
        return ANTS_ERROR;
    }

    status = k_i2c_read(ANTS_I2C_BUS, ants_addr, (uint8_t *) time, 2);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read antenna %d activation times: %d\n",
                (antenna + 1), status);
        return ANTS_ERROR;
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}

KANTSStatus k_ants_watchdog_kick()
{
    KI2CStatus  status;
    KANTSStatus ret = ANTS_OK;
    uint8_t     cmd = WATCHDOG_RESET;

    status = k_i2c_write(ANTS_I2C_BUS, ANTS_PRIMARY, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to kick AntS primary watchdog: %d\n", status);
        ret = ANTS_ERROR;
    }

#if ANTS_SECONDARY != 0
    status = k_i2c_write(ANTS_I2C_BUS, ANTS_SECONDARY, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to kick AntS redundant watchdog: %d\n", status);
        ret = ANTS_ERROR;
    }
#endif

    return ret;
}

pthread_t handle_watchdog = { 0 };

void * kprv_ants_watchdog_thread(void * args)
{
    KANTSStatus status;

    while (1)
    {
        k_ants_watchdog_kick();

        sleep(ANTS_WD_TIMEOUT / 3);
    }

    return NULL;
}

KANTSStatus k_ants_watchdog_start()
{
    if (handle_watchdog != 0)
    {
        fprintf(stderr, "AntS watchdog thread already started\n");
        return ANTS_OK;
    }

    if (ANTS_WD_TIMEOUT == 0)
    {
        fprintf(
            stderr,
            "AntS watchdog has been disabled. No thread will be started\n");
        return ANTS_OK;
    }

    if (pthread_create(&handle_watchdog, NULL, kprv_ants_watchdog_thread, NULL)
        != 0)
    {
        perror("Failed to create AntS watchdog thread");
        handle_watchdog = 0;
        return ANTS_ERROR;
    }

    return ANTS_OK;
}

KANTSStatus k_ants_watchdog_stop()
{
    /* Send the cancel request */
    if (pthread_cancel(handle_watchdog) != 0)
    {
        perror("Failed to cancel AntS watchdog thread");
        return ANTS_ERROR;
    }

    /* Wait for the cancellation to complete */
    if (pthread_join(handle_watchdog, NULL) != 0)
    {
        perror("Failed to rejoin AntS watchdog thread");
        return ANTS_ERROR;
    }

    handle_watchdog = 0;

    return ANTS_OK;
}

KANTSStatus k_ants_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx,
                               int rx_len)
{
    if (tx == NULL || tx_len < 1 || (rx == NULL && rx_len != 0))
    {
        return ANTS_ERROR_CONFIG;
    }

    KI2CStatus status;

    status = k_i2c_write(ANTS_I2C_BUS, ants_addr, (uint8_t *) tx, tx_len);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send AntS passthrough packet: %d\n", status);
        return ANTS_ERROR;
    }

    if (rx_len != 0)
    {
        status = k_i2c_read(ANTS_I2C_BUS, ants_addr, rx, rx_len);
        if (status != I2C_OK)
        {
            fprintf(stderr, "Failed to read AntS passthrough response: %d\n",
                    status);
            return ANTS_ERROR;
        }
    }

    nanosleep(&TRANSFER_DELAY, NULL);

    return ANTS_OK;
}
