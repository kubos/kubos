# KubOS RT Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/KubOS-rt) demonstrating some basic functionality from KubOS-RT (CSP, UART, FreeRTOS). 

This is the default application included when the `kubos init` command is executed. It is intended to provide an overview of several components as they might run in a KubOS RT application.

Four threads are created:
  - A CSP server
  - A CSP client
  - A button poll
  - An interval print

The interval thread prints out “echo, x={n}” to the default UART port, where {n} is a basic counter.

The button poll thread continually polls the board’s button to see if it has been pressed. If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. If found, it connects to the CSP server’s port and sends a message “Hello World”.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).
