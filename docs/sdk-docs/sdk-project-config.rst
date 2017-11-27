Kubos Project Configuration
===========================

Kubos project configuration is derived from Yotta's `configuration system <http://docs.yottabuild.org/reference/config.html>`__ 
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

File System
###########

If present, the ``fs`` file system structure enables support for accessing storage on a peripheral device.

**Note:** `This structure was created for KubOS RT. KubOS Linux has native support for various file systems.`

.. json:object:: fs

    File system support
    
    :property fatfs: FatFS settings
    :proptype fatfs: :json:object:`fatfs`
        
.. json:object:: fs.fatfs

    `FatFS <http://elm-chan.org/fsw/ff/00index_e.html>`__ support
       
    :property driver: Device connection settings
    :proptype driver: :json:object:`driver`
    
.. json:object:: fs.fatfs.driver

    Driver settings for the device the FatFS file system is on.
    
    **Note:** `Only one driver property may be specified`
    
    :property sdio: An SDIO device is available
    :proptype sdio: :json:object:`sdio_dev <fs.fatfs.driver.sdio>`
    :property spi: A SPI device is available
    :proptype spi: :json:object:`spi_dev <fs.fatfs.driver.spi>`
    
.. json:object:: fs.fatfs.driver.sdio

    SDIO device settings
    
    **WARNING:** :json:object:`SDIO HAL support <hardware.sdio>` **must be turned on for this feature to work.**
    
    SDIO is currently supported by:

    - STM32F407 (daughter board)
    - PyBoard
    
    `There are no configuration properties for SDIO. It is assumed that only
    one port is available and will have predetermined settings` 
    
    **Example**:: 

        {
            "fs": {
                "fatfs": {
                    "driver": {
                        "sdio": {}
                    }
                }
            }
        }
        
.. json:object:: fs.fatfs.driver.spi

    SPI device settings
    
    **Note:** `While FatFS over SPI will work for any target with a SPI bus, we recommend
    using FatFS over SDIO if it is available on your target.`
    
    :property dev: SPI bus the device is connected to
    :proptype dev: :cpp:type:`KSPINum`
    :property pin cs: Chip select pin assigned to the device
    
    **Example**:: 

        {
            "fs": {
                "fatfs": {
                    "driver": {
                        "spi": {
                            "dev": "K_SPI1",
                            "cs": "P37" 
                        }
                    }
                }
            }
        }
        
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
    
                
Built-in Peripheral Support
###########################

Kubos Core supports a variety of end-point peripherals. In order to turn on support for these
devices within a Kubos project, they should be added to the ``sensors`` structure of the `config.json` 
file.

.. json:object:: sensors

    Kubos Core sensor APIs
    
    By default, including the ``sensors`` object turns on the following APIs:
    
    - :doc:`Altimeter <../apis/kubos-core/sensors/altimeter>`
    - :doc:`IMU <../apis/kubos-core/sensors/imu>`
    - :doc:`Temperature <../apis/kubos-core/sensors/temperature>`
    
    Without including a corresponding sensor device (ex. HTU21D), these APIs serve only as code stubs.
    
    :property htu21d: HTU21D humidity sensor support
    :proptype htu21d: :json:object:`htu21d <sensors.htu21d>`
    :property bno055: BNO055 absolute orientation sensor support
    :proptype bno055: :json:object:`bno055 <sensors.bno055>`
    :property bme280: BME280 humidity and pressure sensor support
    :proptype bme280: :json:object:`bme280 <sensors.bme280>`
    :property gps: GPS (NMEA) support
    :proptype gps: :json:object:`gps <sensors.gps>`
        
.. json:object:: sensors.htu21d

    `HTU21D humidity sensor <https://cdn-shop.adafruit.com/datasheets/1899_HTU21D.pdf>`__ configuration
    
    :property i2c_bus: The I2C bus connected to the sensor
    :proptype i2c_bus: :cpp:type:`KI2CNum`
    
    **Example**::
    
        {
            "sensors": {
                "htu21d": { 
                    "i2c_bus": "K_I2C1" 
                }                
            }
        }
        
        
.. json:object:: sensors.bno055

    `BNO055 absolute orientation sensor <https://cdn-shop.adafruit.com/datasheets/BST_BNO055_DS000_12.pdf>`__ configuration
    
    **Note:** *The sensor supports interfacing with both I2C and UART, but only I2C support has been implemented in Kubos Core*
    
    :property i2c_bus: The I2C bus connected to the sensor
    :proptype i2c_bus: :cpp:type:`KI2CNum`
    
    **Example**::
    
        {
            "sensors": {
                "bno055": { 
                    "i2c_bus": "K_I2C1" 
                } 
            }
        }
    
.. json:object:: sensors.bme280

    `BME280 humidity and pressure sensor <https://cdn-shop.adafruit.com/datasheets/BST-BME280_DS001-10.pdf>`__ configuration
    
    **Note:** *The sensor supports interfacing with both SPI and I2C, but only SPI support has been implemented in Kubos Core*
    
    :property spi_bus: The SPI bus connected to the sensor
    :proptype spi_bus: :cpp:type:`KSPINum`
    :property pin CS: The chip select pin connected to the sensor
    
    **Example**::
    
        {
            "sensors": {     
                "bme280": {
                    "spi bus": "K_SPI1",
                    "CS": "PA4"
                } 
            }
        }
    
.. json:object:: sensors.gps

    `NMEA-formatted GPS data <http://www.gpsinformation.org/dale/nmea.htm>`__ support
    
    **Note:** `There are no configuration properties for GPS within the config.json file. All configuration will be done
    within the Kubos application's code`
    
    **Example**::
    
        {
            "sensors": {
                "gps": {}          
            }
        }
    
    
User-Configurable Included Settings
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

These are settings which may be changed by the user without compromising the target device,
but which will automatically be included in the project without a `config.json` file present.

System
######

.. json:object:: system

    KubOS Linux file system properties related to Kubos applications
    
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

Command and Control
###################

.. json:object:: cnc

    :doc:`Kubos Command and Control <../middleware/command-and-control>` configuration
    
    **Note:** `Kubos C2 is currently only supported by KubOS Linux`
    
    :property path daemon_log_path: Absolute path for daemon log file
    :property path registry_dir: Absolute path to C2 executables
    :property client: C2 client pipe configuration
    :proptype client: :json:object:`client <cnc.client>`
    :property daemon: C2 daemon pipe configuration
    :proptype daemon: :json:object:`daemon <cnc.daemon>`

    **Example**::
    
        {
            "cnc": {
                "daemon_log_path": "\"/home/var/log.daemon.log\"",
                "registry_dir": "\"/usr/local/kubos\""
            }
        }

.. json:object:: cnc.client

    Kubos Command and Control client configuration
    
    **Note:** `In the future, multiple clients will be able to connect to the single
    C2 daemon. Currently only the command line client is supported`
    
    :property path tx_pipe: Client transmit pipe absolute path
    :property path rx_pipe: Client receive pipe aboslute path
    
    **Example**::
    
        {
           "cnc": {
               "client": {
                   "tx_pipe": "\"/usr/local/kubos/client-to-daemon\"",
                   "rx_pipe": "\"/usr/local/kubos/daemon-to-client\""
               }
           }
        }
        
.. json:object:: cnc.daemon

    Kubos Command and Control daemon configuration
    
    :property path tx_pipe: Daemon transmit pipe absolute path
    :property path rx_pipe: Daemon receive pipe aboslute path
    
    **Example**::
    
        {
           "cnc": {
               "daemon": {
                   "tx_pipe": "\"/usr/local/kubos/daemon-to-client\"",
                   "rx_pipe": "\"/usr/local/kubos/client-to-daemon\""
               }
           }
        }

Telemetry
#########

.. json:object:: telemetry

    Kubos Telemetry configuration
    
    :property csp: CSP connection configuration
    :proptype csp: :json:object:`csp <telemetry.csp>`
    :property aggregator: Aggregator configuration
    :proptype aggregator: :json:object:`aggregator <telemetry.aggregator>`
    :property subscribers: Subscriber configuration
    :proptype subscribers: :json:object:`subscribers <telemetry.subscribers>`
    :property integer message_queue_size: `(Default: 10)` Max number of messages allowed in telemetry queue
    :property integer internal_port: `(Default: 20)` Port number used for the telemetry server's internal connections
    :property integer external_port: `(Default: 10)` Port number used for telemetry's external socket connections
    :property rx_thread: Receive thread configuration
    :proptype rx_thread: :json:object:`rx_thread <telemetry.rx_thread>`
    :property integer buffer_size: `(Default: 256) KubOS Linux only.` Max size of a message which can be sent/processed by the telemetry system
    :property storage: Telemetry storage configuration
    :proptype storage: :json:object:`storage <telemetry.storage>`

    **Example**::
    
        {
            "telemetry": {
                "message_queue_size": 10,
                "internal_port": 20,
                "external_port": 10,
                "buffer_size": 256,
            }
        }
        
.. json:object:: telemetry.csp

    Kubos Telemetry server's CSP configuration
    
    :property integer address: `KubOS RT only.` CSP address used by telemetry server 
    :property integer client_address: `KubOS RT only.` CSP address for a telemetry client thread/process
    
    **Example**::
    
        {
            "telemetry": {
                "csp": {
                    "address": 1,
                    "client_address": 2
                }
            }
        }
        
.. json:object:: telemetry.aggregator

    Kubos Telemetry aggregator configuration
    
    :property integer interval: `(Default: 300)` Time interval (in ms) between calls to the user-defined telemetry aggregator 
    
    **Example**::
    
        {
            "telemetry": {
                "aggregator": {
                    "interval": 300
                }
            }
        }
    
.. json:object:: telemetry.subscribers

    Kubos Telemetry subscribers configuration
    
    :property integer max_num: `(Default: 10)` Maximum number of subscribers allowed by the telemetry server
    :property integer read_attempts: `(Default: 10)` Number of attempts allowed for a subscriber to read a message from the telemetry server
    
    **Example**::
    
        {
            "telemetry": {
                "subscribers": {
                    "max_num": 10,
                    "read_attempts": 10
                }
            }
        }
    
.. json:object:: telemetry.rx_thread

    Kubos Telemetry server receive thread configuration
    
    :property integer stack_size: `(Default: 1000)` Stack size of the thread
    :property integer priority: `(Default: 2)` Priority level of the thread
    
    **Example**::
    
        {
            "telemetry": {
                "rx_thread": {
                    "stack_size": 1000,
                    "priority": 2
                }
            }
        }
    
.. json:object:: telemetry.storage

    Kubos Telemetry storage configuration
    
    :property integer file_name_buffer_size: `(Default: 128)` Maximum file name length of telemetry storage files
    :property data: Telemetry data storage configuration
    :proptype data: :json:object:`data <telemetry.storage.data>`
    :property string subscriptions: `(Default: "0x0")` Hex flag value indicating topics which telemetry storage should subscribe to and capture in files
    :property integer stack_depth: `(Default: 1000)` Telemetry storage receive task stack depth
    :property integer task_priority: `(Default: 0)` Telemetry storage receive task priority
    
    **Example**::
    
        {
             "telemetry": {
                 "storage": {                
                    "file_name_buffer_size": 128,
                    "data": {
                        "buffer_size": 64,
                        "part_size": 51200,
                        "max_parts": 10,
                        "output_format": "FORMAT_TYPE_CSV"
                    },
                    "subscriptions": "0x0",
                    "subscribe_retry_interval": 50,
                    "stack_depth": 1000,
                    "task_priority": 0
                }
            }
        }
    
.. json:object:: telemetry.storage.data

    Kubos Telemetry data storage configuration

    :property integer buffer_size: `(Default: 64)` Maximum size/length of the storage buffer
    :property integer part_size: `(Default: 51200)` Maximum file size before file rotation is triggered
    :property integer max_parts: `(Default: 10)` Maximum number of files before file rotation in triggered
    :property output_format: `(Default: "FORMAT_TYPE_CSV")` Output format of telemetry storage files
    :proptype output_format: :cpp:type:`output_data_format`

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
        
IPC
###

.. json:object:: ipc

    Kubos IPC (Inter-Process Communication) configuration
    
    :property integer read_timeout: `(Default: 50)` Timeout value for reading
    :property integer send_timeout: `(Default: 1000)` Timeout value for sending
    :property integer socket_port: `(Default:8888)` Port for IPC sockets to listen/connect on

    **Example**::
    
        {
            "ipc": {
                "read_timeout": 50,
                "send_timeout": 1000,
                "socket_port": 8888
            }
        }

Target-Required Settings
^^^^^^^^^^^^^^^^^^^^^^^^

These are configuration options that are required by a specific target which **should not be changed** by the user.
They are documented here only for reference.
    
    
Architecture
############

.. json:object:: arch

    Architecture of the target's processor

    :property object arm: Specifies that the target has an ARM architecture
    :property object msp430: Specifies that the target has an MSP430 architecture
    
    **Example**::
    
        {
            "arch": {
              "msp430": {}
            }
        }
    
CMSIS
#####
    
.. json:object:: cmsis

    Cortex Microcontroller Software Interface Standard
    
    *Settings specific to targets with Cortex processors*
    
    :property nvic: "Nester Vector Interrupt Controller"
    :proptype nvic: :json:object:`nvic <cmsis.nvic>`
    
    **Example**::
    
        {
            "cmsis": {
              "nvic": {
                "ram_vector_address": "0x20000000",
                "flash_vector_address": "0x08000000",
                "user_irq_offset": 16,
                "user_irq_number": 82
              }
            }
        }
    
    
.. json:object:: cmsis.nvic

    Nested Vector Interupt Controller
    
    :property string ram_vector_address: Location of vectors in RAM
    :property string flash_vector_address: Initial vector position in flash
    :property integer user_irq_offset: `(Default: 16)` Number of ARM core vectors (HardFault handler, SysTick, etc)
    :property integer user_irq_number: `(Default: 82)` Number of manufacturer vectors
    :property boolean has_vtor: `(Default: false)` Specifies whether a Vector Table Offset Register exists on the target
    :property boolean has_custom_vtor: `(Default: false)` Specifies whether a non-default VTOR exists on the target
    
UVisor
######

.. json:object:: uvisor

    `uVisor <https://github.com/ARMmbed/uvisor>`__ RTOS security settings
    
    *Specific to STM32F4* targets*
    
    :property integer present: `(Default: 0. Values: 0, 1)` Specifies whether uVisor is present on the target device
    
    **Example**::
    
        {
            "uvisor": {
              "present": 0
            }
        }
    
GCC
###
    
.. json:object:: gcc

    Project compiler options
    
    :property boolean printf-float: Enables floating point support in ``printf`` commands. **Note:** Must be ``false`` for MSP430* targets
    
    **Example**::
    
        {
            "gcc": {
              "printf-float": false
            }
        }

module.json
-----------

The Kubos project's `module.json` file is originally based on `Yotta's module.json file <http://docs.yottabuild.org/reference/module.html>`__

Default Configurations
^^^^^^^^^^^^^^^^^^^^^^

When you run ``kubos init``, a `module.json` file is created for you with some default values.

KubOS RT Default File::

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
            "kubos-rt": "kubostech/kubos-rt#~0.1.0"
        },
        "homepage": "https://<homepage>",
        "description": "Example app running on kubos-rt."
    }
    

KubOS Linux Default File::

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
            "csp": "kubostech/libcsp#~1.5.0"
        },
        "homepage": "https://<homepage>",
        "description": "Example app running on KubOS Linux."
    }

Relevant Configuration Options
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

These are the configuration options which are most likely to be changed for a project.
(For all other options, refer to `Yotta's documentation <http://docs.yottabuild.org/reference/module.html>`__.)

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
    
    **WARNING: "kubos-rt" is a required dependency for all KubOS RT projects**
    
    :property string {component}: Project dependency location and/or version
    
    Available dependency name/value pairs (hierarchy denotes included dependecies. Italics denotes Yotta targetDependencies):
    
    - "cmd-control-client": "kubostech/cmd-control-client"
    
        - "csp": "kubostech/libcsp"
        - "command-and-control": "kubostech/command-and-control"
        - "ipc": "kubostech/ipc"
        - "tinycbor": "kubostech/tinycbor"
        
    - "cmd-control-daemon": "kubostech/cmd-control-daemon"
    
        - "csp": "kubostech/libcsp"
        - "command-and-control": "kubostech/command-and-control"
        - "ipc": "kubostech/ipc"
        - "tinycbor": "kubostech/tinycbor"
        - "kubos-core": "kubostech/kubos-core"
        
    - "cmsis-core": "kubostech/cmsis-core"
    
        - `"cmsis-core-st": "kubostech/cmsis-core-st"`
        
            - `"cmsis-core-stm32f4": "kubostech/cmsis-core-stm32f4"`
            
                - "cmsis-core": "kubostech/cmsis-core"
                - "stm32cubef4": "kubostech/stm32cubef4"
                - `"cmsis-core-stm32f405rg": "kubostech/cmsis-core-stm32f405rg"`
                
                    - "cmsis-core": "kubostech/cmsis-core"
                    
                - `"cmsis-core-stm32f407xg": "kubostech/cmsis-core-stm32f407xg"`
                
                    - "cmsis-core": "kubostech/cmsis-core"
                    
    - "command-and-control": "kubostech/command-and-control"
    - "csp": "kubostech/libcsp"
    
        - `"freertos": "kubostech/freertos"`
        - `"kubos-hal": "kubostech/kubos-hal"`
        - `"tinycbor": "kubostech/tinycbor"`
        
    - "freertos": "kubostech/freertos"
    
        - `"cmsis-core": "kubostech/cmsis-core"`
        - `"freertos-config-stm32f4": "kubostech/freertos-config-stm32f4"`
        - `"freertos-config-msp430f5529": "kubostech/freertos-config-msp430f5529"`
        
    - "ipc": "kubostech/ipc"
    
        - "csp": "kubostech/libcsp"
        - "tinycbor": "kubostech/tinycbor"
        - `"kubos-rt": "kubostech/kubos-rt"`
        
    - "kubos-core": "kubostech/kubos-core"
    
        - "csp": "kubostech/libcsp"
        - "kubos-hal": "kubostech/kubos-hal"
        
    - "kubos-hal": "kubostech/kubos-hal"
    
        - "csp": "kubostech/libcsp"
        - `"kubos-hal-linux": "kubostech/kubos-hal-linux"`
        
            - "kubos-hal" : "kubostech/kubos-hal"
            
        - `"kubos-hal-msp430f5529": "kubostech/kubos-hal-msp430f5529"`
        
            - "kubos-hal" : "kubostech/kubos-hal"
            - "msp430f5529-hal": "kubostech/msp430f5529-hal"
            
        - `"kubos-hal-stm32f4": "kubostech/kubos-hal-stm32f4"`
        
            - "kubos-hal": "kubostech/kubos-hal"
            - `"stm32cubef4-stm32f405rg": "kubostech/stm32cubef4-stm32f405rg"`
            
                - "cmsis-core": "kubostech/cmsis-core"
                
            - `"stm32cubef4-stm32f407vg": "kubostech/stm32cubef4-stm32f407vg"`
            
                - "cmsis-core": "kubostech/cmsis-core#"
                
    - "kubos-rt": "kubostech/kubos-rt"
    
        - "freertos": "kubostech/freertos"
        - "csp": "kubostech/libcsp"
        - "kubos-hal": "kubostech/kubos-hal"
        - "kubos-core": "kubostech/kubos-core"

    - "stm32cubef4": "kubostech/stm32cubef4"
    
        - `"stm32cubef4-stm32f405rg": "kubostech/stm32cubef4-stm32f405rg"`
        
            - "cmsis-core": "kubostech/cmsis-core"
            
        - `"stm32cubef4-stm32f407vg": "kubostech/stm32cubef4-stm32f407vg"`
        
            - "cmsis-core": "kubostech/cmsis-core"

    - "telemetry": "kubostech/telemetry"
    
        - "ipc": "kubostech/ipc"
        - "kubos-core": "kubostech/kubos-core"
        - `"telemetry-linux": "kubostech/telemetry-linux"`
        
              - "ipc": "kubostech/ipc"
              - "kubos-core": "kubostech/kubos-core"
              - "telemetry": "kubostech/telemetry"
              - "tinycbor": "kubostech/tinycbor"
              
        - `"telemetry-rt": "kubostech/telemetry-rt"`
        
              - "ipc": "kubostech/ipc"
              - "kubos-core": "kubostech/kubos-core"
              - `"kubos-rt": "kubostech/kubos-rt"`
              
    - "telemetry-aggregator": "kubostech/telemetry-aggregator"
    
        - "telemetry": "kubostech/telemetry"
        
    - "telemetry-storage": "kubostech/telemetry-storage"
    
        - "kubos-core": "kubostech/kubos-core"
        - "telemetry": "kubostech/telemetry"
        - `"kubos-rt": "kubostech/kubos-rt"`
        
    - "tinycbor": "kubostech/tinycbor"
    
    
    
    