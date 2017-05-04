Kubos Project Configuration
===========================

   

config.json
-----------

The current configuration of a project can be seen using the ``kubos config`` command. 
Each configuration option in the output will have a comment showing the origin of the value.
Anything marked with "application's config.json" will have been taken from the project's `config.json` file.
All other comments will have "*-gcc", which indicates that that option is a default value taken from
the corresponding `target.json` file.

For example:

::

    $ kubos config
    
    {
      "test": false, // application's config.json
      "hardware": {
        "console": {
          "uart": "K_UART2", // msp430f5529-gcc
          "baudRate": 115200 // msp430f5529-gcc
        },
        "i2c": {
          "count": 2, // msp430f5529-gcc
          "defaults": {
            "bus": "K_I2C2", // msp430f5529-gcc
            "role": "K_MASTER", // kubos-gcc
            "clockSpeed": 100000, // kubos-gcc
            "addressingMode": "K_ADDRESSINGMODE_7BIT" // kubos-gcc
          },
          "i2c1": {},
          "i2c2": {}
        },
        "spi": {
          "count": 2, // msp430f5529-gcc
          "defaults": {
            "bus": "K_SPI1", // msp430f5529-gcc
            "role": "K_SPI_MASTER", // kubos-gcc
            "direction": "K_SPI_DIRECTION_2LINES", // kubos-gcc
            "dataSize": "K_SPI_DATASIZE_8BIT", // kubos-gcc
            "clockPolarity": "K_SPI_CPOL_HIGH", // kubos-gcc
            "clockPhase": "K_SPI_CPHA_1EDGE", // kubos-gcc
            "firstBit": "K_SPI_FIRSTBIT_LSB", // kubos-gcc
            "speed": "10000" // kubos-gcc
          },
          "spi1": {},
          "spi2": {}
        },
        "uart": {
          "count": 2, // msp430f5529-gcc
          "uart1": {
            "tx": "P33", // msp430f5529-gcc
            "rx": "P34" // msp430f5529-gcc
          },
          "uart2": {
            "tx": "P44", // msp430f5529-gcc
            "rx": "P45" // msp430f5529-gcc
          },
          "defaults": {
            "baudRate": 9600, // kubos-gcc
            "wordLen": "K_WORD_LEN_8BIT", // kubos-gcc
            "stopBits": "K_STOP_BITS_1", // kubos-gcc
            "parity": "K_PARITY_NONE", // kubos-gcc
            "rxQueueLen": 128, // kubos-gcc
            "txQueueLen": 128 // kubos-gcc
          }
        }
      },
      "gcc": {
        "printf-float": false // kubos-msp430-gcc
      },
      "arch": {
        "msp430": {}
      }
    }
    
Overview
^^^^^^^^

Here is a high-level overview of all of the currently available options:

.. code-block:: json

    {
        "cmsis": {
            "nvic": {
                "ram_vector_address": "hex address",
                "flash_vector_address": "hex address",
                "user_irq_offset": "number",
                "user_irq_number": "number"
            }
        },
        "uvisor": {
            "present": "number"
        },
        "gcc": {
            "printf-float": "boolean"
        },
        "arch": {
            "arm": {},
            "msp430": {}
        },
        "system": {
            "initAfterFlash": "boolean",
            "initAtBoot": "boolean",
            "runLevel": "number",
            "destDir": "file location",
            "password": "string"
        },
        "hardware": {
            "externalClock": "number as string",
            "console": {
                "uart": "UART bus",
                "baudRate": "number as string"
            },
            "pins": {
                "{name}": "pin"
            },
            "test-pins": {
                "spi": {
                    "mosi": "pin",
                    "miso": "pin",
                    "sclk": "pin",
                    "ssel": "pin"
                },
                "i2c": {
                    "sda": "pin",
                    "scl": "pin"
                },
                "serial": {
                    "tx": "pin",
                    "rx": "pin"
                }
            },
            "i2c": {
                "count": "number",
                "defaults": {
                    "bus": "pin",
                    "role": "enum",
                    "clockSpeed": "number",
                    "addressingMode": "enum"
                },
                "i2c{n}": {
                    "scl": {
                        "pin": "pin",
                        "mode": "enum",
                        "pullup": "enum",
                        "speed": "enum"
                    },
                    "sda": {
                        "pin": "pin",
                        "mode": "enum",
                        "pullup": "enum",
                        "speed": "enum"
                    },
                    "alt": "string"
                }
            },
            "uart": {
                "count": "number",
                "defaults": {
                    "baudRate": "number",
                    "wordLen": "enum",
                    "stopBits": "enum",
                    "parity": "enum",
                    "rxQueueLen": "number",
                    "txQueueLen": "number"
                },
                "uart{n}": {
                    "tx": "pin",
                    "rx": "pin"
                }
            },
            "spi": {
                "count": "number",
                "defaults": {
                    "bus": "enum",
                    "role": "enum",
                    "direction": "enum",
                    "dataSize": "enum",
                    "clockPolarity": "enum",
                    "clockPhase": "enum",
                    "firstBit": "enum",
                    "speed": "number as string"
                },
                "spi{n}": {
                    "mosi": "pin",
                    "miso": "pin",
                    "sck": "pin",
                    "cs": "pin",
                    "port": "string",
                    "alt": "string"
                }
            }
        }
    }
    
Descriptions
^^^^^^^^^^^^

Each of the objects in more detail:

.. json:object:: hardware

    Description of board's hardware peripherals
    
    :property console: Debug console
    :proptype console: :json:object:`console`
    :property integer externalClock: Clock rate of external clock
    :property pins: todo
    :proptype pins: :json:object:`pins`
    :property test-pins: todo
    :proptype test-pins: :json:object:`test-pins`
    :property i2c: todo
    :proptype i2c: :json:object:`i2c`
    :property uart: todo
    :proptype uart: :json:object:`uart`
    :property spi: todo
    :proptype spi: :json:object:`spi`
    
.. json:object:: console

    The debug UART console

    :property string uart: UART bus to connect to. Will be in the 
      form ``K_UART{n}``, where `n` matches a defined :json:object:`uart-bus`
    :property string baudRate: Connection speed
    :options baudRate: default 115200
    
.. json:object:: pins

    todo
    
    :property pin {pin-name}: Pin name/value pair
    
.. json:object:: test-pins

    todo
    
    :property spi:
    :proptype spi: :json:object:`test-pins/spi`
    :property i2c:
    :proptype i2c: :json:object:`test-pins/i2c`
    :property serial:
    :proptype serial: :json:object:`test-pins/serial`
    
.. json:object:: test-pins/spi

    SPI connection pins
    
    :property pin mosi: Master-out pin
    :property pin miso: Master-in pin
    :property pin sclk: Slave clock pin
    :property pin ssel: Slave-select pin

.. json:object:: test-pins/i2c

    I2C connection pins

    :property pin sda: Data pin
    :property pin scl: Clock pin

.. json:object:: test-pins/serial

    Serial connection pins
    
    :property pin tx: Transmit pin
    :property pin rx: Receive pin
    
.. json:object:: i2c

    Availability and properties of I2C on the target device
    
    :property integer count: Number of I2C buses available
    :property defaults: Default setting for all I2C buses
    :proptype defaults: :json:object:`i2c-defaults`
    :property i2c{n}: I2C bus definitions
    :proptype i2c{n}: :json:object:`i2c-bus`
    
.. json:object:: i2c-defaults

    Default connection settings for all I2C buses
    
    :property bus: The default I2C bus
    :proptype bus: :cpp:enum:`KI2CNum`
    :property role: Default master/slave role
    :proptype role: :cpp:enum:`I2CRole`
    :proptype role:  
    :property integer clockSpeed: Default bus speed
    :property addressingMode: I2C addressing mode
    :proptype addressingMode: :cpp:enum:`I2CAddressingMode`
    
.. json:object:: i2c-bus

    I2C bus definition
    
    :property scl: Clock line settings
    :proptype scl: :json:object:`scl`
    :property sda: Data line settings
    :proptype sda: :json:object:`sda`
    :property alt: todo
    
.. json:object:: scl

    I2C bus clock line settings
    
    :property pin pin: Clock line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:enum:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:enum:`KGPIOPullup`
    :property enum speed: Clock line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH]

.. json:object:: sda

    I2C bus data line settings
    
    :property pin pin: Data line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:enum:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:enum:`KGPIOPullup`
    :property enum speed: Data line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH] TODO is this defined anywhere that we can reference instead?
    

.. json:object:: uart

    Availability and properties of UART on the target device
    
    :property integer count: Number of UART buses available
    :property defaults: Default setting for all UART buses
    :proptype defaults: :json:object:`uart-defaults`
    :property uart{n}: UART bus definitions
    :proptype uart{n}: :json:object:`uart-bus`
    
.. json:object:: uart-defaults

.. json:object:: spi

    Availability and properties of SPI on the target device
    
    :property integer count: Number of SPI buses available
    :property defaults: Default setting for all SPI buses
    :proptype defaults: :json:object:`spi-defaults`
    :property spi{n}: The `n`th SPI bus
    :proptype spi{n}: :json:object:`spi-bus`
    
.. json:object:: cmsis

    TODO: What is this thing...
    
    :property nvic: todo
    :proptype nvic: :json:object:`nvic`
    
.. json:object:: nvic

    TODO
    
    :property string ram_vector_address: Hex address of TODO
    :property string flash_vector_address: Hex address of TODO
    :property integer user_irq_offset: Offset of
    :options user_irq_offset: todo. max/min?
    :property integer user_irq_number: todo
    

.. json:object:: uvisor

    TODO
    
    :property integer present: TODO
    
.. json:object:: gcc

    TODO
    
    :property boolean printf-float: todo

.. json:object:: arch

    Architecture of the target's processor

    :property object arm: Specifies that the target has an ARM architecture
    :property object msp430: Specifies that the target has an MSP430 architecture

.. json:object:: system
    
    :property boolean initAfterFlash: Specifies whether the 
      application should be started as a background daemon on the target 
      device immediately after being flashed
    :options initAfterFlash: Default false
    :property boolean initAtBoot: Specifies whether the application should 
      be started on the target device during system initialization
    :options initAtBoot: Default true. An init script will be generated with the 
      run level specified by ``runLevel`` 
    :property number runLevel: The priority of the generated init script. 
      Scripts with lower values will be run first
    :options runLevel: Default: 50. Range: 10-99
    :property string destDir: Specifies flashing destination directory for all 
      non-application files
    :options destDir: Default /home/usr/local/bin
    :property string password: Specifies the root password to be used by 
      ``kubos flash`` to successfully connect to the target device
    

module.json
-----------
