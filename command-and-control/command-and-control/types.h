#ifndef CNC_H
#define CNC_H

#include <stdint.h>

typedef enum
{
    execute = 0,
    status,
    version,
    help
} cnc_action;


typedef struct arguments
{
    int arg_count;
    cnc_action action;
    char * cmd_name;
    char * args[10]; //TODO: Make a dynamic number of arguments
} cnc_cmd_packet;


typedef struct
{
    uint8_t return_code;
    double  execution_time;
    char    output[10]; //TODO: Make Dynamic
} cnc_res_packet;


#endif
