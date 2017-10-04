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
 * Ethernet RX example using TCP
 *
 * Usage: kubos-linux-tcprx
 *
 * This program listens for TCP connections on port 3456, prints out anything
 * that is received, and then sends a reply message back to the sender.
 *
 */

#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define PORT 3456
#define MAX_WAIT 5

uint8_t quit;

int main(int argc, char * argv[])
{
    struct sockaddr_in server, client;
    int server_fd, client_fd, len;
    int status = 0;

    char buffer[256];
    char reply[] = "Test message from server\n";

    quit = 0;

    /* Create an IPv4 socket (AF_INET) for TCP communication (SOCK_STREAM) */
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == -1)
    {
        perror("Failed to get server socket");
        exit(-1);
    }

    /* Setup the socket info, including the port to bind to */
    server.sin_family      = AF_INET;
    server.sin_port        = htons(PORT);
    server.sin_addr.s_addr = INADDR_ANY;

    /* Bind the new socket to the desired port */
    if ((bind(server_fd, (struct sockaddr *) &server, sizeof(struct sockaddr)))
        != 0)
    {
        perror("Failed to bind server socket");
        exit(-1);
    }

    /* Listen for connections on this socket */
    if ((listen(server_fd, MAX_WAIT)) == -1)
    {
        perror("Failed to set up listener");
        exit(-1);
    }

    /* Receive connections loop */
    while (!quit)
    {
        socklen_t size = sizeof(struct sockaddr_in);

        if ((client_fd = accept(server_fd, (struct sockaddr *) &client, &size))
            == -1)
        {
            perror("Failed to accept client connection");
            status = -1;
            break;
        }

        printf("Received connection from %s\n", inet_ntoa(client.sin_addr));

        /* Receive messages loop */
        while (1)
        {
            /* Receive message */
            if ((len = recv(client_fd, buffer, sizeof(buffer), 0)) == -1)
            {
                perror("Error while receiving message");
                status = -1;
                quit   = 1;
                break;
            }
            else if (len == 0)
            {
                printf("Connection closed by client\n");
                break;
            }

            buffer[len] = '\0';
            printf("Message received: %s\n", buffer);

            /* Send reply */
            if ((send(client_fd, reply, strlen(reply), 0)) == -1)
            {
                perror("Error while sending message");
                quit = 1;
                break;
            }
            else
            {
                printf("Message being sent: %s\nNumber of bytes sent: %d\n",
                       reply, strlen(reply));
            }
        } /* End of message receive loop */

        close(client_fd);
    } /* End of connections loop */

    close(server_fd);

    return status;
}
