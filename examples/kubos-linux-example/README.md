# KubOS Linux Example App

This is a simple application built on top of the [KubOS Linux Platform](https://github.com/kubostech/kubos-linux-build) 
demonstrating some basic functionality from KubOS Linux (CSP). 

This is the default application included when the `kubos init --linux` command is executed. It is intended as a basic example of how an application could be written to run on a KubOS Linux system.

The application contains a CSP interaction between client and server tasks.

Every 200ms, the CSP client thread pings the CSP server’s address to see if it is available, and then connects and sends a CSP packet containing the message “Hello World”.

The CSP server thread checks for connections on the CSP port and then prints any received messages to STDOUT.

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).