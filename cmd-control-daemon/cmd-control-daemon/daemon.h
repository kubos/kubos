#ifndef DAEMON_H
#define DAEMON_H
#include <csp/csp.h>
#include <command-and-control/types.h>

bool get_command(csp_socket_t* sock, char * command);

bool run_command(cnc_command_packet * command, cnc_response_packet * response); 

bool parse (char * args, cnc_command_packet * my_arguments);

void send_usage_error(cnc_command_packet * command);

#endif
