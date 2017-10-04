# TCP TX with KubOS Linux

This is a demo program to test sending TCP data over a valid IP connection (the ethernet port for the Pumpkin MBM2 and Beaglebone Black targets)

The program takes the IP address and port to send to as input parameters, then sends a test message to the requested end point.
It then waits for a reply message to be returned and exits.

	Usage: kubos-linux-tcptx <ip_addr> <port>