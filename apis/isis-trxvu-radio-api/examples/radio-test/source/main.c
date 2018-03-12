/*
 * Kubos Linux
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http:www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Example Project Using the ISIS TRXVU Radio
 */

#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>

#include "trxvu-radio-api/trxvu.h"

#define check_status()           \
    ({                           \
        if (status != RADIO_OK)  \
        {                        \
            k_radio_terminate(); \
            exit(-1);            \
        }                        \
    })

KRadioStatus get_tx_telem()
{
    KRadioStatus       status;
    radio_telem tx_telem1 = { 0 };
    status = k_radio_get_telemetry(&tx_telem1, RADIO_TX_TELEM_ALL);
    if (status != RADIO_OK)
    {
        printf("Failed to get all telemetry: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Transmitter Telemetry - All\n"
               "---------------------------------\n");
        printf("TX inst RF reflected: %f dBm\n", get_rf_power_dbm(tx_telem1.tx_telem.inst_RF_reflected));
        printf("TX inst RF forward:   %f dBm\n", get_rf_power_dbm(tx_telem1.tx_telem.inst_RF_forward));
        printf("TX supply voltage:    %f V\n", get_voltage(tx_telem1.tx_telem.supply_voltage));
        printf("TX supply current:    %f mA\n", get_current(tx_telem1.tx_telem.supply_current));
        printf("TX power amp temp:    %f C\n", get_temperature(tx_telem1.tx_telem.temp_power_amp));
        printf("TX oscillator temp:   %f C\n\n", get_temperature(tx_telem1.tx_telem.temp_oscillator));
    }

    radio_telem tx_telem2 = { 0 };
    status = k_radio_get_telemetry(&tx_telem2, RADIO_TX_TELEM_LAST);
    if (status != RADIO_OK)
    {
        printf("Failed to get last telemetry: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Transmitter Telemetry - Last\n"
               "---------------------------------\n");
        printf("TX inst RF reflected: %f dBm\n", get_rf_power_dbm(tx_telem2.tx_telem.inst_RF_reflected));
        printf("TX inst RF forward:   %f dBm\n", get_rf_power_dbm(tx_telem2.tx_telem.inst_RF_forward));
        printf("TX supply voltage:    %f V\n", get_voltage(tx_telem2.tx_telem.supply_voltage));
        printf("TX supply current:    %f mA\n", get_current(tx_telem2.tx_telem.supply_current));
        printf("TX power amp temp:    %f C\n", get_temperature(tx_telem2.tx_telem.temp_power_amp));
        printf("TX oscillator temp:   %f C\n\n", get_temperature(tx_telem2.tx_telem.temp_oscillator));
    }

    radio_telem tx_up;
    status = k_radio_get_telemetry(&tx_up, RADIO_TX_UPTIME);
    if (status != RADIO_OK)
    {
        printf("Failed to get uptime: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Transmitter Telemetry - Uptime\n"
               "---------------------------------\n");
        printf("TX Uptime: %d\n\n", tx_up.uptime);
    }

    radio_telem tx_state = {0};
    status = k_radio_get_telemetry(&tx_state, RADIO_TX_STATE);
    if (status != RADIO_OK)
    {
        printf("Failed to get state: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Transmitter Telemetry - State\n"
               "---------------------------------\n");
        printf("State: %x\n", tx_state.tx_state);
        printf("TX Idle On:   %s\n",
                tx_state.tx_state & RADIO_STATE_IDLE_ON ? "True" : "False");
        printf("TX Beacon:    %s\n",
                tx_state.tx_state & RADIO_STATE_BEACON_ACTIVE ? "On" : "Off");

        char rate[] = "1200";
        uint8_t rate_flag = tx_state.tx_state >> 2;
        if (rate_flag == RADIO_STATE_RATE_9600)
        {
            strcpy(rate, "9600");
        }
        else if (rate_flag == RADIO_STATE_RATE_4800)
        {
            strcpy(rate, "4800");
        }
        else if (rate_flag == RADIO_STATE_RATE_2400)
        {
            strcpy(rate, "2400");
        }

        printf("TX Data Rate: %s\n\n", rate);
    }

    return status;
}

KRadioStatus get_rx_telem()
{
    KRadioStatus       status;
    radio_telem rx_telem = { 0 };
    status = k_radio_get_telemetry(&rx_telem, RADIO_RX_TELEM_ALL);
    if (status != RADIO_OK)
    {
        printf("Failed to get all telemetry: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Receiver Telemetry - All\n"
               "------------------------------\n");
        printf("RX inst doppler offset:    %f Hz\n", get_doppler_offset(rx_telem.rx_telem.inst_doppler_offset));
        printf("RX inst signal strength:   %f dBm\n",
               get_signal_strength(rx_telem.rx_telem.inst_signal_strength));
        printf("RX supply voltage:         %f V\n", get_voltage(rx_telem.rx_telem.supply_voltage));
        printf("RX supply current:         %f mA\n", get_current(rx_telem.rx_telem.supply_current));
        printf("RX power amp temp:         %f C\n", get_temperature(rx_telem.rx_telem.temp_power_amp));
        printf("RX oscillator temp:        %f C\n\n", get_temperature(rx_telem.rx_telem.temp_oscillator));
    }

    radio_telem rx_up;
    status = k_radio_get_telemetry(&rx_up, RADIO_RX_UPTIME);
    if (status != RADIO_OK)
    {
        printf("Failed to get uptime: %d\n", status);
        return status;
    }
    else
    {
        printf("TRXVU Receiver Telemetry - Uptime\n"
               "---------------------------------\n");
        printf("RX Uptime: %d\n\n", rx_up.uptime);
    }

    return status;
}

KRadioStatus send_message()
{
    KRadioStatus status;
    uint8_t message[] = "Radio Test Message";
    uint8_t len = sizeof(message);
    uint8_t response;

    status = k_radio_send(message, len, &response);
    if (status != RADIO_OK)
    {
        printf("Failed to send message: %d\n", status);
    }
    else
    {
        printf("Message Send Response: %d\n", response);
    }

    return status;
}

KRadioStatus send_override()
{
    KRadioStatus status;

    ax25_callsign to = {
            .ascii = "KBSTO",
            .ssid = 1
    };

    ax25_callsign from = {
            .ascii = "KBSFRM",
            .ssid = 2
    };

    uint8_t beacon_msg[] = "Beacon Message";

    radio_tx_beacon beacon = {
            .interval = 5,
            .msg = beacon_msg,
            .len = sizeof(beacon_msg)
    };

    status = k_radio_set_beacon_override(to, from, beacon);
    if (status != RADIO_OK)
    {
        printf("Failed to send message: %d\n", status);
        return status;
    }
    else
    {
        printf("Beacon started successfully\n");
    }

    uint8_t message[] = "Radio Test Message";
    uint8_t len = sizeof(message);
    uint8_t response;

    status = k_radio_send_override(to, from, message, len, &response);
    if (status != RADIO_OK)
    {
        printf("Failed to send message: %d\n", status);
    }
    else
    {
        printf("Message Send Response: %d\n", response);
    }

    return status;
}


KRadioStatus read_message()
{
    KRadioStatus status;
    radio_rx_message buffer;
    uint8_t len;

    status = k_radio_recv(&buffer, &len);
    if (status == RADIO_RX_EMPTY)
    {
        printf("No messages to receive\n");
    }
    else if (status == RADIO_OK)
    {
        printf("Received message(%d %fHz %fdBm): %s\n",
                len, get_doppler_offset(buffer.doppler_offset), get_signal_strength(buffer.signal_strength),
                buffer.message);
    }
    else
    {
        printf("Failed to send message: %d\n", status);
    }

    return status;
}

KRadioStatus set_options()
{
    radio_config config = {0};
    char beacon_msg[] = "Radio Beacon Message";
    char option[1] = {0};

    printf("Set new 'to' callsign? (y/n)\n");
    scanf("%s", option);

    if (option[0] == 'y' || option[0] == 'Y')
    {
        strcpy(config.to.ascii,"MJRTOM");
    }

    printf("Set new 'from' callsign? (y/n)\n");
    scanf("%s", option);

    if (option[0] == 'y' || option[0] == 'Y')
    {
        strcpy(config.from.ascii,"HMLTN1");
    }

    printf("Enter a data rate: \n\t"
            "1 - 1200\n\t"
            "2 - 2400\n\t"
            "3 - 4800\n\t"
            "4 - 9600\n\t"
            "5 - Don't change data rate\n\t");
    scanf("%s", option);

    switch (option[0])
    {
        case '1':
            config.data_rate = RADIO_TX_RATE_1200;
            break;
        case '2':
            config.data_rate = RADIO_TX_RATE_2400;
            break;
        case '3':
            config.data_rate = RADIO_TX_RATE_4800;
            break;
        case '4':
            config.data_rate = RADIO_TX_RATE_9600;
            break;
        default:
            printf("Not changing data rate\n");
    }

    printf("Turn on beacon? (y/n)\n");
    scanf("%s", option);

    if (option[0] == 'y' || option[0] == 'Y')
    {
        config.beacon.interval = 5;
        config.beacon.msg = beacon_msg;
        config.beacon.len = sizeof(beacon_msg);
    }

    printf("Idle On? (y/n/*)\n");
    scanf("%s", option);

    if (option[0] == 'y' || option[0] == 'Y')
    {
        config.idle = RADIO_IDLE_ON;
    }
    else if (option[0] == 'n' || option[0] == 'N')
    {
        config.idle = RADIO_IDLE_OFF;
    }


    printf("Setting configuration options\n");

    KRadioStatus status = k_radio_configure(&config);
    if (status != RADIO_OK)
    {
        printf("Something went wrong during configuration: %d\n", status);
    }

    return status;
}

int running;

void sigint_handler(int sig)
{
    running = 0;
}

int main(int argc, char * argv[])
{
    printf("TRXVU Test Program\n");

    running = 1;

    /* Ctrl+C will trigger a signal to end the program */
    struct sigaction action;
    action.sa_handler = sigint_handler;
    sigaction(SIGINT, &action, NULL);

    KRadioStatus status;
    status = k_radio_init();
    check_status();

    status = k_radio_watchdog_start();
    check_status();

    char option[1] = {0};
    while (running)
    {
        printf("Enter a command option (enter 'h' to see list of commands): \n");
        scanf("%s", option);

        switch (option[0])
        {
            /* Configure */
            case 'c':
                status = set_options();
                check_status();
                break;
            /* Fetch - Read a message */
            case 'f':
                status = read_message();
                check_status();
                break;
            case 'h':
                printf("Options: \n\t"
                        "c - Set configuration options\n\t"
                        "f - Fetch a message from the receive buffer\n\t"
                        "h - Print this help\n\t"
                        "o - Send a beacon and message with non-default callsigns\n\t"
                        "r - Get receiver telemetry\n\t"
                        "s - Send a message through the transmitter\n\t"
                        "t - Get transmitter telemetry\n\t"
                        "z - Reboot radio\n");
                break;
            /* Override - Send a message and beacon with non-default callsigns */
            case 'o':
                status = send_override();
                check_status();
                break;
            /* RX - Get RX Telemetry */
            case 'r':
                status = get_rx_telem();
                check_status();
                break;
            /* Send - Send a message */
            case 's':
                status = send_message();
                check_status();
                break;
            /* TX - Get TX Telemetry */
            case 't':
                status = get_tx_telem();
                check_status();
                break;
            /* Reboot the radio */
            case 'z':
                status = k_radio_reset(RADIO_HARD_RESET);
                check_status();
                break;
            default:
                printf("Invalid option: %c\n", option[0]);
        }
    }

    k_radio_watchdog_stop();
    k_radio_terminate();

    printf("Execution completed successfully\n");

    return 0;
}
