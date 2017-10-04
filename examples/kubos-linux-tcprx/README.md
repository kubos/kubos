# TCP RX with KubOS Linux

This is a demo program to test receiving TCP data over a valid IP connection (the ethernet port for the Pumpkin MBM2 and Beaglebone Black targets)

The program will wait for a client to connect over the socket, then read in any messages and send back a reply.

By default, port 3456 will be used for the connection.