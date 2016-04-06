#ifndef K_MSG_H
#define K_MSG_H

#include <stdint.h>
#include "csp/csp.h"

typedef struct k_msg {
    uint16_t type;
    char * content;
} k_msg_t;

int k_msg_send(k_msg_t * m, csp_conn_t * conn);

#endif
