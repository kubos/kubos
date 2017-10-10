/*
 * KubOS Linux
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
 *
 * Ethernet TX example using TCP
 *
 * Usage: kubos-linux-tcptx <ip_addr> <port>
 *
 * This program sends "Test message!" to the specified IP address/port and
 * then reads back a reply message.
 *
 */

#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char * argv[])
{
    int                client_fd, port, len;
    struct sockaddr_in server;

    char tx_buf[] = "Test message!\n";
    char rx_buf[256] = {0};

    if (argc < 3)
    {
        fprintf(stderr, "Usage: %s ip_addr port\n", argv[0]);
        exit(0);
    }

    /* Set socket type to IPv4 */
    server.sin_family = AF_INET;

    /* Convert text version of IP address to usable version */
    if(inet_pton(AF_INET, argv[1], (void *) &server.sin_addr.s_addr) != 1)
    {
        perror("Error converting IP address");
        exit(-1);
    }

    /* Convert text version of port */
    server.sin_port = htons(atoi(argv[2]));

    /* Open a socket to use for TCP communication */
    client_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (client_fd < 0)
    {
        perror("Error getting socket");
        exit(-1);
    }

    /* Connect to the endpoint */
    if (connect(client_fd, (struct sockaddr *) &server, sizeof(server)) < 0)
    {
        perror("Error connecting to address");
        close(client_fd);
        exit(-1);
    }

    /* Send the message */
    len = write(client_fd, tx_buf, strlen(tx_buf));
    if (len < 0)
    {
        perror("Error writing to address");
        close(client_fd);
        exit(-1);
    }

    /* Wait for the reply */
    len = read(client_fd, rx_buf, 255);
    if (len < 0)
    {
        perror("Error reading from socket");
        close(client_fd);
        exit(-1);
    }

    printf("Reply received: %s\n", rx_buf);

    close(client_fd);

    return 0;
}
