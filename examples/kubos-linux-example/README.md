# KubOS Linux Example App

This is a simple application built on top of the [KubOS Linux Platform](https://github.com/kubostech/kubos-linux-build) demonstrating some basic functionality from KubOS Linux (CSP). 

This is the default application included when the `kubos init --linux` command is executed. It is intended as a basic example of how an application could be written to run on a KubOS Linux system.

## Project Overview

The application contains a CSP interaction between client and server tasks.

Every 200ms, the CSP client thread pings the CSP server’s address to see if it is available, and then connects and sends a CSP packet containing the message “Hello World”.

The CSP server thread checks for connections on the CSP port and then prints any received messages to STDOUT.
  
## Project Requirements

This project can be built and run natively within a Kubos Vagrant image. No board is required for execution.

## Application Setup

Nagivate to your [Kubos Vagrant image](docs.kubos.co/sdk-installing.html) and clone this project's parent repo into a desired directory

    $ git clone http://github.com/kubostech/kubos my-kubos
    
Navigate to this project's folder

    $ cd my-kubos/examples/kubos-linux-example
    
Run the linking command to set up the project environment

    $ kubos link --all
    
Set the target for your board

    $ kubos target x86-linux-native
    
Build the project

    $ kubos build
    
Execute the project

    $ kubos flash
    
The output should look like this:

    Initializing CSP
    Starting example tasks
    Ping result 44 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 8 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 86 [ms]
    Packet received on MY_PORT: Hello World

Press `CTRL+C` to stop execution
    
## Resources

For more information about the SDK, see our docs:

- [Kubos Docs](http://docs.kubos.co)
- [Installing the Kubos SDK](http://docs.kubos.co/latest/sdk-installing.html)
- [Kubos SDK Cheatsheet](http://docs.kubos.co/latest/sdk-cheatsheet.html) 
- [Kubos CLI Command Reference](http://docs.kubos.co/latest/sdk-reference.html) 
- [Kubos Project Configuration](http://docs.kubos.co/latest/sdk-project-config.html)
- [ISIS-OBC Guide](http://docs.kubos.co/latest/working-with-the-iobc.html)

    
    
