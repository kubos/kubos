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
 * ISIS iMTQ API - Configuration Commands
 */

#include <isis-imtq-api/imtq.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

KADCSStatus k_adcs_configure(uint16_t param, imtq_config_value value)
{
    KADCSStatus status      = ADCS_OK;
    KADCSStatus imtq_status = ADCS_OK;

    /* Send the request */
    imtq_status = k_imtq_set_param(param, &value, NULL);
    if (imtq_status != ADCS_OK)
    {
      fprintf(stderr,
              "Failed to set iMTQ configuration parameter (%x): %d\n",
              param, imtq_status);
      status = ADCS_ERROR;
    }
    return status;
}

KADCSStatus k_imtq_get_param(uint16_t param, imtq_config_resp * response)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t    packet[3] = {
            GET_PARAM,
            param & 0xFF, param >> 8
    };

    if (param == 0 || response == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    status = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) response,
                                sizeof(imtq_config_resp), NULL);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to retrieve parameter (%x): %d\n", param,
                status);
        return status;
    }

    if (param != response->param)
    {
        fprintf(stderr, "Parameter mismatch - Sent: %x Received: %x\n", param,
                response->param);
        return ADCS_ERROR;
    }

    return ADCS_OK;
}

KADCSStatus k_imtq_set_param(uint16_t param, const imtq_config_value * value,
                             imtq_config_resp * response)
{
    KADCSStatus status = ADCS_OK;
    uint8_t    packet[3 + sizeof(imtq_config_value)] = {
            SET_PARAM,
            param & 0xFF, param >> 8,
            0
    };

    if (param == 0 || value == NULL)
    {
        return ADCS_ERROR_CONFIG;
    }

    memcpy(packet + 3, value, sizeof(imtq_config_value));

    if (response != NULL)
    {
        status
            = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) response,
                                 sizeof(imtq_config_resp), NULL);
    }
    else
    {
        imtq_resp_header header;
        status = kprv_imtq_transfer(packet, sizeof(packet),
                                    (uint8_t *) &header, sizeof(header), NULL);
    }

    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to set parameter (%x): %d\n", param, status);
        return status;
    }

    return status;
}

KADCSStatus k_imtq_reset_param(uint16_t param, imtq_config_resp * response)
{
    KADCSStatus status    = ADCS_OK;
    uint8_t    packet[3] = {
            RESET_PARAM,
            param & 0xFF,
            param >> 8
    };

    if (param == 0)
    {
        return ADCS_ERROR_CONFIG;
    }

    if (response != NULL)
    {
        status
            = kprv_imtq_transfer(packet, sizeof(packet), (uint8_t *) response,
                                 sizeof(imtq_config_resp), NULL);
    }
    else
    {
        imtq_resp_header header;
        status = kprv_imtq_transfer(packet, sizeof(packet),
                                    (uint8_t *) &header, sizeof(header), NULL);
    }

    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to reset parameter (%x): %d\n", param, status);
        return status;
    }

    return status;
}
