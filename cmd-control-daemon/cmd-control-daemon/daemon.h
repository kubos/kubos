#ifndef DAEMON_H
#define DAEMON_H
#include <csp/csp.h>
#include <command-and-control/types.h>

char* get_command(csp_socket_t* sock);

cnc_res_packet * run_command(cnc_cmd_packet * command);

cnc_cmd_packet * parse (char * args);

#endif
