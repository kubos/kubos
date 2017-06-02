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

#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#define DEFAULT_STR_LEN 20

#ifdef YOTTA_CFG_CSP_MTU
#define MTU      YOTTA_CFG_CSP_MTU
#else
#define MTU      250
#endif

#ifdef YOTTA_CFG_CNC_CMD_MAX_NUM_ARGS
#define CMD_MAX_NUM_ARGS          YOTTA_CFG_CNC_CMD_MAX_NUM_ARGS
#else
#define CMD_MAX_NUM_ARGS          10
#endif

#ifdef YOTTA_CFG_CNC_CMD_MAX_ARG_LEN
#define CMD_MAX_ARG_LEN          YOTTA_CFG_CNC_CMD_MAX_ARG_LEN
#else
#define CMD_MAX_ARG_LEN           DEFAULT_STR_LEN
#endif

#ifdef YOTTA_CFG_CNC_CMD_PACKET_CMD_NAME_LEN
#define CMD_PACKET_CMD_NAME_LEN   YOTTA_CFG_CNC_CMD_PACKET_CMD_NAME_LEN
#else
#define CMD_PACKET_CMD_NAME_LEN   DEFAULT_STR_LEN
#endif

#ifdef YOTTA_CFG_CNC_CMD_PACKET_ARG_LEN
#define CMD_PACKET_ARG_LEN        YOTTA_CFG_CNC_CMD_PACKET_ARG_LEN
#else
#define CMD_PACKET_ARG_LEN        DEFAULT_STR_LEN
#endif

#ifdef YOTTA_CFG_CNC_CMD_PACKET_NUM_ARGS
#define CMD_PACKET_NUM_ARGS        YOTTA_CFG_CNC_CMD_PACKET_ARG_LEN
#else
#define CMD_PACKET_NUM_ARGS        5
#endif


//The size of all the members of the command packet, except the output field
//The packet must fit into the CSP MTU or bad things will happen
#define CMD_PACKET_MEMBER_SIZE sizeof(int) + CMD_PACKET_CMD_NAME_LEN

#ifdef YOTTA_CFG_CNC_RES_PACKET_STDOUT_LEN
#define RES_PACKET_STDOUT_LEN        YOTTA_CFG_CNC_CMD_PACKET_ARG_LEN
#else
#define RES_PACKET_STDOUT_LEN     MTU - CMD_PACKET_MEMBER_SIZE
#endif

#define MESSAGE_TYPE_COMMAND_INPUT      0
#define RESPONSE_TYPE_COMMAND_RESULT    1
#define RESPONSE_TYPE_PROCESSING_ERROR  2

typedef struct arguments
{
    int arg_count;
    char cmd_name[CMD_PACKET_CMD_NAME_LEN];
    char args[CMD_PACKET_NUM_ARGS][CMD_PACKET_ARG_LEN];
} CNCCommandPacket;


typedef struct
{
    uint8_t return_code;
    double  execution_time;
    char    output[RES_PACKET_STDOUT_LEN];
} CNCResponsePacket;


// Used inside the daemon to track and provide error messages back to the client
typedef struct
{
    CNCCommandPacket  * command_packet;
    CNCResponsePacket * response_packet;
    bool err;
    char output[RES_PACKET_STDOUT_LEN];
} CNCWrapper;

//The CborDataWrapper keeps the length and buffer data together.
typedef struct
{
    size_t    length;
    uint8_t * data;
} CborDataWrapper;

