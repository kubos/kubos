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
          },
          "i2c1": {
            "device": "/dev/i2c-0" // kubos-linux-isis-gcc
          }
        },
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
      "TEST": {
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

    #define YOTTA_CFG_TEST_MY_ADDRESS 1
    #define YOTTA_CFG_TEST_TARGET_ADDRESS 2
    #define YOTTA_CFG_TEST_PORT 10
    #define YOTTA_CFG_TEST_UART_BUS K_UART6
    #define YOTTA_CFG_TEST_UART_BAUDRATE 115200
    #define YOTTA_CFG_TEST_USART      
    
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
    :property string destDir: `(Default: "/home/system/usr/local/bin")` Specifies flashing destination directory for all 
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

    :property i2c: Availability and properties of I2C
    :proptype i2c: :json:object:`i2c <hardware.i2c>`
    
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
                },
                "i2c1": {
                  "device": "/dev/i2c-1"
                },
                "i2c2": {
                  "device": "/dev/i2c-2"
                }
              }
            }
        }
    
.. json:object:: hardware.i2c.defaults

    Default I2C connection settings
    
    :property bus: The default I2C bus
    :proptype bus: :cpp:type:`KI2CNum`
    
.. json:object:: hardware.i2c.i2c{n}

    I2C bus definition
    
    :property string device: Linux bus device name   

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
        "dependencies":{},
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
    - "kubos-hal": "kubos/kubos-hal"
    
        - `"kubos-hal-linux": "kubos/kubos-hal-linux"`
        
            - "kubos-hal" : "kubos/kubos-hal"
        
    - "isis-iobc-supervisor": "kubos/isis-iobc-supervisor"
    - "tinycbor": "kubos/tinycbor"
