#ifndef CNC_TYPES_H
#define CNC_TYPES_H

#include <stdint.h>
#include <stdbool.h>

#define LIB_FORMAT_STR "/lib%s.so"

#define MODULE_REGISTRY_DIR YOTTA_CFG_CNC_REGISTRY_DIR LIB_FORMAT_STR

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

//The size of all the members of the command packet, except the output field
//The packet must fit into the CSP MTU or bad things will happen
#define CMD_PACKET_MEMBER_SIZE sizeof(int) + sizeof(CNCAction) + CMD_PACKET_CMD_NAME_LEN

#ifdef YOTTA_CFG_CNC_RES_PACKET_STDOUT_LEN
#define RES_PACKET_STDOUT_LEN        YOTTA_CFG_CNC_CMD_PACKET_ARG_LEN
#else
#define RES_PACKET_STDOUT_LEN     MTU - CMD_PACKET_MEMBER_SIZE
#endif

#define RESPONSE_TYPE_COMMAND_RESULT 0
#define RESPONSE_TYPE_PROCESSING_ERROR 1


typedef enum
{
    EXECUTE = 0,
    STATUS,
    OUTPUT,
    HELP
} CNCAction;


typedef struct arguments
{
    int arg_count;
    CNCAction action;
    char cmd_name[CMD_PACKET_CMD_NAME_LEN];
    char * args[CMD_PACKET_ARG_LEN];
} CNCCommandPacket;


typedef struct
{
    uint8_t return_code;
    double  execution_time;
    char    output[RES_PACKET_STDOUT_LEN]; //TODO: Figure out optimal size
} CNCResponsePacket;


// Used inside the daemon to track and provide error messages back to the client
typedef struct
{
    CNCCommandPacket  * command_packet;
    CNCResponsePacket * response_packet;
    bool err;
    char output[RES_PACKET_STDOUT_LEN];
} CNCWrapper;

#endif
