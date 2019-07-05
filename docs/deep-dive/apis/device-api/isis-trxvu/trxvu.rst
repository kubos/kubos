Using the ISIS TRXVU Radio in a Kubos Project
=============================================

This document covers the particular capabilities and requirements of the Kubos API for the
`ISIS TRXVU <https://www.isispace.nl/product/isis-uhf-downlink-vhf-uplink-full-duplex-transceiver/>`__ radio.

.. toctree::
    :maxdepth: 1
    
    ISIS TRXVU API <trxvu_api>

Reference Documents
-------------------

ISIS
~~~~

    - TRXVU Interface Control Document - The main hardware guide for the radio
    - TRXVU Options Sheet - Allows customers to specify non-default options that their radio should be manufactured with

Kubos
~~~~~

    - :doc:`Creating a Kubos Project <../../../../tutorials/first-obc-project>`
    - :doc:`Using Kubos Linux <../../../../ecosystem/linux-docs/using-kubos-linux>`
    - :doc:`Working with the iOBC <../../../../obc-docs/iobc/working-with-the-iobc>`

Run-Time Radio Configuration
----------------------------

The ISIS TRXVU radio will need to be initialized via :cpp:func:`k_radio_init` with a few different options.
These option values should match what was specified in your TRXVU options sheet.

The :cpp:func:`k_radio_init` function takes the following arguments:

    - ``bus`` - The I2C bus device name (ex. ``"/dev/i2c-0"``)
    - ``tx`` - The transmitter's properties
    - ``rx`` - The receiver's properties
    - ``timeout`` - The radio's watchdog timeout (in seconds)

The transmitter and receiver property structures contain the following elements:

    - ``addr`` - The I2C address of the component
    - ``max_size`` - The maximum payload size
    - ``max_frames`` - The maximum number of frames that the component's buffer can store

The radio also supports in-flight configuration changes. These changes can be made using the :cpp:func:`k_radio_configure` function.

.. note::

    **These configuration changes will not persist through a radio reboot, including one triggered by the watchdog**

The function will take a pointer to a :cpp:type:`radio_config` structure.

Unneeded configuration parameters may be left with zero values to indicate that they should not be changed.

For example:

.. code-block:: c

    #include "isis-trxvu-api/trxvu.h"
    
    radio_config config = {0};
    config.data_rate = RADIO_TX_RATE_1200;
    strncpy(config.to.ascii, "MAJORT", sizeof(config.to.ascii));
    strncpy(config.from.ascii, "HMLTN1", sizeof(config.from.ascii));

    k_radio_configure(&config);
    
Sending a Message
-----------------

In order to write a message to the radio's transmit buffer, call the :cpp:func:`k_radio_send` function.

The function takes three parameters:

- A pointer to the message to be sent
- The length of the message
- A pointer to a response byte

The function will return one of two values:

- RADIO_OK - Indicating a message was successfully received
- RADIO_ERROR - Indicating that something went wrong during the send process

The TRXVU will automatically package any message written to it into either an AX.25 or HDLC frame
(as specified in the options sheet). As a result, only the payload message needs to be written.

The response byte will have one of two values:

- 0xFF - Indicates that the radio was unable to add the message to the transmit buffer. Possible reasons:

    - The transmit buffer is full
    - The message length given was zero
    - The message length given was greater than the maximum payload size

- 0x{nn} - The number of remaining transmission buffer slots

For example:

.. code-block:: c

    #include "isis-trxvu-api/trxvu.h"
    
    KRadioStatus status;
    uint8_t message[] = "Radio Test Message";
    uint8_t len = sizeof(message);
    uint8_t response;

    status = k_radio_send(message, len, &response);
    
Overriding Call-Signs
~~~~~~~~~~~~~~~~~~~~~

To send a message with non-default AX.25 call-signs, use the :cpp:func:`k_radio_send_override` function instead.

For example:

.. code-block:: c

    #include "isis-trxvu-api/trxvu.h"
    
    KRadioStatus status;
    uint8_t response;
    uint8_t message[] = "Radio Test Message";
    uint8_t len = sizeof(message);

    ax25_callsign to = {0};
    ax25_callsign from = {0};

    strncpy(to.ascii, "MAJORT", sizeof(((ax25_callsign *)0)->ascii));
    strncpy(from.ascii, "HMLTN1", sizeof(((ax25_callsign *)0)->ascii));

    status = k_radio_send_override(to, from, message, len, &response);
    
Receiving a Message
-------------------

In order to read a message from a radio's receive buffer, call the :cpp:func:`k_radio_recv` function.

The function takes three parameters:

    - A pointer to a :cpp:type:`radio_rx_header` structure where the meta-data should be put
    - A pointer to a buffer where the message payload should be put
    - *(Optional. Value of 'NULL' may be passed)* A pointer to a 2-byte variable which will be updated to contain the length of the message received.

The function will return one of three values:

- ``RADIO_OK`` - Indicating a message was successfully received
- ``RADIO_RX_EMPTY`` - Indicating there are no messages to receive
- ``RADIO_ERROR`` - Indicating that something went wrong during the receive process

If the receive process is successful, the retrieved message will be automatically removed from the receive buffer.
If the receive process fails for any reason, the receiver buffer will not be modified.

The message will be returned in the :cpp:member:`radio_rx_message.message <radio_rx_message::message>` structure member.

Message Meta-Data
~~~~~~~~~~~~~~~~~

If the function completes successfully, there will be three meta-data fields which can be examined:

    - :cpp:member:`msg_size <radio_rx_message::msg_size>` - The length of the message received
    - :cpp:member:`doppler_offset <radio_rx_message::doppler_offset>` - The doppler shift on the packet at the time it was received by the radio
      (convert to human-readable units using the :cpp:func:`get_doppler_offset` function)
    - :cpp:member:`signal_strength <radio_rx_message::signal_strength>` - The measured Received Signal Strength Indicator (RSSI) at the time the packet was received by the radio
      (convert to human-readable units using the :cpp:func:`get_signal_strength` function)

Example
~~~~~~~

.. code-block:: c

    #include "isis-trxvu-api/trxvu.h"
    
    KRadioStatus status;
    radio_rx_header header = { 0 };
    // Allocate room for the maximum payload size
    uint8_t message[200] = { 0 };

    status = k_radio_recv(&header, message, NULL);
    if (status == RADIO_RX_EMPTY)
    {
        printf("No messages to receive\n");
    }
    else if (status == RADIO_OK)
    {
        printf("Received message(%d %fHz %fdBm): %s\n",
                len, get_doppler_offset(header.doppler_offset), get_signal_strength(header.signal_strength),
                message);
    }
    else
    {
        printf("Failed to send message: %d\n", status);
    }

Telemetry
---------

There is a selection of telemetry values which can be read from the radio using the :cpp:func:`k_radio_get_telemetry` function.

The function takes two parameters:

- A pointer to a :cpp:type:`radio_telem` structure
- The requested telemetry type, selected by passing one of the :cpp:type:`RadioTelemType` values

The values can then be read by specifying the :cpp:type:`radio_telem` sub-structure and the desired member.

For example::

    #include "isis-trxvu-api/trxvu.h"

    trx_prop tx = {
            .addr = 0x60,
            .max_size = TX_SIZE,
            .max_frames = 40,
    };
    trx_prop rx = {
                .addr = 0x61,
                .max_size = RX_SIZE,
                .max_frames = 40,
    };

    k_radio_init("/dev/i2c-0", tx, rx, 10);
    
    radio_telem tx_telem = {0};
    k_radio_get_telemetry(&tx_telem, RADIO_TX_TELEM_ALL);
    
    printf("TX supply current: %f mA\n", get_current(tx_telem.trxvu_telem_raw.supply_current));
    
Converting to Human-Readable Values
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The values in the :cpp:type:`trxvu_tx_telem_raw` and :cpp:type:`trxvu_rx_telem_raw` structures are all two-byte raw ADC values.
These values can be converted as desired into human-readable `float` values.

    - :cpp:func:`get_voltage`
    - :cpp:func:`get_current`
    - :cpp:func:`get_temperature`
    - :cpp:func:`get_doppler_offset`
    - :cpp:func:`get_signal_strength`
    - :cpp:func:`get_rf_power_dbm`
    - :cpp:func:`get_rf_power_mw`

Transmitter
~~~~~~~~~~~

RADIO_TX_TELEM_ALL
^^^^^^^^^^^^^^^^^^

Returns the current measurements of all the transmitter's telemetry channels:

- Instantaneous RF reflected power from transmitter port
- Instantaneous RF forward power from transmitter port
- Supply voltage
- Total supply current
- Power amplifier temperature
- Local oscillator temperature

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`trxvu_tx_telem_raw` sub-structure.

RADIO_TX_TELEM_LAST
^^^^^^^^^^^^^^^^^^^
Returns the telemetry channels that were sampled during the last frame transmission:

- Instantaneous RF reflected power from transmitter port
- Instantaneous RF forward power from transmitter port
- Supply voltage
- Total supply current
- Power amplifier temperature
- Local oscillator temperature

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`trxvu_tx_telem_raw` sub-structure.

RADIO_TX_UPTIME
^^^^^^^^^^^^^^^

Returns the amount of time, in seconds, that the transmitter portion of the radio has been active.

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`uptime` sub-structure.

RADIO_TX_STATE
^^^^^^^^^^^^^^

Returns the current state of the transmitter.

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`tx_state` sub-structure.

The returned value contains several flags:

- :c:macro:`RADIO_STATE_IDLE_ON` - Indicates that the transmitter will remain on while idle
- :c:macro:`RADIO_STATE_BEACON_ACTIVE` - Indicates that the transmitter has been set up to send an automatic beacon message
- ``RADIO_STATE_RATE_{1200|2400|4800|9600}`` - Indicates the transmitter's data rate

Detecting Transmitter Rate
##########################

The ``RADIO_STATE_RATE_*`` flags are actually composed of two flag bits. In order to properly detect the rate, use this logic:

.. code-block:: c

    #include "isis-trxvu-api/trxvu.h"

    radio_telem state = {0};
    k_radio_get_telemetry(&state, RADIO_TX_STATE);

    uint8_t rate_flag = state.tx_state >> 2;
    if (rate_flag == RADIO_STATE_RATE_9600)
    {
        //Data rate is 9600bps
    }
    else if (rate_flag == RADIO_STATE_RATE_4800)
    {
        //Data rate is 4800bps
    }
    else if (rate_flag == RADIO_STATE_RATE_2400)
    {
        //Data rate is 2400bps
    }
    else
    {
        //Data rate is 1200bps
    }
    
Receiver
~~~~~~~~

RADIO_RX_TELEM_ALL
^^^^^^^^^^^^^^^^^^

Returns the current measurements of all the receiver's telemetry channels:

- Total supply current
- Power amplifier temperature
- Local oscillator temperature
- Instantaneous received signal Doppler offset at the receiver port
- Instantaneous received signal strength at the receiver port
- Supply voltage

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`trxvu_rx_telem_raw` sub-structure.

RADIO_RX_UPTIME
^^^^^^^^^^^^^^^

Returns the amount of time, in seconds, that the receiver portion of the radio has been active.

A pointer to a :cpp:type:`radio_telem` structure should be passed to the :cpp:func:`k_radio_get_telemetry` function.
The values can then be read from the :cpp:type:`uptime` sub-structure.

Special Functions
-----------------

The ISIS TRXVU radio has a few functions which are not common to all radios:

Beacon
~~~~~~

The radio can be set up to automatically transmit a beacon message at a specified interval. This beacon can either be
defined in the TRXVU options sheet or with the :cpp:func:`k_radio_configure` function, but not both.

.. note:: 

    The beacon will be disabled if another message is transmitted by the radio. The beacon will then need to be re-enabled
    by running the :cpp:func:`k_radio_configure` function again, or by restarting the tranmitter (if the beacon was pre-defined).

For example::

    #include "isis-trxvu-api/trxvu.h"

    radio_config config = {0};
    char beacon_msg[] = "Radio Beacon Message";

    config.beacon.interval = 5;
    config.beacon.msg = beacon_msg;
    config.beacon.len = sizeof(beacon_msg);
    
    k_radio_configure(&config);
    
Beacon Override
^^^^^^^^^^^^^^^

If, for some reason, the beacon needs to be set up with different callsigns than the radio was configured with (either with the options
sheet, or with :cpp:func:`k_radio_configure`), use the :cpp:func:`k_radio_beacon_override` function.

For example::

    #include "isis-trxvu-api/trxvu.h"

    radio_tx_beacon beacon = {0};
    ax25_callsign to = {0};
    ax25_callsign from = {0};
    char beacon_msg[] = "Radio Beacon Message";

    beacon.interval = 5;
    beacon.msg = beacon_msg;
    beacon.len = sizeof(beacon_msg);
    strncpy(to.ascii, "MAJORT", sizeof(((ax25_callsign *)0)->ascii));
    strncpy(from.ascii, "HMLTN1", sizeof(((ax25_callsign *)0)->ascii));
    
    k_radio_beacon_override(to, from, beacon);

Watchdogs
~~~~~~~~~

The transmitter and receiver portions of the radio both have a watchdog the may need to be kicked periodically.
These watchdogs and the timeout interval are configured before delivery based on the options sheet.

Since the timeout, no matter the value, will be the same for both the transmitter and receiver, one unifying
function, :cpp:func:`k_radio_watchdog_kick`, can be used to kick both watchdogs.

Alternatively, the :cpp:func:`k_radio_watchdog_start` function can be called to start a thread which will automatically
kick each watchdog every (interval/3) seconds. If this function is used, then the :cpp:func:`k_radio_watchdog_stop`
function should be used to stop and cleanup the thread before the user's program exits. The `config.json` file
for the project should include the :json:object:`watchdog` parameter to specify the watchdog's timeout interval if it
deviates from the default of 60 seconds.

.. note::

    Any communication with the radio will automatically reset the appropriate watchdog. If communication is routinely occuring
    within the watchdog's timeout interval, then there is no need to manually establish a watchdog thread.

For example::

    #include "isis-trxvu-api/trxvu.h"
    
    int main(void)
    {  
        trx_prop tx = {
                .addr = 0x60,
                .max_size = TX_SIZE,
                .max_frames = 40,
        };
        trx_prop rx = {
                    .addr = 0x61,
                    .max_size = RX_SIZE,
                    .max_frames = 40,
        };
    
        k_radio_init("/dev/i2c-0", tx, rx, 10);
        k_radio_watchdog_start();
        
        //Program logic
        
        k_radio_watchdog_stop();
        k_radio_terminate();
    
        return 0;
    }
    
Reset
~~~~~

The radio can be manually reset using the :cpp:func:`k_radio_reset` function.
There are two available reset types: a full hardware reset (``RADIO_HARD_RESET``), and a software-only reset (``RADIO_SOFT_RESET``).

To specify which reset is desired, pass the appropriate :cpp:type:`RadioResetType`  value to the function.

For example::

    #include "isis-trxvu-api/trxvu.h"
    
    trx_prop tx = {
            .addr = 0x60,
            .max_size = TX_SIZE,
            .max_frames = 40,
    };
    trx_prop rx = {
                .addr = 0x61,
                .max_size = RX_SIZE,
                .max_frames = 40,
    };

    k_radio_init("/dev/i2c-0", tx, rx, 10);
    k_radio_reset(RADIO_SOFT_RESET);

