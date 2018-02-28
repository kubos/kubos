Kubos Project Configuration
===========================

For Kubos projects using C, the project configuration is derived from yotta's `configuration system <http://docs.yottabuild.org/reference/config.html>`__ 
and `module.json <http://docs.yottabuild.org/reference/module.html>`__ files.

If a project's configuration is changed, the new settings will be incorporated during the next execution of ``kubos build``.

config.json
-----------
    
Overview
^^^^^^^^

Each Kubos target comes with a set of default configuration options. These options describe things
like hardware bus availability and communication settings.
The `config.json` file, which lives in the top level directory of a Kubos project, allows users to 
override any of these options with a custom value.

Under the covers, the `target.json` and `config.json` files are used to generate a `yotta_config.h` file,
which contains ``#define YOTTA_CFG_{option}`` statements for each defined option. These ``YOTTA_CFG_*``
variables can then be referenced within the Kubos project's source code.

The current configuration of a project can be seen using the ``kubos config`` command. 
Each configuration option in the output will have a comment showing the origin of the value.
Anything marked with "application's config.json" will have been taken from the project's `config.json` file.
All other comments will have "\*-gcc", which indicates that that option is a default value taken from
the corresponding `target.json` file.

For example:

::

    $ kubos config
    
    {
      "hardware": {
        "i2c": {
          "count": 1, // kubos-linux-isis-gcc
          "defaults": {
            "bus": "K_I2C1", // kubos-linux-isis-gcc
            "role": "K_MASTER", // kubos-gcc
            "clockSpeed": 100000, // kubos-gcc
            "addressingMode": "K_ADDRESSINGMODE_7BIT" // kubos-gcc
          },
          "i2c1": {
            "device": "/dev/i2c-0" // kubos-linux-isis-gcc
          }
        },
        "uart": {
          "count": 0, // kubos-gcc
          "defaults": {
            "baudRate": 9600, // kubos-gcc
            "wordLen": "K_WORD_LEN_8BIT", // kubos-gcc
            "stopBits": "K_STOP_BITS_1", // kubos-gcc
            "parity": "K_PARITY_NONE", // kubos-gcc
            "rxQueueLen": 128, // kubos-gcc
            "txQueueLen": 128 // kubos-gcc
          }
        },
        "spi": {
          "count": 0, // kubos-gcc
          "defaults": {
            "bus": "K_SPI1", // kubos-gcc
            "role": "K_SPI_MASTER", // kubos-gcc
            "direction": "K_SPI_DIRECTION_2LINES", // kubos-gcc
            "dataSize": "K_SPI_DATASIZE_8BIT", // kubos-gcc
            "clockPolarity": "K_SPI_CPOL_HIGH", // kubos-gcc
            "clockPhase": "K_SPI_CPHA_1EDGE", // kubos-gcc
            "firstBit": "K_SPI_FIRSTBIT_LSB", // kubos-gcc
            "speed": "10000" // kubos-gcc
          }
        }
      },
      "system": {
        "initAfterFlash": false, // kubos-linux-gcc
        "initAtBoot": false, // kubos-linux-gcc
        "runLevel": 50, // kubos-linux-gcc
        "destDir": "/home/system/usr/local/bin", // kubos-linux-gcc
        "password": "Kubos123" // kubos-linux-gcc
      }
    }

    
Custom Settings
^^^^^^^^^^^^^^^

Users can add new settings to a `config.json` file which can then be used within their project.
These settings will be generated as ``#define YOTTA_CFG_{user_option} {value}`` statements
during project compilation time.

For example::

    {
      "CSP": {
        "my_address": "1",
        "target_address": "2",
        "port": "10",
        "uart_bus": "K_UART6",
        "uart_baudrate": "115200",
        "usart": {}
      }
    }

Will generate the following statements:


.. code-block:: c

    #define YOTTA_CFG_CSP_MY_ADDRESS 1
    #define YOTTA_CFG_CSP_TARGET_ADDRESS 2
    #define YOTTA_CFG_CSP_PORT 10
    #define YOTTA_CFG_CSP_UART_BUS K_UART6
    #define YOTTA_CFG_CSP_UART_BAUDRATE 115200
    #define YOTTA_CFG_CSP_USART
    

    
Non-Default Settings
^^^^^^^^^^^^^^^^^^^^

These are settings which are not included by default as part of any target device, so must
be explicitly provided in a `config.json` file in order to be made available to the project.
        
SDIO
####

General SDIO support is turned on via the ``hardware.sdio`` object. This support is not 
automatically included with any target device.

.. json:object:: hardware.sdio

    SDIO support
    
    `There are no configuration properties for this object. It simply enables the use
    of the HAL SDIO library`
    
    **Example**:: 

        {
            "hardware": {
                "sdio": {}
            }
        }
    
                
    
User-Configurable Included Settings
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

These are settings which may be changed by the user without compromising the target device,
but which will automatically be included in the project without a `config.json` file present.

System
######

.. json:object:: system

    Kubos Linux file system properties related to Kubos applications
    
    :property boolean initAfterFlash: `(Default: false)` Specifies whether the 
      application should be started as a background daemon on the target 
      device immediately after being flashed
    :property boolean initAtBoot: `(Default: true)` Specifies whether the application should 
      be started on the target device during system initialization. An init script will be 
      generated with the run level specified by ``runLevel`` 
    :property number runLevel: `(Default: 50. Range: 10-99)` The priority of the generated init script. 
      Scripts with lower values will be run first
    :property string destDir: `(Default: "/home/usr/local/bin")` Specifies flashing destination directory for all 
      non-application files
    :property string password: `(Default: "Kubos123") Specifies the root password to be used by 
      ``kubos flash`` to successfully connect to the target device
    
    **Example**::
    
        {
            "system": {
              "initAfterFlash": true,
              "initAtBoot": true,
              "runLevel": 40,
              "destDir": "/home/myUser/storage",
              "password": "password"
            }
        }

Hardware
########

.. json:object:: hardware

    Description of target board's hardware peripherals
    
    :property console: Debug console
    :proptype console: :json:object:`console <hardware.console>`
    :property integer externalClock: Clock rate of external clock
    :property pins: Custom name -> pin mapping
    :proptype pins: :json:object:`pins <hardware.pins>`
    :property i2c: Availability and properties of I2C
    :proptype i2c: :json:object:`i2c <hardware.i2c>`
    :property uart: Availability and properties of UART
    :proptype uart: :json:object:`uart <hardware.uart>`
    :property spi: Availability and properites of SPI
    :proptype spi: :json:object:`spi <hardware.spi>`
    :proptype sdio: Availability of SDIO
    :proptype sdio: :json:object:`sdio <hardware.sdio>`
    
.. json:object:: hardware.console

    The debug UART console

    :property uart: UART bus to connect to
    :proptype uart: :cpp:type:`KUARTNum`
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
    
.. json:object:: hardware.i2c

    Availability and properties of I2C on the target device
    
    :property integer count: Number of I2C buses available
    :property defaults: Default I2C connection settings
    :proptype defaults: :json:object:`defaults <hardware.i2c.defaults>`
    :property i2c{n}: I2C bus definitions
    :proptype i2c{n}: :json:object:`bus <hardware.i2c.i2c{n}>`
    
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
    :proptype bus: :cpp:type:`KI2CNum`
    :property role: Default communication role
    :proptype role: :cpp:type:`I2CRole`
    :property integer clockSpeed: Default bus speed
    :property addressingMode: I2C addressing mode
    :proptype addressingMode: :cpp:type:`I2CAddressingMode`
    
.. json:object:: hardware.i2c.i2c{n}

    I2C bus definition
    
    :property scl: Clock line settings
    :proptype scl: :json:object:`scl <hardware.i2c.i2c{n}.scl>`
    :property sda: Data line settings
    :proptype sda: :json:object:`sda <hardware.i2c.i2c{n}.sda>`
    :property string alt: `(STM32F4* only)` GPIO alternate function mapping
    :options alt: GPIO_AFx_I2Cy
    
.. json:object:: hardware.i2c.i2c{n}.scl

    I2C bus clock line settings
    
    :property pin pin: Clock line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:type:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:type:`KGPIOPullup`
    :property type speed: Clock line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH]

.. json:object:: hardware.i2c.i2c{n}.sda

    I2C bus data line settings
    
    :property pin pin: Data line pin
    :property mode: Pin GPIO mode
    :proptype mode: :cpp:type:`KGPIOMode`
    :property pullup: Pin pullup/pulldown setting
    :proptype pullup: :cpp:type:`KGPIOPullup`
    :property string speed: Data line speed
    :options speed: GPIO_SPEED_[LOW, MEDIUM, FAST, HIGH]
    

.. json:object:: hardware.uart

    Availability and properties of UART on the target device
    
    :property integer count: Number of UART buses available
    :property defaults: Default UART connection settings
    :proptype defaults: :json:object:`defaults <hardware.uart.defaults>`
    :property uart{n}: UART bus definitions
    :proptype uart{n}: :json:object:`bus <hardware.uart.uart{n}>`
    
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
    :proptype wordLen: :cpp:type:`KWordLen`
    :property stopBits: Default number of stop bits
    :proptype stopBits: :cpp:type:`KStopBits`
    :property parity: Default parity setting
    :proptype parity: :cpp:type:`KParity`
    :property integer rxQueueLen: Default size of RX queue
    :property integer txQueueLen: Default size of TX queue
    
.. json:object:: hardware.uart.uart{n}

    UART bus definition
    
    :property pin tx: Bus transmit pin
    :property pin rx: Bus receive pin
    
.. json:object:: hardware.spi

    Availability and properties of SPI on the target device
    
    :property integer count: Number of SPI buses available
    :property defaults: Default SPI connection settings
    :proptype defaults: :json:object:`defaults <hardware.spi.defaults>`
    :property spi{n}: SPI bus definitions
    :proptype spi{n}: :json:object:`bus <hardware.spi.spi{n}>`
    
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
    :proptype bus: :cpp:type:`KSPINum`
    :property role: Default communication role
    :proptype role: :cpp:type:`SPIRole`
    :property direction: Default SPI communication direction/s
    :proptype direction: :cpp:type:`SPIDirection`
    :property dataSize: Default data size
    :proptype dataSize: :cpp:type:`SPIDataSize`
    :property clockPolarity: Default clock polarity
    :proptype clockPolarity: :cpp:type:`SPIClockPolarity`
    :property clockPhase: Defaut clock phase
    :proptype clockPhase: :cpp:type:`SPIClockPhase`
    :property firstBit: Default endianness
    :proptype firstBit: :cpp:type:`SPIFirstBit`
    :property integer speed: Default bus speed
    
.. json:object:: hardware.spi.spi{n}

    SPI bus definition
    
    :property pin mosi: Master-out pin
    :property pin miso: Master-in pin
    :property pin sck: Clock pin
    :property pin cs: Chip-select pin
    :property pin port: GPIO port that the SPI pins belong to
    :property string alt: `(STM32F4* only)` GPIO alternate function mapping
    :options alt: GPIO_AFx_I2Cy

CSP
###

.. json:object:: csp

    Kubos CSP (CubeSat Protocol) configuration
    
    :property boolean debug: Turn on CSP debug messages

    **Example**::
    
        {
            "csp": {
                "debug": true
            }
        }

module.json
-----------

The Kubos project's `module.json` file is originally based on `yotta's module.json file <http://docs.yottabuild.org/reference/module.html>`__

Default Configurations
^^^^^^^^^^^^^^^^^^^^^^

When you run ``kubos init -l``, a `module.json` file is created for you with some default values::

    {
        "bin": "./source",
        "license": "Apache-2.0",
        "name": "{your-project-name}",
        "repository":{
            "url": "git://<repository_url>",
            "type": "git"
        },
        "version": "0.1.0",
        "dependencies":{
            "csp": "kubos/libcsp#~1.5.0"
        },
        "homepage": "https://<homepage>",
        "description": "Example app running on Kubos Linux."
    }

Relevant Configuration Options
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

These are the configuration options which are most likely to be changed for a project.
(For all other options, refer to `yotta's documentation <http://docs.yottabuild.org/reference/module.html>`__.)

.. json:object:: name

    The module name, which is also used as the file name of the compiled application binary.
    
    By default, this is the project name, however, it can be changed to anything.
    
    Naming rules:
    
    - Must start with a letter
    - No uppercase letters
    - Numbers are allowed
    - Hyphens are allowed
    
.. json:object:: bin
    
    Relative path to the project's source code.
    
.. json:object:: dependencies

    Project library dependencies.

    To keep Kubos project binaries small, ``kubos build`` will only include libraries which have been specified in this object.
    As a result, if you want to use a Kubos library, it **must** be specified here, or must be included with another library
    you specify.
    
    :property string {component}: Project dependency location and/or version
    
    Available dependency name/value pairs (hierarchy denotes included dependencies. Italics denotes yotta targetDependencies):
                
    - "ccan-json": "kubos/ccan-json"
    - "cmocka": "kubos/cmocka"             
    - "csp": "kubos/libcsp"
    
        - `"kubos-hal": "kubos/kubos-hal"`
        - `"tinycbor": "kubos/tinycbor"`
        
    - "kubos-hal": "kubos/kubos-hal"
    
        - "csp": "kubos/libcsp"
        - `"kubos-hal-linux": "kubos/kubos-hal-linux"`
        
            - "kubos-hal" : "kubos/kubos-hal"
        
    - "kubos-hal-iobc": "kubos/kubos-hal-iobc"
    - "tinycbor": "kubos/tinycbor"
