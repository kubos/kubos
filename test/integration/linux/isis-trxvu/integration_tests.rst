Integration Tests for the ISIS TRXVU Radio API
==============================================

These are the tests that can be run against the physical iOBC<->TRXVU<->RF Checkout Box
setup.

Setup
-----

Set up an iOBC with a TRXVU connected to it. 

The iOBC should be set up with 3.3V and a max current of 150mA.

The TRXVU should be set up with 8V and a max current of 600mA.

Turn on the iOBC's power first and make sure it boots successfully before powering the radio.
(This isn't a required step, but it increases the safety and amount of logging available in 
case of an issue)

Connect the RF checkout box's USB to your computer.

Project
~~~~~~~

Build and flash the ``radio-test`` project in this folder onto your iOBC.

Use `minicom kubos` to connect to the board and then run the ``radio-test`` program.
It should give you a dialog asking for a command to run.

RF Checkout Box
~~~~~~~~~~~~~~~

Find a Windows machine (it's currently the only supported OS).

Follow Chapter 4 of the `RF Checkout Box User Manual` to install the checkout box software.

Open the `RF Checkout Box` program and select "CheckoutBox". 
The program will take 10-20 seconds to load the initial configuration.

Click the 'ASCII Output' option to cause the ASCII values of the message payloads to be
displayed, rather than the hex values.

Click the 'Log' option to turn on logging. The output files will be available in `$HOME/.isis`.
The `cb_downlink.log` files store the received messages, the `cb_uplink.log` files store the transmitted
messages, and the `checkoutbox.txt` file stores the software initialization messages.

Update the settings to match those specified in the radio's options sheet. The values will likely
look like this:

- Downlink

  - Scheme: BPSK-G3RUH
  - Datarate: 1200

- Uplink

  - Scheme: AFSK
  - Datarate: 1200

Send Message
------------

Enter ``s`` in the ``radio_test`` prompt to send a message. It should appear in the checkout box's
"RX Terminal" page. The `to` and `from` callsigns should match what was defined in the radio's options sheet.
The message payload should be "Radio Test Message".

Error Handling
~~~~~~~~~~~~~~

If no message appears, try the following steps:

- Resend the message. Occasionally packets might be dropped.
- Navigate to the ``FFT`` tab of the software and resend the message. Verify that you can see the waveform change.
  
  - If the waveform appears, identify the location of the apex. Update the ``Downlink`` frequency to use this value.
  - If the waveform doesn't appear, verify that you are starting with the correct ``Downlink`` settings.

Receive Message
---------------

Press the "Send Text" button. Optionally, change the test string before sending.

Enter ``f`` in the ``radio_test`` program.

Verify: 

- The test string is displayed
- The length displayed matches the length of the test string
- The doppler offset and signal strength values are not zero

Error Handling
~~~~~~~~~~~~~~

If no message appears, try the following steps:

- Resend the message. Occasionally packets might be dropped.
- Verify that you are starting with the correct ``Uplink`` settings.

Telemetry
---------

TX
~~

Enter ``t`` in the ``radio_test`` program.

Verify that the "TRXVU Transmitter Telemetry - Last" section contains non-empty values.

Verify that all values seem reasonable (ex. voltage just under 8V).

RX
~~

Enter ``r`` in the ``radio_test`` program.

Verify that all values seem reasonable (ex. voltage just under 8V).

Watchdog
--------

Enter ``t`` in the ``radio_test`` program. Verify that the values in the 
"TRXVU Transmitter Telemetry - Last" output aren't empty (if they are, enter ``s``
to send a message and then retry).

Leave the radio alone for several minutes.

Get the telemetry again and verify that the "TRXVU Transmitter Telemetry - Last"
are the same (so, not empty). This indicates that the watchdog timeout interval
passed without the watchdog starving.

Reset
-----

Enter ``z`` in the ``radio_test`` program, followed by ``t``.

The "TRXVU Transmitter Telemetry - Last" output values should now be empty and the
"TX Uptime" field should be close to zero.

Override Functions
------------------

Enter ``o`` in the ``radio_test`` program.

The "RX Terminal" tab should show two new messages, both with the callsigns "KBSTO-1" and "KBSFRM-2".
The first message should be "Beacon Message" and the second should be "Radio Test Message".

Configuration
-------------

.. note::

    These tests should be run several times, in various combinations, to ensure that leaving some configuration options
    empty/unset doesn't interfere with anything and that the different data rates all work.
    
    The ``z`` option can be used to reset the radio back to the default configuration.

Enter ``c`` in the ``radio_test`` program.

Callsigns
~~~~~~~~~

Selecting these options will change the "to" callsign to "MJRTOM" and the "from" callsign to "HMLTN1".
This can be verified by sending a message and checking the output in the "RX Terminal" tab.

Data Rate
~~~~~~~~~

Select one of the data rate options. Adjust the ``Downlink`` Datarate value to match.
After configuration is completed, send a message and verify that is successfully received by the checkout box.

Beacon
~~~~~~

Enabling the beacon should cause the "Radio Beacon Message" payload to be sent every five seconds.
Sending an addition message with the ``s`` option should disable the beacon.

Idle On
~~~~~~~

Enabling 'Idle On' should cause the current draw of the radio to increase to around 400mA.

Entering 'n' in the prompt will set the configuration to "Idle Off".

Any other input will cause the idle state to be unaffected.

TX State
~~~~~~~~

After changing the configuration, enter the ``t`` option and verify that the data in the "TRXVU Transmitter Telemetry - State"
section matches the options that were set.

.. note::

    Reminder: If the beacon has been activated, but then a different message is sent, the beacon will be automatically
    disabled. This will be reflected in the "TX Beacon" field.
