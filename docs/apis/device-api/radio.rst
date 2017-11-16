Using a Radio
=============

This document covers the general API for interacting with a supported radio. The underlying structure for radios varies
quite dramatically, so please refer to the appropriate specific radio document in conjunction with this one.

The specific radio documents will cover things like configuration options, available telemetry types, and system requirements.

Project Configuration
---------------------

The specific radio being used in a Kubos project should be specified in the ``radio`` parameter of the project's `config.json` file.
This tells the Kubos SDK to include the correct radio files when building the project.
The radio's subparameters may then be specified within the specific radio's structure.

For example::

    {
      "radio": {
        "trxvu": {
          "i2c_bus": K_I2C1
        }
      }
    }
    
Radio Initialization and Termination
------------------------------------

In order for a Kubos project to communicate with a radio, the ``k_radio_init`` function should be called. This opens the correct
underlying KubOS Linux device file. The function should return ``RADIO_OK`` if it completed successfully.

Similarly, before the project finishes executing, the ``k_radio_terminate`` function should be called to perform the appropriate
system cleanup.

For example:

.. code-block:: c

    #include "radio-api/radio.h"
    
    if (k_radio_init() != RADIO_OK)
    {
        return;
    }
    
    // Project logic
    
    k_radio_terminate();

Run-Time Radio Configuration
----------------------------

Some radios may support or require that the radio be configured during run-time. This can be done by passing a pointer to 
a ``radio_config`` structure to the ``k_radio_configure`` function.

Unneeded configuration parameters may be left with zero values to indicate that they should not be changed.

For example:

.. code-block:: c

    #include "radio-api/radio.h"
    
    radio_config config;
    config.data_rate = RADIO_RATE_1200;
    strncpy(config.to.ascii, "MAJORT", sizeof(config.to.ascii);
    strncpy(config.from.ascii, "HMLTN1", sizeof(config.from.ascii);

    k_radio_configure(&config);
    
Sending a Message
-----------------
In order to write a message to the radio's transmit buffer, call the ``k_radio_send`` function.

The function takes three parameters:

- A pointer to the message to be sent
- The length of the message
- A pointer to a response byte (varies by radio. For example, returning the number of bytes written)

The function will return one of two values:

- RADIO_OK - Indicating a message was successfully received
- RADIO_ERROR - Indicating that something went wrong during the send process

For example:

.. code-block:: c

    #include "radio-api/radio.h"
    
    KRadioStatus status;
    uint8_t message[] = "Radio Test Message";
    uint8_t len = sizeof(message);
    uint8_t response;

    status = k_radio_send(message, len, &response);
    
Receiving a Message
-------------------

In order to read a message from a radio's receive buffer, call the ``k_radio_recv`` function.

The function takes two parameters:

- A pointer to a ``radio_rx_message`` structure where the message will be put
- A pointer to a variable which will be updated to contain the length of the message received.

The function will return one of three values:

- RADIO_OK - Indicating a message was successfully received
- RADIO_RX_EMPTY - Indicating there are no messages to receive
- RADIO_ERROR - Indicating that something went wrong during the receive process

For example:

.. code-block:: c

    #include "radio-api/radio.h"
    
    KRadioStatus status;
    radio_rx_message buffer;
    uint8_t len;

    status = k_radio_recv(&buffer, &len);