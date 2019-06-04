Kubos Device API Creation Guide
===============================

This guide covers the process developers should go through when writing an API for a new device.

Our goal is to create an API which covers the *majority* of functions which a consumer might need.

By not implementing all possible functionality, we save ourselves a potentially large amount of development
time which can instead be put to better use.

Most device APIs will be created to be consumed by services. They are used to abstract away the particular
requirements of communicating with a particular device, making the service code much smaller and simpler and,
therefore, more readable.

Research
--------

Read the hardware doc to get an idea of what commands are available.

Refer to the correlating service outline doc to figure out which commands/functions are important.
For example, when implementing a new ADCS API, refer to the ADCS service outline.

Create a list of functions you expect to implement.
If needed, create a list of potential functions you're unsure about.

Find a Kubos device API to model the new API's structure and naming conventions after.
Ideally, there will be an existing API within the same category as the new API;
for example, the TRXVU API if you were implementing a new radio.

If you have any questions about what should be implemented and how it should be structured,
talk with other KubOS developers or the KubOS product manager.

General API Framework
---------------------

Most APIs will likely implement most of these kinds of functions:

    - Init/terminate - Used to open/close the Linux device file
    - Configuration - Run-time configuration
    - Power - On, off, reset
    - Action commands - (Highly device specific) Kick watchdog, send/receive message, deploy antenna, change orientation, etc
    - Fetch Information - Get uptime, get system status, get system telemetry, get orientation, etc

Any internal configuration required (for example, setting an I2C slave address) should be done dynamically.
For example, by using an argument in the API's ``{API}::new()`` function.

.. note::

    Historically, this kind of configuration has been done with `config.json` options, but this has been deprecated
    in favor of the dynamic configuration to make the interaction between C, Rust, and Python more smooth.
    
File Location
-------------

New APIs should be located in a new folder within the `apis` folder of the `Kubos repo <https://github.com/kubos/kubos>`__.

::

    +-- apis\
    |   +-- adcs-api\
    |   +-- clyde-3g-eps-api\
    |   +-- isis-trxvu-api\
    |   +-- <new-api>\
    |
    +-- cargo-kubos\
    +-- ccan\
    +-- cmocka\

APIs may be written in Rust, Python, or C. The language that the corresponding service will be written in should determine
the language of choice for the API, as the service and the API will be tightly integrated.

Coding Style
------------

While each API is highly device-specific, our goal is to keep the overall styling and layout as similar as possible.
This makes the codebase much more maintainable and reduces the amount of effort required for a service developer
to navigate between APIs.

In addition to mimicking existing APIs, please refer to the :doc:`../../contributing/kubos-standards` doc for more specific coding rules.

Guidelines for Rust APIs
~~~~~~~~~~~~~~~~~~~~~~~~

All relevant functionality should be implemented on a struct which owns any primitives used for device communications.

.. code-block:: rust

    pub struct RadioDevice {
        conn: UartConnection
    }

    impl RadioDevice {
        pub fn new(uart_device: &str) -> RadioDevice { ... }

        pub fn read(&self) -> RadioResult<Vec<u8>> { ... }

        pub fn write(&self, data: &[u8]) -> RadioResult<()> { ... }
    }

Each API should implement their own ``Result`` type and ``Errors`` enum.

.. code-block:: rust

    pub enum RadioError { ... }

    pub type RadioResult<T> = Result<T, RadioError>

Please refer to the :doc:`../../sdk-docs/sdk-rust` doc for additional guidance on working with Rust.

Guidelines for C APIs
~~~~~~~~~~~~~~~~~~~~~

All relevant functionality should be exposed through public functions in a header file.

.. code-block:: c

    // radio_device.h
    
    KRadioStatus k_radio_init(char * bus);

    KRadioStatus k_radio_read(char * buffer);

    KRadioStatus k_radio_write(const char * data);

All structures or primitives used for device communication should private
and hidden in implementation files.

.. code-block:: c

    // radio_device.c

    static int radio_bus_handle = 0;

Each API should implement its own ``Status`` enum.

.. code-block:: c

    // radio_device.h

    typedef enum {
        RADIO_OK,     /** Everything is good */
        RADIO_ERROR,  /** Generic error */
        ...
    } KRadioStatus;

Please refer to the :doc:`../../sdk-docs/sdk-c` doc for additional guidance on working with C.

Guidelines for Python APIs
~~~~~~~~~~~~~~~~~~~~~~~~~~

All relevant functionality should be exposed through classes which store any data relevant to device communications.

.. code-block:: python

    class RadioDevice:
        connection = None

        def __init__(self, path):
            ...

        def read(self):
            ...
        
        def write(self, data):
            ...

Please refer to the :doc:`../../sdk-docs/sdk-python` doc for additional guidance on working with Python.

Documentation
-------------

Documentation for new device APIs should be added to the `docs/deep-dive/apis/device-api` folder.

Each API will have at least one doc:

    - ``<new-api>.rst`` - API's users guide

APIs which are written in C will require a second doc:

    - ``<new-api>_api.rst`` - Source-level API doc

These docs should be added to the table of contents in `docs/deep-dive/apis/device-api/index.rst`.

In addition to mimicking documentation for existing APIs, please refer to the :doc:`../../contributing/documentation` doc for more details on writing source-level documentation and verifying new documentation.

Users Guide
~~~~~~~~~~~

The API's users guide should give an overview of the capabilities of the API.

It should cover things like:

    - An overview of the hardware device
    - Run-time configuration options
    - Complex functions
    - Available telemetry items (i.e. anything returned by a "get" function)
    - A reference to the API's source-level documentation

The API's main audience will be service developers, so write the documentation with them in mind.

Testing
-------

Despite the fact that it's usually done last and frequently simply ignored in the face of time-constraints,
creating the API's unit and integration tests is still very important.

The unit tests will be run by CircleCI with each code change.

The integration tests will normally be run as an automated suite.

In addition to mimicking tests in existing APIs, please refer to the :doc:`../../contributing/testing` doc for more specific testing guidelines.