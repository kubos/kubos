#ifndef DAEMON_H
#define DAEMON_H
#include <csp/csp.h>
#include <command-and-control/types.h>

bool get_command(csp_socket_t* sock, char * command);

bool run_command(cnc_cmd_packet * command, cnc_res_packet * response); 

bool parse (char * args, cnc_cmd_packet * my_arguments);

#endif
