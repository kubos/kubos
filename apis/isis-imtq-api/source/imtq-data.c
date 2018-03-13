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
 * ISIS iMTQ API - Data Request Commands
 */

#include <isis-imtq-api/imtq.h>
#include <stdio.h>
#include <string.h>

/*
 * Array of all possible iMTQ configuration parameters. Used for fetching the
 * current configuration settings for debug telemetry
 */
const uint16_t adcs_config_params[] = {
        MTM_SELECT,
        MTM_INTERNAL_TIME, MTM_EXTERNAL_TIME,
        MTM_INTERNAL_MAP_X, MTM_INTERNAL_MAP_Y, MTM_INTERNAL_MAP_Z,
        MTM_EXTERNAL_MAP_X, MTM_EXTERNAL_MAP_Y, MTM_EXTERNAL_MAP_Z,
        MTM_MATRIX_R1_C1, MTM_MATRIX_R1_C2, MTM_MATRIX_R1_C3, MTM_MATRIX_R2_C1, MTM_MATRIX_R2_C2, MTM_MATRIX_R2_C3, MTM_MATRIX_R3_C1, MTM_MATRIX_R3_C2, MTM_MATRIX_R3_C3,
        MTM_BIAS_X, MTM_BIAS_Y, MTM_BIAS_Z,
        ADC_COIL_CURRENT_BIAS_X, ADC_COIL_CURRENT_BIAS_Y, ADC_COIL_CURRENT_BIAS_Z,
        ADC_COIL_CURRENT_MULT_X, ADC_COIL_CURRENT_MULT_Y, ADC_COIL_CURRENT_MULT_Z,
        ADC_COIL_CURRENT_DIV_X, ADC_COIL_CURRENT_DIV_Y, ADC_COIL_CURRENT_DIV_Z,
        ADC_COIL_TEMP_BIAS_X, ADC_COIL_TEMP_BIAS_Y, ADC_COIL_TEMP_BIAS_Z,
        ADC_COIL_TEMP_MULT_X, ADC_COIL_TEMP_MULT_Y, ADC_COIL_TEMP_MULT_Z,
        ADC_COIL_TEMP_DIV_X, ADC_COIL_TEMP_DIV_Y, ADC_COIL_TEMP_DIV_Z,
        DETUMBLE_FREQUENCY, BDOT_GAIN, MTM_FILTER_SENSITIVITY, MTM_FILTER_WEIGHT,
        COIL_AREA_X, COIL_AREA_Y, COIL_AREA_Z,
        COIL_CURRENT_LIMIT,
        CURRENT_FEEDBACK_ENABLE,
        CURRENT_FEEDBACK_GAIN_X, CURRENT_FEEDBACK_GAIN_Y, CURRENT_FEEDBACK_GAIN_Z,
        CURRENT_MAP_TEMP_T1, CURRENT_MAP_TEMP_T2, CURRENT_MAP_TEMP_T3, CURRENT_MAP_TEMP_T4, CURRENT_MAP_TEMP_T5, CURRENT_MAP_TEMP_T6, CURRENT_MAP_TEMP_T7,
        CURRENT_MAX_X_T1, CURRENT_MAX_X_T2, CURRENT_MAX_X_T3, CURRENT_MAX_X_T4, CURRENT_MAX_X_T5, CURRENT_MAX_X_T6, CURRENT_MAX_X_T7,
        CURRENT_MAX_Y_T1, CURRENT_MAX_Y_T2, CURRENT_MAX_Y_T3, CURRENT_MAX_Y_T4, CURRENT_MAX_Y_T5, CURRENT_MAX_Y_T6, CURRENT_MAX_Y_T7,
        CURRENT_MAX_Z_T1, CURRENT_MAX_Z_T2, CURRENT_MAX_Z_T3, CURRENT_MAX_Z_T4, CURRENT_MAX_Z_T5, CURRENT_MAX_Z_T6, CURRENT_MAX_Z_T7,
        HW_CONFIG, WATCHDOG_TIMEOUT, SLAVE_ADDRESS, SOFTWARE_VERSION
};

/* Human-readable names for the axis tested in a self-test step */
const char test_step[8][5] = {
        "init",
        "posx",
        "negx",
        "posy",
        "negy",
        "posz",
        "negz",
        "fina"
};

/* ADCS API Functions */

KADCSStatus k_adcs_get_mode(ADCSMode * mode)
{
    KADCSStatus status;
    imtq_state  state;

    if (mode == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = k_imtq_get_system_state(&state);
    if (status == ADCS_OK)
    {
        *mode = state.mode;
    }

    return status;
}

KADCSStatus k_adcs_get_power_status(adcs_power_status * uptime)
{
    KADCSStatus status;
    imtq_state  state;

    if (uptime == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = k_imtq_get_system_state(&state);
    if (status == ADCS_OK)
    {
        *uptime = state.uptime;
    }
    else if (status == ADCS_ERROR)
    {
        /* Assume system is offline, so uptime is zero */
        *uptime = 0;
        status  = ADCS_OK;
    }

    return status;
}

KADCSStatus k_adcs_get_telemetry(ADCSTelemType type, JsonNode * buffer)
{
    KADCSStatus status;

    if (buffer == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_adcs_get_status_telemetry(buffer);
    if (status != ADCS_OK)
    {
        return status;
    }

    if (type == DEBUG)
    {
        status = kprv_adcs_get_debug_telemetry(buffer);
    }
    else if (type == NOMINAL)
    {
        status = kprv_adcs_get_nominal_telemetry(buffer);
    }
    else
    {
        fprintf(stderr, "Unknown iMTQ telemetry type requested: %d\n", type);
        status = ADCS_ERROR_CONFIG;
    }

    return status;
}

KADCSStatus k_adcs_get_orientation(adcs_orient * data)
{
    return ADCS_ERROR_NOT_IMPLEMENTED;
}

KADCSStatus k_adcs_get_spin(adcs_spin * data)
{
    return ADCS_ERROR_NOT_IMPLEMENTED;
}

KADCSStatus kprv_adcs_get_status_telemetry(JsonNode * buffer)
{
    KADCSStatus status;
    imtq_state  state;

    if (buffer == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = k_imtq_get_system_state(&state);
    if (status == ADCS_OK)
    {
        switch (state.mode)
        {
            case IDLE:
                json_append_member(buffer, "system_mode", json_mkstring("IDLE"));
                break;
            case DETUMBLE:
                json_append_member(buffer, "system_mode", json_mkstring("DETUMBLE"));
                break;
            case SELFTEST:
                json_append_member(buffer, "system_mode", json_mkstring("SELFTEST"));
                break;
        }

        json_append_member(buffer, "system_error", json_mkstring((state.error) ? "yes" : "no"));
        json_append_member(buffer, "system_configured", json_mkstring((state.config) ? "yes" : "no"));
        json_append_member(buffer, "system_uptime", json_mknumber((double) state.uptime));


    }
    else if (status == ADCS_ERROR)
    {
        /* Assume system is offline, so uptime is zero */
        json_append_member(buffer, "system_mode", json_mkstring("OFFLINE"));
        json_append_member(buffer, "system_uptime", json_mknumber(0));
    }

    return status;
}


KADCSStatus kprv_adcs_get_nominal_telemetry(JsonNode * buffer)
{
    KADCSStatus status = ADCS_OK;
    KADCSStatus nom_status;

    imtq_housekeeping_raw house_raw = { 0 };
    imtq_housekeeping_eng house_eng = { 0 };
    imtq_detumble         detumble  = { 0 };
    imtq_mtm_msg          mtm_raw   = { 0 };
    imtq_mtm_msg          mtm_calib = { 0 };
    imtq_dipole           dipole    = { 0 };

    if (buffer == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    /* Housekeeping data */
    nom_status = k_imtq_get_raw_housekeeping(&house_raw);
    nom_status |= k_imtq_get_eng_housekeeping(&house_eng);
    if (nom_status != ADCS_OK)
    {
        status = ADCS_ERROR;
    }
    else
    {
        /* Raw ADC values */
        json_append_member(buffer, "supply_voltage_digital_raw", json_mknumber((double) house_raw.voltage_d));
        json_append_member(buffer, "supply_voltage_analog_raw", json_mknumber((double) house_raw.voltage_a));
        json_append_member(buffer, "supply_current_digital_raw", json_mknumber((double) house_raw.current_d));
        json_append_member(buffer, "supply_current_analog_raw", json_mknumber((double) house_raw.current_a));
        json_append_member(buffer, "coil_current_x_raw", json_mknumber((double) house_raw.coil_current.x));
        json_append_member(buffer, "coil_current_y_raw", json_mknumber((double) house_raw.coil_current.y));
        json_append_member(buffer, "coil_current_z_raw", json_mknumber((double) house_raw.coil_current.z));
        json_append_member(buffer, "coil_temp_x_raw", json_mknumber((double) house_raw.coil_temp.x));
        json_append_member(buffer, "coil_temp_y_raw", json_mknumber((double) house_raw.coil_temp.y));
        json_append_member(buffer, "coil_temp_z_raw", json_mknumber((double) house_raw.coil_temp.z));
        json_append_member(buffer, "mcu_temp_raw", json_mknumber((double) house_raw.mcu_temp));

        /* Converted values */
        json_append_member(buffer, "supply_voltage_digital_eng", json_mknumber((double) house_eng.voltage_d));
        json_append_member(buffer, "supply_voltage_analog_eng", json_mknumber((double) house_eng.voltage_a));
        json_append_member(buffer, "supply_current_digital_eng", json_mknumber((double) house_eng.current_d));
        json_append_member(buffer, "supply_current_analog_eng", json_mknumber((double) house_eng.current_a));
        json_append_member(buffer, "coil_current_x_eng", json_mknumber((double) house_eng.coil_current.x));
        json_append_member(buffer, "coil_current_y_eng", json_mknumber((double) house_eng.coil_current.y));
        json_append_member(buffer, "coil_current_z_eng", json_mknumber((double) house_eng.coil_current.z));
        json_append_member(buffer, "coil_temp_x_eng", json_mknumber((double) house_eng.coil_temp.x));
        json_append_member(buffer, "coil_temp_y_eng", json_mknumber((double) house_eng.coil_temp.y));
        json_append_member(buffer, "coil_temp_z_eng", json_mknumber((double) house_eng.coil_temp.z));
        json_append_member(buffer, "mcu_temp_eng", json_mknumber((double) house_eng.mcu_temp));
    }

    /* Data during last detumble loop */
    nom_status = k_imtq_get_detumble(&detumble);
    if (nom_status != ADCS_OK)
    {
        status = ADCS_ERROR;
    }
    else
    {
        json_append_member(buffer, "detumble_calib_mtm_x", json_mknumber((double) detumble.mtm_calib.x));
        json_append_member(buffer, "detumble_calib_mtm_y", json_mknumber((double) detumble.mtm_calib.y));
        json_append_member(buffer, "detumble_calib_mtm_z", json_mknumber((double) detumble.mtm_calib.z));
        json_append_member(buffer, "detumble_filter_mtm_x", json_mknumber((double) detumble.mtm_filter.x));
        json_append_member(buffer, "detumble_filter_mtm_y", json_mknumber((double) detumble.mtm_filter.y));
        json_append_member(buffer, "detumble_filter_mtm_z", json_mknumber((double) detumble.mtm_filter.z));
        json_append_member(buffer, "detumble_bdot_x", json_mknumber((double) detumble.bdot.x));
        json_append_member(buffer, "detumble_bdot_y", json_mknumber((double) detumble.bdot.y));
        json_append_member(buffer, "detumble_bdot_z", json_mknumber((double) detumble.bdot.z));
        json_append_member(buffer, "detumble_dipole_x", json_mknumber((double) detumble.dipole.x));
        json_append_member(buffer, "detumble_dipole_y", json_mknumber((double) detumble.dipole.y));
        json_append_member(buffer, "detumble_dipole_z", json_mknumber((double) detumble.dipole.z));
        json_append_member(buffer, "detumble_cmd_current_x", json_mknumber((double) detumble.cmd_current.x));
        json_append_member(buffer, "detumble_cmd_current_y", json_mknumber((double) detumble.cmd_current.y));
        json_append_member(buffer, "detumble_cmd_current_z", json_mknumber((double) detumble.cmd_current.z));
        json_append_member(buffer, "detumble_coil_current_x", json_mknumber((double) detumble.coil_current.x));
        json_append_member(buffer, "detumble_coil_current_y", json_mknumber((double) detumble.coil_current.y));
        json_append_member(buffer, "detumble_coil_current_z", json_mknumber((double) detumble.coil_current.z));
    }

    /* Current magnetometer measurements */
    nom_status = k_imtq_start_measurement();
    if (nom_status != ADCS_OK)
    {
        status = ADCS_ERROR;
    }
    else
    {
        const struct timespec TRANSFER_DELAY
            = {.tv_sec = 0, .tv_nsec = 1000001 };

        nanosleep(&TRANSFER_DELAY, NULL);

        nom_status = k_imtq_get_raw_mtm(&mtm_raw);
        nom_status |= k_imtq_get_calib_mtm(&mtm_calib);

        if (nom_status != ADCS_OK)
        {
            status = ADCS_ERROR;
        }
        else
        {
            json_append_member(buffer, "mtm_actuating", json_mkstring((mtm_raw.act_status) ? "yes" : "no"));
            json_append_member(buffer, "mtm_x_raw", json_mknumber((double) mtm_raw.data.x));
            json_append_member(buffer, "mtm_y_raw", json_mknumber((double) mtm_raw.data.y));
            json_append_member(buffer, "mtm_z_raw", json_mknumber((double) mtm_raw.data.z));
            json_append_member(buffer, "mtm_x_calib", json_mknumber((double) mtm_calib.data.x));
            json_append_member(buffer, "mtm_y_calib", json_mknumber((double) mtm_calib.data.y));
            json_append_member(buffer, "mtm_z_calib", json_mknumber((double) mtm_calib.data.z));
        }
    }

    /* Commanded actuation dipole */
    nom_status = k_imtq_get_dipole(&dipole);
    if (nom_status != ADCS_OK)
    {
        status = ADCS_ERROR;
    }
    else
    {
        json_append_member(buffer, "dipole_x", json_mknumber((double) dipole.data.x));
        json_append_member(buffer, "dipole_y", json_mknumber((double) dipole.data.y));
        json_append_member(buffer, "dipole_z", json_mknumber((double) dipole.data.z));
    }

    return status;
}

KADCSStatus kprv_adcs_get_debug_telemetry(JsonNode * buffer)
{
    KADCSStatus      status = ADCS_OK;
    KADCSStatus      debug_status;
    imtq_config_resp config_data;

    if (buffer == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    /* Get all of the configuration values */
    int num_config_params
        = sizeof(adcs_config_params) / sizeof(adcs_config_params[0]);
    for (int i = 0; i < num_config_params; i++)
    {
        debug_status = k_imtq_get_param(adcs_config_params[i], &config_data);
        if (debug_status == ADCS_OK)
        {
            char param[7] = { 0 };
            sprintf(param, "%#x", adcs_config_params[i]);

            /* Convert the param value to a double and add a new JSON element
             * to the return buffer */
            switch (adcs_config_params[i] >> 12)
            {
                case 0x1:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.int8_val));
                    break;
                case 0x2:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.uint8_val));
                    break;
                case 0x3:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.int16_val));
                    break;
                case 0x4:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.uint16_val));
                    break;
                case 0x5:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.int32_val));
                    break;
                case 0x6:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.uint32_val));
                    break;
                case 0x7:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.float_val));
                    break;
                case 0x8:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.int64_val));
                    break;
                case 0x9:
                    json_append_member(buffer, param, json_mknumber((double) config_data.value.uint64_val));
                    break;
                case 0xA:
                    json_append_member(buffer, param, json_mknumber(config_data.value.double_val));
                    break;
                default:
                    /* We shouldn't ever get here... */
                    fprintf(stderr, "Unknown iMTQ configuration parameter "
                                    "type passed: %s\n",
                            param);
                    status = ADCS_ERROR;
            }
        }
        else
        {
            fprintf(stderr, "Failed to fetch iMTQ param %#x: %d\n", adcs_config_params[i], status);
            status = ADCS_ERROR;
            continue;
        }
    }

    /* Get the last-run test results */
    imtq_test_result_all data = { 0 };
    debug_status              = k_imtq_get_test_results_all(&data);
    if (debug_status == ADCS_OK)
    {
        kprv_adcs_process_test(buffer, data.init);
        kprv_adcs_process_test(buffer, data.x_pos);
        kprv_adcs_process_test(buffer, data.x_neg);
        kprv_adcs_process_test(buffer, data.y_pos);
        kprv_adcs_process_test(buffer, data.y_neg);
        kprv_adcs_process_test(buffer, data.z_pos);
        kprv_adcs_process_test(buffer, data.z_neg);
        kprv_adcs_process_test(buffer, data.final);
    }
    else if (debug_status != ADCS_ERROR_INTERNAL
             && kprv_imtq_check_error(data.init.hdr.status) != IMTQ_ERROR_MODE)
    {
        fprintf(stderr, "Encountered an unexpected error: %d\n", debug_status);
        status = ADCS_ERROR;
    }
    else
    {
        /*
         * Should mean that no test results are available
         * because no tests have been run since the iMTQ was
         * powered
         */
    }

    return status;
}

void kprv_adcs_process_test(JsonNode * parent, imtq_test_result test)
{
    if (parent == NULL)
    {
        return;
    }

    if (test.hdr.cmd != GET_TEST)
    {
        /*
         * This will most likely only happen while returning the last
         * self-test results for debug telemetry.
         * For simplicity, that logic attempts to process the results of an
         * all-axes self-test. If the last test was a single axis, then the 
         * last 5 step results will be empty.
         * This results in some calls to this function being made with 
         * empty test data.
         */
        return;
    }

    char error[]          = "tr_nnnn_error";
    char mtm_raw_x[]      = "tr_nnnn_mtm_raw_x";
    char mtm_raw_y[]      = "tr_nnnn_mtm_raw_y";
    char mtm_raw_z[]      = "tr_nnnn_mtm_raw_z";
    char mtm_calib_x[]    = "tr_nnnn_mtm_calib_x";
    char mtm_calib_y[]    = "tr_nnnn_mtm_calib_y";
    char mtm_calib_z[]    = "tr_nnnn_mtm_calib_z";
    char coil_current_x[] = "tr_nnnn_coil_current_x";
    char coil_current_y[] = "tr_nnnn_coil_current_y";
    char coil_current_z[] = "tr_nnnn_coil_current_z";
    char coil_temp_x[]    = "tr_nnnn_coil_temp_x";
    char coil_temp_y[]    = "tr_nnnn_coil_temp_y";
    char coil_temp_z[]    = "tr_nnnn_coil_temp_z";

    char step[5];
    strncpy(step, test_step[test.step], sizeof(step));

    sprintf(error, "tr_%s_error", step);
    sprintf(mtm_raw_x, "tr_%s_mtm_raw_x", step);
    sprintf(mtm_raw_y, "tr_%s_mtm_raw_y", step);
    sprintf(mtm_raw_z, "tr_%s_mtm_raw_z", step);
    sprintf(mtm_calib_x, "tr_%s_mtm_calib_x", step);
    sprintf(mtm_calib_y, "tr_%s_mtm_calib_y", step);
    sprintf(mtm_calib_z, "tr_%s_mtm_calib_z", step);
    sprintf(coil_current_x, "tr_%s_coil_current_x", step);
    sprintf(coil_current_y, "tr_%s_coil_current_y", step);
    sprintf(coil_current_z, "tr_%s_coil_current_z", step);
    sprintf(coil_temp_x, "tr_%s_coil_temp_x", step);
    sprintf(coil_temp_y, "tr_%s_coil_temp_y", step);
    sprintf(coil_temp_z, "tr_%s_coil_temp_z", step);

    json_append_member(parent, error, json_mknumber((double) test.error));
    json_append_member(parent, mtm_raw_x, json_mknumber((double) test.mtm_raw.x));
    json_append_member(parent, mtm_raw_y, json_mknumber((double) test.mtm_raw.y));
    json_append_member(parent, mtm_raw_z, json_mknumber((double) test.mtm_raw.z));
    json_append_member(parent, mtm_calib_x, json_mknumber((double) test.mtm_calib.x));
    json_append_member(parent, mtm_calib_y, json_mknumber((double) test.mtm_calib.y));
    json_append_member(parent, mtm_calib_z, json_mknumber((double) test.mtm_calib.z));
    json_append_member(parent, coil_current_x, json_mknumber((double) test.coil_current.x));
    json_append_member(parent, coil_current_y, json_mknumber((double) test.coil_current.y));
    json_append_member(parent, coil_current_z, json_mknumber((double) test.coil_current.z));
    json_append_member(parent, coil_temp_x, json_mknumber((double) test.coil_temp.x));
    json_append_member(parent, coil_temp_y, json_mknumber((double) test.coil_temp.y));
    json_append_member(parent, coil_temp_z, json_mknumber((double) test.coil_temp.z));
}

/* iMTQ-specific functions */
KADCSStatus k_imtq_get_system_state(imtq_state * state)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_STATE;

    if (state == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) state,
                                sizeof(imtq_state), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ system state: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_raw_mtm(imtq_mtm_msg * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_MTM_RAW;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_mtm_data), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ MTM data (raw): %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_calib_mtm(imtq_mtm_msg * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_MTM_CALIB;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_mtm_data), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ MTM data (calibrated): %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_coil_current(imtq_coil_current * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_CURRENT;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_coil_current), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ coil currents: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_coil_temps(imtq_coil_temp * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_TEMPS;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_coil_temp), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ coil temperatures: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_dipole(imtq_dipole * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_DIPOLE;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_dipole), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ command actuation dipole: %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_test_results_single(imtq_test_result_single * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_TEST;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_test_result_single), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr,
                "Failed to get iMTQ self-test result (single-axis): %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_test_results_all(imtq_test_result_all * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_TEST;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_test_result_all), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr,
                "Failed to get iMTQ self-test result (all-axes): %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_detumble(imtq_detumble * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_DETUMBLE;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_detumble), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ detumble data: %d\n", status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_raw_housekeeping(imtq_housekeeping_raw * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_HOUSE_RAW;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_housekeeping_raw), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ housekeeping data (raw): %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_get_eng_housekeeping(imtq_housekeeping_eng * data)
{
    KADCSStatus status = ADCS_OK;
    uint8_t     cmd    = GET_HOUSE_ENG;

    if (data == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(&cmd, 1, (uint8_t *) data,
                                sizeof(imtq_housekeeping_eng), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr,
                "Failed to get iMTQ housekeeping data (engineering): %d\n",
                status);
        return status;
    }

    return ADCS_OK;
}
