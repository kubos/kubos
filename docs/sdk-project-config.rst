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

    Description of target board's hardware peripherals
    
    :property console: Debug console
    :proptype console: :json:object:`console <hardware.console>`
    :property integer externalClock: Clock rate of external clock
    :property pins: Custom name -> pin mapping
    :proptype pins: :json:object:`pins <hardware.pins>`
    :property test-pins: todo
    :proptype test-pins: :json:object:`test-pins <hardware.test-pins>`
    :property i2c: Availability and properties of I2C
    :proptype i2c: :json:object:`i2c <hardware.i2c>`
    :property uart: Availability and properties of UART
    :proptype uart: :json:object:`uart <hardware.uart>`
    :property spi: Availability and properites of SPI
    :proptype spi: :json:object:`spi <hardware.spi>`
    
.. json:object:: hardware.console

    The debug UART console

    :property uart: UART bus to connect to
    :proptype uart: :cpp:enum:`KUARTNum`
    :property string baudRate: `(Default: "115200")` Connection speed
    
    **Example**::
    
        {
            "hardware": {
                "console": {
                    "uart": "K_UART1",
                    "baudRate": "9600"
                }
            }
        }
    
.. json:object:: hardware.pins

    Custom name -> pin mapping. Allows more readable pin names to be used in Kubos projects.
    
    :property pin {pin-name}: Pin name/value pair
    
    **Example**::
     
        {
            "hardware": {
                "pins": {
                    "LED1": "PA1",
                    "LED2": "PA2",
                    "USER_BUTTON": "PA3"
                }
            }
        }
    
.. json:object:: hardware.test-pins

    todo
    
    :property spi:
    :proptype spi: :json:object:`spi <hardware.test-pins.spi>`
    :property i2c:
    :proptype i2c: :json:object:`i2c <hardware.test-pins.i2c>`
    :property serial:
    :proptype serial: :json:object:`serial <hardware.test-pins.serial>`
    
.. json:object:: hardware.test-pins.spi

    SPI connection test pins
    
    :property pin mosi: Master-out pin
    :property pin miso: Master-in pin
    :property pin sclk: Slave clock pin
    :property pin ssel: Slave-select pin

.. json:object:: hardware.test-pins.i2c

    I2C connection test pins

    :property pin sda: Data pin
    :property pin scl: Clock pin

.. json:object:: hardware.test-pins.serial

    Serial connection test pins
    
    :property pin tx: Transmit pin
    :property pin rx: Receive pin
    
.. json:object:: hardware.i2c

    Availability and properties of I2C on the target device
    
    :property integer count: Number of I2C buses available
    :property defaults: Default I2C connection settings
    :proptype defaults: :json:object:`defaults <hardware.i2c.defaults>`
    :property i2c{n}: I2C bus definitions
    :proptype i2c{n}: :json:object:`bus <hardware.i2c.bus>`
    
    **Example**::
    
        {
            "hardware": {
              "i2c": {
                "count": 2,
                "defaults": {
                  "bus": "K_I2C1",
                  "role": "K_MASTER",
                  "clockSpeed": 100000,
                  "addressingMode": "K_ADDRESSINGMODE_7BIT"
                },
                "i2c1": {
                  "scl": {
                    "pin": "PB6",
                    "mode": "GPIO_MODE_AF_PP",
                    "pullup": "GPIO_NOPULL",
                    "speed": "GPIO_SPEED_MEDIUM"
                  },
                  "sda": {
                    "pin": "PB7",
                    "mode": "GPIO_MODE_AF_OD",
                    "pullup": "GPIO_PULLUP",
                    "speed": "GPIO_SPEED_MEDIUM"
                  },
                  "alt": "GPIO_AF4_I2C1"
                },
                "i2c2": {
                  "scl": {
                    "pin": "PB10",
                    "mode": "GPIO_MODE_AF_PP",
                    "pullup": "GPIO_NOPULL",
                    "speed": "GPIO_SPEED_MEDIUM"
                  },
                  "sda": {
                    "pin": "PB11",
                    "mode": "GPIO_MODE_AF_OD",
                    "pullup": "GPIO_PULLUP",
                    "speed": "GPIO_SPEED_MEDIUM"
                  },
                  "alt": "GPIO_AF4_I2C2"
                }
              }
            }
        }
    
.. json:object:: hardware.i2c.defaults

    Default I2C connection settings
    
    :property bus: The default I2C bus
    :proptype bus: :cpp:enum:`KI2CNum`
    :property role: Default communication role
    :proptype role: :cpp:enum:`I2CRole`
    :property integer clockSpeed: Default bus speed
    :property addressingMode: I2C addressing mode
    :proptype addressingMode: :cpp:enum:`I2CAddressingMode`
    
.. json:object:: hardware.i2c.bus

    I2C bus definition
    
    :property scl: Clock line settings
    :proptype scl: :json:object:`scl <hardware.i2c.bus.scl>`
    :property sda: Data line settings
    :proptype sda: :json:object:`sda <hardware.i2c.bus.sda>`
    :property string alt: `(STM32F4* only)` GPIO alternate function mapping
    :options alt: GPIO_AFx_I2Cy
    
.. json:object:: hardware.i2c.bus.scl

    I2C bus clock line settings
    
    :property pin pin: Clock line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:enum:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:enum:`KGPIOPullup`
    :property enum speed: Clock line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH]

.. json:object:: hardware.i2c.bus.sda

    I2C bus data line settings
    
    :property pin pin: Data line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:enum:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:enum:`KGPIOPullup`
    :property string speed: Data line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH]
    

.. json:object:: hardware.uart

    Availability and properties of UART on the target device
    
    :property integer count: Number of UART buses available
    :property defaults: Default UART connection settings
    :proptype defaults: :json:object:`defaults <hardware.uart.defaults>`
    :property uart{n}: UART bus definitions
    :proptype uart{n}: :json:object:`bus <hardware.uart.bus>`
    
    **Example**::
    
        {
            "hardware": {
              "uart": {
                "count": 2,
                "defaults": {
                  "baudRate": 9600,
                  "wordLen": "K_WORD_LEN_8BIT",
                  "stopBits": "K_STOP_BITS_1",
                  "parity": "K_PARITY_NONE",
                  "rxQueueLen": 128,
                  "txQueueLen": 128
                },
                "uart1": {
                    "tx": "P33",
                    "rx": "P34"
                },
                "uart2": {
                    "tx": "P44",
                    "rx": "P45"
                }
              }
            }
        }
    
.. json:object:: hardware.uart.defaults

    Default UART connection settings
    
    :property integer baudRate: Default bus speed
    :property wordLen: Default word length
    :proptype wordLen: :cpp:enum:`KWordLen`
    :property stopBits: Default number of stop bits
    :proptype stopBits: :cpp:enum:`KStopBits`
    :property parity: Default parity setting
    :proptype parity: :cpp:enum:`KParity`
    :property integer rxQueueLen: Default size of RX queue
    :property integer txQueueLen: Default size of TX queue
    
.. json:object:: hardware.uart.bus

    UART bus definition
    
    :property pin tx: Bus transmit pin
    :property pin rx: Bus receive pin
    
.. json:object:: hardware.spi

    Availability and properties of SPI on the target device
    
    :property integer count: Number of SPI buses available
    :property defaults: Default SPI connection settings
    :proptype defaults: :json:object:`defaults <hardware.spi.defaults>`
    :property spi{n}: SPI bus definitions
    :proptype spi{n}: :json:object:`bus <hardware.spi.bus>`
    
    **Example**::
    
        {
            "hardware": {
              "spi": {
                "count": 3,
                "defaults": {
                  "bus": "K_SPI1",
                  "role": "K_SPI_MASTER",
                  "direction": "K_SPI_DIRECTION_2LINES",
                  "dataSize": "K_SPI_DATASIZE_8BIT",
                  "clockPolarity": "K_SPI_CPOL_HIGH",
                  "clockPhase": "K_SPI_CPHA_1EDGE",
                  "firstBit": "K_SPI_FIRSTBIT_LSB",
                  "speed": "10000"
                },
                "spi1": {
                  "mosi": "PA7",
                  "miso": "PA6",
                  "sck": "PA5",
                  "cs": "PA4",
                  "port": "GPIOA",
                  "alt": "GPIO_AF5_SPI1"
                },
                "spi2": {
                  "mosi": "PB15",
                  "miso": "PB14",
                  "sck": "PB13",
                  "cs": "PB12",
                  "port": "GPIOB",
                  "alt": "GPIO_AF5_SPI2"
                },
                "spi3": {
                  "mosi": "PC12",
                  "miso": "PC11",
                  "sck": "PC10",
                  "cs": "PC8",
                  "port": "GPIOC",
                  "alt": "GPIO_AF6_SPI3"
                }
              }
            }
        }
    
.. json:object:: hardware.spi.defaults

    Default SPI connection settings
    
    :property bus: Default SPI bus
    :proptype bus: :cpp:enum:`KSPINum`
    :property role: Default communication role
    :proptype role: :cpp:enum:`SPIRole`
    :property direction: Default SPI communication direction/s
    :proptype direction: :cpp:enum:`SPIDirection`
    :property dataSize: Default data size
    :proptype dataSize: :cpp:enum:`SPIDataSize`
    :property clockPolarity: Default clock polarity
    :proptype clockPolarity: :cpp:enum:`SPIClockPolarity`
    :property clockPhase: Defaut clock phase
    :proptype clockPhase: :cpp:enum:`SPIClockPhase`
    :property firstBit: Default endianness
    :proptype firstBit: :cpp:enum:`SPIFirstBit`
    :property integer speed: Default bus speed
    
.. json:object:: hardware.spi.bus

    SPI bus definition
    
    :property pin mosi: Master-out pin
    :property pin miso: Master-in pin
    :property pin sck: Clock pin
    :property pin cs: Chip-select pin
    :property pin port: GPIO port that the SPI pins belong to
    :property string alt: `(STM32F4* only)` GPIO alternate function mapping
    :options alt: GPIO_AFx_I2Cy
    
.. json:object:: cmsis

    Cortex Microcontroller Software Interface Standard
    
    Settings specific to targets with Cortex processors
    
    :property nvic: "Nester Vector Interrupt Controller"
    :proptype nvic: :json:object:`nvic <cmsis.nvic>`
    
.. json:object:: cmsis.nvic

    Nested Vector Interupt Controller
    
    :property string ram_vector_address: Location of vectors in RAM
    :property string flash_vector_address: Initial vector position in flash
    :property integer user_irq_offset: `(Default: 16)` Number of ARM core vectors (HardFault handler, SysTick, etc)
    :property integer user_irq_number: `(Default: 82)` Number of manufacturer vectors
    

.. json:object:: uvisor

    `uVisor <https://github.com/ARMmbed/uvisor>`__ RTOS security settings
    
    :property integer present: `(Default: 0. Values: 0, 1)` Specifies whether uVisor is present on the target device
    
.. json:object:: gcc

    Project compiler options
    
    :property boolean printf-float: `(Default: False)` Enables floating point support in ``printf`` commands

.. json:object:: arch

    Architecture of the target's processor

    :property object arm: Specifies that the target has an ARM architecture
    :property object msp430: Specifies that the target has an MSP430 architecture

.. json:object:: system
    
    :property boolean initAfterFlash: `(Default: false)` Specifies whether the 
      application should be started as a background daemon on the target 
      device immediately after being flashed
    :property boolean initAtBoot: `(Default: true)` Specifies whether the application should 
      be started on the target device during system initialization.vAn init script will be 
      generated with the run level specified by ``runLevel`` 
    :property number runLevel: `(Default: 50. Range: 10-99)` The priority of the generated init script. 
      Scripts with lower values will be run first
    :property string destDir: `(Default: "/home/usr/local/bin")` Specifies flashing destination directory for all 
      non-application files
    :property string password: `(Default: "Kubos123") Specifies the root password to be used by 
      ``kubos flash`` to successfully connect to the target device
    

module.json
-----------
