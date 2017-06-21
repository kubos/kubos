# Kubos CSP Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/KubOS-rt) demonstrating CSP over the Kubos HAL's UART interface.

This application shows an example CSP (Cubesat Space Protocol) interaction between client and server tasks.

Three threads are created:
  - A CSP server
  - A CSP client
  - A button poll

The button poll thread continually polls the board’s button to see if it has been pressed. If it has, a notification is added to a dedicated queue.

The CSP client thread checks for messages on the button queue. If found, it connects to the CSP server’s port and sends a message “Hello World”.

The CSP server thread checks for connections on the CSP port and then blinks the green LED if any messages are received.

The CSP connection configuration is set using the included config.json file.

UART Bus: K_UART1

STM32F407 Discovery:
  - TX - PA9
  - RX - PA10

MSP430F5529 Launchpad
  - TX - P3.3
  - RX - P3.4

Notes:
1.  To successfully run this project, two boards must be used. One should use this project with the included config.json file.
    The other should use this project, but should have config.json file edited to reverse the "my_address" and "target_address" values.
2.  Due to a current peculiarity with the debouncing logic, the button must be pressed twice in order for the 'send message' event to occur.

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).
