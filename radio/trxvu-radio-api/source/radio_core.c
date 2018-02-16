/*
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
 */

#include <kubos-hal/i2c.h>
#include <radio-api/radio.h>
#include <stdio.h>
#include <unistd.h>

KRadioStatus k_radio_init()
{
    /*
     * All I2C configuration is done at the kernel level,
     * but we still need to pass a config structure to make
     * our I2C API happy.
     */
    KI2CConf conf = k_i2c_conf_defaults();

    KI2CStatus status;
    status = k_i2c_init(TRXVU_I2C_BUS, &conf);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to initialize radio: %d\n", status);
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

void k_radio_terminate()
{
    k_i2c_terminate(TRXVU_I2C_BUS);

    return;
}

/*
 * Calls the appropriate configuration functions based on what options
 * have actually been specified in the configuration structure.
 * All of these will have default values set by ISIS based on the options
 * sheet, so calling this function is not a requirement during the startup
 * process. Additionally, this function can be called at any point after
 * initialization to change the settings.
 */
KRadioStatus k_radio_configure(radio_config * config)
{
    KRadioStatus status = RADIO_OK;

    if (config == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    if (config->to.ascii[0] != 0)
    {
        status |= kprv_radio_tx_set_default_to(config->to);
    }
    if (config->from.ascii[0] != 0)
    {
        status |= kprv_radio_tx_set_default_from(config->from);
    }
    if (config->data_rate != 0)
    {
        status |= kprv_radio_tx_set_rate(config->data_rate);
    }
    if (config->idle != RADIO_IDLE_UNKNOWN)
    {
        status |= kprv_radio_tx_set_idle(config->idle);
    }
    if (config->beacon.len != 0)
    {
        status |= kprv_radio_tx_set_beacon(
            config->beacon.interval, config->beacon.msg, config->beacon.len);
    }

    return status;
}

KRadioStatus k_radio_watchdog_kick()
{
    KRadioStatus status;

    status = kprv_radio_tx_watchdog_kick();
    status |= kprv_radio_rx_watchdog_kick();

    return status;
}

void * kprv_radio_watchdog_thread(void * args)
{
    KRadioStatus status;

    while (1)
    {
        kprv_radio_tx_watchdog_kick();
        kprv_radio_rx_watchdog_kick();

        sleep(TRXVU_WD_TIMEOUT / 3);
    }

    return NULL;
}

pthread_t handle_watchdog = { 0 };

KRadioStatus k_radio_watchdog_start()
{
    if (handle_watchdog != 0)
    {
        fprintf(stderr, "TRXVU watchdog thread already started\n");
        return RADIO_OK;
    }

    if (TRXVU_WD_TIMEOUT == 0)
    {
        fprintf(
            stderr,
            "TRXVU watchdog has been disabled. No thread will be startd\n");
        return RADIO_OK;
    }

    if (pthread_create(&handle_watchdog, NULL, kprv_radio_watchdog_thread, NULL)
        != 0)
    {
        perror("Failed to create TRXVU watchdog thread");
        handle_watchdog = 0;
        return RADIO_ERROR;
    }

    return RADIO_OK;
}

KRadioStatus k_radio_watchdog_stop()
{
    /* Send the cancel request */
    if (pthread_cancel(handle_watchdog) != 0)
    {
        perror("Failed to cancel TRXVU watchdog thread");
        return RADIO_ERROR;
    }

    /* Wait for the cancellation to complete */
    if (pthread_join(handle_watchdog, NULL) != 0)
    {
        perror("Failed to rejoin TRXVU watchdog thread");
        return RADIO_ERROR;
    }

    handle_watchdog = 0;

    return RADIO_OK;
}

KRadioStatus k_radio_reset(KRadioReset type)
{
    KRadioStatus status;

    status = kprv_radio_rx_reset(type);
    status |= kprv_radio_tx_reset(type);

    return status;
}

KRadioStatus k_radio_get_telemetry(radio_telem * buffer, RadioTelemType type)
{
    if (buffer == NULL)
    {
        return RADIO_ERROR_CONFIG;
    }

    if (type >= RADIO_RX_TELEM_ALL)
    {
        return kprv_radio_rx_get_telemetry(buffer, type);
    }
    else
    {
        return kprv_radio_tx_get_telemetry(buffer, type);
    }
}
