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
 *
 * ISIS iMTQ API - Operational Commands
 */

#include <imtq.h>
#include <stdio.h>
#include <stdlib.h>

KADCSStatus k_adcs_noop(void)
{
    KADCSStatus      status = ADCS_OK;
    uint8_t          cmd    = NOOP;
    imtq_resp_header response;

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to execute iMTQ no-op command: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_adcs_reset(KADCSReset type)
{
    KADCSStatus      status;
    imtq_resp_header response;
    uint8_t          packet[2] = { RESET_MTQ >> 8, RESET_MTQ & 0xFF };

    if (type != SOFT_RESET)
    {
        fprintf(stderr, "Unknown iMTQ reset type requested: %d\n", type);
        return ADCS_ERROR_CONFIG;
    }

    /* 
     * We need a longer delay (100 ms) to allow the iMTQ time to come back up
     */
    const struct timespec TRANSFER_DELAY = {.tv_sec = 0, .tv_nsec = 100000000 };

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), &TRANSFER_DELAY);

    /*
     * It should just be an empty response, since the iMTQ rebooted and
     * doesn't have any non-volatile memory
     */
    if (status != ADCS_ERROR_NO_RESPONSE)
    {
        KIMTQStatus imtq_status = kprv_imtq_check_error(response.status);

        fprintf(stderr, "Failed to reset iMTQ: %d\n", imtq_status);

        return ADCS_ERROR;
    }

    return ADCS_OK;
}

/*
 * For the iMTQ, the only mode parameter that may be passed is the duration
 * value for detumble mode 
 */
KADCSStatus k_adcs_set_mode(ADCSMode mode, const adcs_mode_param * duration)
{
    KADCSStatus status;

    switch (mode)
    {
        case DETUMBLE:
            if (duration == NULL)
            {
                status = ADCS_ERROR_CONFIG;
                break;
            }
            status = k_imtq_start_detumble(*duration);
            break;
        case IDLE:
            status = k_imtq_cancel_op();
            break;
        case SELFTEST:
            fprintf(
                stderr,
                "iMTQ self-test mode must be started with k_adcs_run_test\n");
            status = ADCS_ERROR_CONFIG;
            break;
        default:
            fprintf(stderr, "Unknown iMTQ mode requested: %d\n", mode);
            status = ADCS_ERROR_CONFIG;
    }

    return status;
}

KADCSStatus k_adcs_run_test(ADCSTestType axis, adcs_test_results buffer)
{
    KADCSStatus status;

    if (buffer == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = k_imtq_start_test(axis);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ self-test for %d axis: %d\n",
                axis, status);
        return status;
    }

    /* Wait an appropriate time for the test to finish */
    const struct timespec TRANSFER_DELAY = {.tv_sec = 1, .tv_nsec = 300000000 };

    nanosleep(&TRANSFER_DELAY, NULL);

    if (axis == TEST_ALL)
    {
        imtq_test_result_all data = { 0 };
        
        status = k_imtq_get_test_results_all(&data);
        if (status != ADCS_OK)
        {
            fprintf(stderr, "Failed to get test results (all): %d\n", status);
            return status;
        }

        kprv_adcs_process_test(buffer, data.init);
        kprv_adcs_process_test(buffer, data.x_pos);
        kprv_adcs_process_test(buffer, data.x_neg);
        kprv_adcs_process_test(buffer, data.y_pos);
        kprv_adcs_process_test(buffer, data.y_neg);
        kprv_adcs_process_test(buffer, data.z_pos);
        kprv_adcs_process_test(buffer, data.z_neg);
        kprv_adcs_process_test(buffer, data.final);
    }
    else
    {
        imtq_test_result_single data = { 0 };
        
        status = k_imtq_get_test_results_single(&data);
        if (status != ADCS_OK)
        {
            fprintf(stderr,
                    "Failed to get single test results (single): %d\n", status);
            return status;
        }

        kprv_adcs_process_test(buffer, data.init);
        kprv_adcs_process_test(buffer, data.step);
        kprv_adcs_process_test(buffer, data.final);
    }

    return status;
}

KADCSStatus k_imtq_cancel_op(void)
{
    KADCSStatus      status = ADCS_OK;
    uint8_t          cmd    = CANCEL_OP;
    imtq_resp_header response;

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to execute iMTQ cancel command: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_measurement(void)
{
    KADCSStatus      status = ADCS_OK;
    uint8_t          cmd    = START_MEASURE;
    imtq_resp_header response;

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ MTM measurement: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_actuation_current(imtq_axis_data current, uint16_t time)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t     packet[9] = {
            START_CURRENT,
            current.x & 0xFF, current.x >> 8,
            current.y & 0xFF, current.y >> 8,
            current.z & 0xFF, current.z >> 8,
            time & 0xFF, time >> 8
    };

    imtq_resp_header response;

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ actuation (current): %d\n",
                status);
        if (kprv_imtq_check_error(response.status) == IMTQ_ERROR_BAD_PARAM)
        {
            fprintf(stderr,
                    "One or more of the requested currents was too large\n");
        }

        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_actuation_dipole(imtq_axis_data dipole, uint16_t time)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t     packet[9] = {
            START_DIPOLE,
            dipole.x & 0xFF, dipole.x >> 8,
            dipole.y & 0xFF, dipole.y >> 8,
            dipole.z & 0xFF, dipole.z >> 8,
            time & 0xFF, time >> 8
    };

    imtq_resp_header response;

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ actuation (dipole): %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_actuation_PWM(imtq_axis_data pwm, uint16_t time)
{
    KADCSStatus status = ADCS_OK;

    if (abs(pwm.x) > 1000 || abs(pwm.y) > 1000 || abs(pwm.z) > 1000)
    {
        fprintf(stderr, "Error: iMTQ duty cycle cannot exceed 100%%\n");
        return ADCS_ERROR_CONFIG;
    }

    uint8_t packet[9] = {
            START_PWM,
            pwm.x & 0xFF, pwm.x >> 8,
            pwm.y & 0xFF, pwm.y >> 8,
            pwm.z & 0xFF, pwm.z >> 8,
            time & 0xFF, time >> 8
    };

    imtq_resp_header response;

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ actuation (PWM): %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_test(ADCSTestType axis)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t     packet[2] = {
            START_TEST,
            (uint8_t) axis
    };
    imtq_resp_header response;

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start iMTQ self-test (%d): %d\n", axis,
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_start_detumble(uint16_t time)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t     packet[3] = {
            START_BDOT,
            time & 0xFF, time >> 8
    };
    imtq_resp_header response;

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) &response,
                                sizeof(response), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start detumble mode: %d\n", status);
        return status;
    }

    return ADCS_OK;
}
