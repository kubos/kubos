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

#include <nanopower-api/nanopower-api.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

static uint8_t eps_bus = 0;
static uint8_t eps_addr = 0;

KEPSStatus k_eps_init(KEPSConf config)
{
    if (config.bus == K_I2C_NO_BUS || config.addr == 0)
    {
        return EPS_ERROR_CONFIG;
    }

    if (eps_bus != 0)
    {
        fprintf(stderr, "EPS already initialized. Ignoring request\n");
        return EPS_ERROR;
    }

    eps_bus = config.bus;
    eps_addr = config.addr;

    /*
     * All I2C configuration is done at the kernel level,
     * but we still need to pass a config structure to make
     * our I2C API happy.
     */
    KI2CConf conf = k_i2c_conf_defaults();

    KI2CStatus status;
    status = k_i2c_init(eps_bus, &conf);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to initialize EPS: %d\n", status);
        return EPS_ERROR;
    }

    return EPS_OK;
}

void k_eps_terminate()
{
    k_i2c_terminate(eps_bus);

    eps_bus = 0;
    eps_addr = 0;

    return;
}

KEPSStatus k_eps_ping()
{
    KI2CStatus status;
    uint8_t    cmd  = PING;
    uint8_t    resp = 0;

    status = k_i2c_write(eps_bus, eps_addr, &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send EPS ping: %d\n", status);
        return EPS_ERROR;
    }

    status = k_i2c_read(eps_bus, eps_addr, &resp, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to get EPS ping response: %d\n", status);
        return EPS_ERROR;
    }

    if (resp != cmd)
    {
        fprintf(stderr, "Unexpected EPS ping response: %#x vs %#x\n", cmd,
                resp);
        return EPS_ERROR;
    }

    return EPS_OK;
}

KEPSStatus k_eps_reset()
{
    KI2CStatus status;
    uint8_t    cmd = HARD_RESET;

    status = k_i2c_write(eps_bus, eps_addr, (uint8_t *) &cmd, 1);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reset EPS: %d\n", status);
        return EPS_ERROR;
    }

    return EPS_OK;
}

KEPSStatus k_eps_reboot()
{
    KI2CStatus status;
    uint8_t    packet[] = { REBOOT, 0x80, 0x07, 0x80, 0x07 };

    status = k_i2c_write(eps_bus, eps_addr, packet, sizeof(packet));
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to reboot EPS: %d\n", status);
        return EPS_ERROR;
    }

    return EPS_OK;
}

KEPSStatus k_eps_configure_system(const eps_system_config_t * config)
{
    KEPSStatus status = EPS_OK;
    eps_resp_header response;
    typedef struct __attribute__((packed))
    {
        uint8_t cmd;
        eps_system_config_t sys_config;
    }  config_packet;

    config_packet packet = { 0 };

    if (config == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    packet.cmd = SET_CONFIG1;

    packet.sys_config.ppt_mode = config->ppt_mode;
    packet.sys_config.battheater_mode = config->battheater_mode;
    packet.sys_config.battheater_low = config->battheater_low;
    packet.sys_config.battheater_high = config->battheater_high;
    for (int i = 0; i < 8; i++)
    {
        packet.sys_config.output_normal_value[i] = config->output_normal_value[i];
    }
    for (int i = 0; i < 8; i++)
    {
        packet.sys_config.output_safe_value[i] = config->output_safe_value[i];
    }
    for (int i = 0; i < 8; i++)
    {
        packet.sys_config.output_initial_on_delay[i] = htobe16(config->output_initial_on_delay[i]);
    }
    for (int i = 0; i < 8; i++)
    {
        packet.sys_config.output_initial_off_delay[i] = htobe16(config->output_initial_off_delay[i]);
    }
    packet.sys_config.vboost[0] = htobe16(config->vboost[0]);
    packet.sys_config.vboost[1] = htobe16(config->vboost[1]);
    packet.sys_config.vboost[2] = htobe16(config->vboost[2]);

    status = kprv_eps_transfer((uint8_t *) &packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS system configuration: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_configure_battery(const eps_battery_config_t * config)
{
    KEPSStatus status = EPS_OK;
    eps_resp_header response;
    typedef struct __attribute__((packed))
    {
        uint8_t cmd;
        eps_battery_config_t batt_config;
    }  config_packet;

    config_packet packet = { 0 };

    if (config == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    packet.cmd = SET_CONFIG2;
    packet.batt_config.batt_maxvoltage = htobe16(config->batt_maxvoltage);
    packet.batt_config.batt_safevoltage = htobe16(config->batt_safevoltage);
    packet.batt_config.batt_criticalvoltage = htobe16(config->batt_criticalvoltage);
    packet.batt_config.batt_normalvoltage = htobe16(config->batt_normalvoltage);

    status = kprv_eps_transfer((uint8_t *) &packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS battery configuration: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_save_battery_config()
{
    KEPSStatus status;
    uint8_t packet[] = { CMD_CONFIG2, 2 };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to reset EPS battery configuration: %d\n", status);
        return status;
    }

    return EPS_OK;
}


KEPSStatus k_eps_set_output(uint8_t channel_mask)
{
    KEPSStatus      status;
    uint8_t         packet[] = { SET_OUTPUT, channel_mask };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS outputs: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_set_single_output(uint8_t channel, uint8_t value, int16_t delay)
{
    KEPSStatus status;
    eps_resp_header response;
    struct __attribute__((packed))
    {
        uint8_t cmd;
        uint8_t channel;
        uint8_t value;
        int16_t delay;
    }  packet;

    if (channel > 7 || value > 1)
    {
        return EPS_ERROR_CONFIG;
    }

    packet.cmd = SET_SINGLE_OUTPUT;
    /*
     * The channel ordering is secretly backwards.
     * Output[0] is actually channel 7 (onboard heater)
     * and output[7] is channel 0
     */
    packet.channel = 7 - channel;
    packet.value = value;
    packet.delay = htobe16(delay);

    status = kprv_eps_transfer((uint8_t *) &packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS output %d value: %d\n", channel, status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_set_input_value(uint16_t in1_voltage, uint16_t in2_voltage,
                                 uint16_t in3_voltage)
{
    KEPSStatus status   = EPS_OK;
    eps_resp_header response;
    struct __attribute__((packed))
    {
        uint8_t cmd;
        uint16_t in1_voltage;
        uint16_t in2_voltage;
        uint16_t in3_voltage;
    }  packet;

    packet.cmd = SET_PV_VOLT;
    packet.in1_voltage = htobe16(in1_voltage);
    packet.in2_voltage = htobe16(in2_voltage);
    packet.in3_voltage = htobe16(in3_voltage);

    status = kprv_eps_transfer((uint8_t *) &packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS input voltages: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_set_input_mode(uint8_t mode)
{
    KEPSStatus      status;
    uint8_t         packet[] = { SET_PV_AUTO, mode };
    eps_resp_header response;

    /* Modes: hardware default, MPPT, software fixed */
    if (mode > 2)
    {
        return EPS_ERROR_CONFIG;
    }


    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));

    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS input mode: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_set_heater(uint8_t cmd, uint8_t heater, uint8_t mode)
{
    KEPSStatus status;
    uint8_t    packet[] = {
            SET_HEATER,
            cmd,
            heater,
            mode
    };
    eps_resp_header response;

    /*
     * Currently there's only one command available (set heater on/off)
     * Heaters: BP4, onboard
     * Modes: off, on
     */
    if (cmd != 0 || heater > 1 || mode > 1)
    {
        return EPS_ERROR_CONFIG;
    }

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to set EPS heater/s %d mode: %d\n", heater, status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_reset_system_config()
{
    KEPSStatus      status;
    uint8_t         packet[] = { CMD_CONFIG1, 1 };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to reset EPS system configuration: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_reset_battery_config()
{
    KEPSStatus      status;
    uint8_t         packet[] = { CMD_CONFIG2, 1 };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to reset EPS battery configuration: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_reset_counters()
{
    KEPSStatus      status;
    uint8_t         packet[] = { RESET_COUNTERS, 0x42 };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to reset EPS counters: %d\n", status);
        return status;
    }

    return EPS_OK;
}

KEPSStatus k_eps_get_housekeeping(eps_hk_t * buff)
{
    KEPSStatus status;
    uint8_t packet[] = { GET_HOUSEKEEPING, 0 }; 
    uint8_t response[sizeof(eps_resp_header) + sizeof(eps_hk_t)] = { 0 };

    if (buff == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    status = kprv_eps_transfer(packet, sizeof(packet), response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to get EPS housekeeping data: %d\n", status);
        return status;
    }

    eps_hk_t * body = (eps_hk_t *) (response + sizeof(eps_resp_header));

    /* Convert big endian to host endianness for multi-byte fields */
    buff->vboost[0] = be16toh(body->vboost[0]);
    buff->vboost[1] = be16toh(body->vboost[1]);
    buff->vboost[2] = be16toh(body->vboost[2]);
    buff->vbatt = be16toh(body->vbatt);
    buff->curin[0] = be16toh(body->curin[0]);
    buff->curin[1] = be16toh(body->curin[1]);
    buff->curin[2] = be16toh(body->curin[2]);
    buff->cursun = be16toh(body->cursun);
    buff->cursys = be16toh(body->cursys);
    for (int i = 0; i < 6; i++)
    {
        buff->curout[i] = be16toh(body->curout[i]);
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output[i] = body->output[i];
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output_on_delta[i] = be16toh(body->output_on_delta[i]);
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output_off_delta[i] = be16toh(body->output_off_delta[i]);
    }
    for (int i = 0; i < 6; i++)
    {
        buff->latchup[i] = be16toh(body->latchup[i]);
    }
    buff->wdt_i2c_time_left = be32toh(body->wdt_i2c_time_left);
    buff->wdt_gnd_time_left = be32toh(body->wdt_gnd_time_left);
    buff->wdt_csp_pings_left[0] = body->wdt_csp_pings_left[0];
    buff->wdt_csp_pings_left[1] = body->wdt_csp_pings_left[1];
    buff->counter_wdt_i2c = be32toh(body->counter_wdt_i2c);
    buff->counter_wdt_gnd = be32toh(body->counter_wdt_gnd);
    buff->counter_wdt_csp[0] = be32toh(body->counter_wdt_csp[0]);
    buff->counter_wdt_csp[1] = be32toh(body->counter_wdt_csp[1]);
    buff->counter_boot = be32toh(body->counter_boot);
    for (int i = 0; i < 6; i++)
    {
        buff->temp[i] = be16toh(body->temp[i]);
    }
    buff->boot_cause = body->boot_cause;
    buff->batt_mode = body->batt_mode;
    buff->ppt_mode = body->ppt_mode;

    return EPS_OK;
}

KEPSStatus k_eps_get_system_config(eps_system_config_t * buff)
{
    KEPSStatus status;
    uint8_t    cmd = GET_CONFIG1;
    uint8_t    response[sizeof(eps_resp_header) + sizeof(eps_system_config_t)] = { 0 };

    if (buff == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    status = kprv_eps_transfer(&cmd, 1, response, sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to get EPS system configuration: %d\n", status);
        return status;
    }

    eps_system_config_t * body = (eps_system_config_t *) (response + sizeof(eps_resp_header));

    buff->ppt_mode = body->ppt_mode;
    buff->battheater_mode = body->battheater_mode;
    buff->battheater_low = body->battheater_low;
    buff->battheater_high = body->battheater_high;
    for (int i = 0; i < 8; i++)
    {
        buff->output_normal_value[i] = body->output_normal_value[i];
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output_safe_value[i] = body->output_safe_value[i];
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output_initial_on_delay[i] = be16toh(body->output_initial_on_delay[i]);
    }
    for (int i = 0; i < 8; i++)
    {
        buff->output_initial_off_delay[i] = be16toh(body->output_initial_off_delay[i]);
    }
    buff->vboost[0] = be16toh(body->vboost[0]);
    buff->vboost[1] = be16toh(body->vboost[1]);
    buff->vboost[2] = be16toh(body->vboost[2]);

    return EPS_OK;
}

KEPSStatus k_eps_get_battery_config(eps_battery_config_t * buff)
{
    KEPSStatus status;
    uint8_t    cmd = GET_CONFIG2;
    uint8_t    response[sizeof(eps_resp_header) + sizeof(eps_battery_config_t)] = { 0 };

    if (buff == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    status = kprv_eps_transfer(&cmd, 1, response, sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to get EPS battery configuration: %d\n", status);
        return status;
    }

    eps_battery_config_t * body = (eps_battery_config_t *) (response + sizeof(eps_resp_header));

    buff->batt_maxvoltage = be16toh(body->batt_maxvoltage);
    buff->batt_safevoltage = be16toh(body->batt_safevoltage);
    buff->batt_criticalvoltage = be16toh(body->batt_criticalvoltage);
    buff->batt_normalvoltage = be16toh(body->batt_normalvoltage);

    return EPS_OK;
}

KEPSStatus k_eps_get_heater(uint8_t * bp4, uint8_t * onboard)
{
    KEPSStatus status;
    uint8_t    cmd                                   = SET_HEATER;
    uint8_t    response[sizeof(eps_resp_header) + 2] = { 0 };

    if (bp4 == NULL && onboard == NULL)
    {
        return EPS_ERROR_CONFIG;
    }

    status = kprv_eps_transfer(&cmd, 1, response, sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to get EPS heater data: %d\n", status);
        return status;
    }

    if (bp4 != NULL)
    {
        memcpy(bp4, response + sizeof(eps_resp_header), 1);
    }
    if (onboard != NULL)
    {
        memcpy(onboard, response + sizeof(eps_resp_header) + 1, 1);
    }

    return EPS_OK;
}

KEPSStatus k_eps_watchdog_kick()
{
    KEPSStatus      status;
    uint8_t         packet[] = { RESET_WDT, 0x78 };
    eps_resp_header response;

    status = kprv_eps_transfer(packet, sizeof(packet), (uint8_t *) &response,
                               sizeof(response));
    if (status != EPS_OK)
    {
        fprintf(stderr, "Failed to kick EPS watchdog: %d\n", status);
        return status;
    }

    return EPS_OK;
}

pthread_t handle_watchdog = { 0 };
uint32_t watchdog_interval = 0;

void * kprv_eps_watchdog_thread(void * args)
{
    KEPSStatus status;

    while (1)
    {
        k_eps_watchdog_kick();

        sleep(watchdog_interval);
    }

    return NULL;
}

KEPSStatus k_eps_watchdog_start(uint32_t interval)
{
    if (interval == 0)
    {
        return EPS_ERROR_CONFIG;
    }

    if (handle_watchdog != 0)
    {
        fprintf(stderr, "EPS watchdog thread already started\n");
        return EPS_OK;
    }

    watchdog_interval = interval;

    if (pthread_create(&handle_watchdog, NULL, kprv_eps_watchdog_thread, NULL)
        != 0)
    {
        perror("Failed to create EPS watchdog thread");
        handle_watchdog = 0;
        return EPS_ERROR;
    }

    return EPS_OK;
}

KEPSStatus k_eps_watchdog_stop()
{
    /* Send the cancel request */
    if (pthread_cancel(handle_watchdog) != 0)
    {
        perror("Failed to cancel EPS watchdog thread");
        return EPS_ERROR;
    }

    /* Wait for the cancellation to complete */
    if (pthread_join(handle_watchdog, NULL) != 0)
    {
        perror("Failed to rejoin EPS watchdog thread");
        return EPS_ERROR;
    }

    handle_watchdog = 0;
    watchdog_interval = 0;

    return EPS_OK;
}

KEPSStatus k_eps_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx,
                             int rx_len)
{
    if (tx == NULL || tx_len < 1 || (rx == NULL && rx_len != 0))
    {
        return EPS_ERROR_CONFIG;
    }

    if (rx == NULL)
    {
        eps_resp_header hdr = { 0 };
        return kprv_eps_transfer(tx, tx_len, (uint8_t *) &hdr, sizeof(hdr));
    }
    else
    {
        return kprv_eps_transfer(tx, tx_len, rx, rx_len);
    }
}

KEPSStatus kprv_eps_transfer(const uint8_t * tx, int tx_len, uint8_t * rx,
                             int rx_len)
{
    KI2CStatus status;

    if (tx == NULL || tx_len < 1 || rx == NULL
        || rx_len < (int) sizeof(eps_resp_header))
    {
        return EPS_ERROR_CONFIG;
    }

    status = k_i2c_write(eps_bus, eps_addr, (uint8_t *) tx, tx_len);
    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to send EPS command: %d\n", status);
        return EPS_ERROR;
    }

    status = k_i2c_read(eps_bus, eps_addr, rx, rx_len);

    if (status != I2C_OK)
    {
        fprintf(stderr, "Failed to read EPS response (%x): %d\n", tx[0],
                status);
        return EPS_ERROR;
    }

    eps_resp_header response = { .cmd = rx[0], .status = rx[1] };

    if (response.cmd != tx[0])
    {
        /* Echoed command should match command requested */
        fprintf(stderr, "Command mismatch - Sent: %d Received: %d\n", tx[0],
                response.cmd);
        return EPS_ERROR;
    }

    /* Check the status byte */
    if (response.status != 0)
    {
        fprintf(stderr, "EPS returned an error (%d): %d\n", tx[0],
                response.status);
        return EPS_ERROR_INTERNAL;
    }

    return EPS_OK;
}
