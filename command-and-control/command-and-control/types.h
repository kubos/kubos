#ifndef CNC_H
#define CNC_H

typedef enum
{
    execute = 0,
    status,
    version,
    help
} cnc_action;


typedef struct
{
    cnc_action action;
    char args[4]; //TODO: Make Dynamic
} cnc_cmd_packet;


typedef struct
{
    uint8_t return_code;
    double  execution_time;
    char output[10]; //TODO: Make Dynamic
} cnc_res_packet;



#endif
