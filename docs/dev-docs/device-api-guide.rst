Kubos Device API Creation Guide/Framework
=========================================

This guide covers the process developers should go through when writing an API for a new device.

Our goal is to create an API which covers the *majority* of functions which a consumer might need.
By not implementing all possible functionality, we save ourselves a potentially large amount of development
time which can instead be put to better use.

Most device APIs will be created to be consumed by services. They are used to abstract away the particular
requirements of communicating with a particular device, making the service code much smaller and simpler and,
therefore, more readable.

For example, this code block::

    int fd = open("/dev/i2c-1", O_RDWR);

    ioctl(fd, I2C_SLAVE, 0x32);
    
    uint8_t cmd = 0x54;
    if(write(fd, &cmd, sizeof(cmd)) != sizeof(cmd)
    {
        perror("Write failed);
        close(fd);
        return -1;
    }
    
    uint32_t uptime;
    if(read(fd, &uptime, sizeof(uptime)) != sizeof(uptime)
    {
        perror("Read failed");
        close(fd);
        return -1;
    }
    
    close(fd);
    
becomes

::

    k_radio_init();
    
    uint32_t uptime;
    
    KRadioStatus status = k_radio_get_uptime(&uptime);
    
    if(status != RADIO_OK)
    {
        printf(stderr, "Failed to get radio uptime: %d\n", status);
        k_radio_terminate();
        return RADIO_ERROR;
    }
    
    k_radio_terminate();

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
For example, by using an argument in the API's ``{api}_init()`` function.

.. note::

    Historically, this kind of configuration has been done with `config.json` options, but this has been deprecated
    in favor of the dynamic configuration to make the interaction between C, Rust, and Python more smooth.
    
File Location
-------------

New APIs should be located in a new folder within the `apis` folder of the `Kubos repo <https://github.com/kubos/kubos>`__.

::

    +-- apis\
    |   +-- <new-c-api>\
    |       +-- <new-c-api>\
    |           +-- <new-api>.h
    |       +-- docs
    |           +-- Doxyfile
    |       +-- CMakeLists.txt
    |       +-- source\
    |           +-- main.c
    |       +-- test\
    |           +-- <test-set>
    |               +-- <test-set>.c
    |               +-- stubs.cmake
    |               +-- sysfs.c
    |
    +-- cargo-kubos\
    +-- ccan\
    +-- cmocka\
    
See the :doc:`C module development doc <../sdk-docs/sdk-c>` for steps to create a new Kubos module in C.

APIs written in Rust will reside under the same parent folder, but their files will be generated with ``cargo``.

Coding Style
------------

While each API is highly device-specific, our goal is to keep the overall styling and layout as similar as possible.
This makes the codebase much more maintainable and reduces the amount of effort required for a service developer
to navigate between APIs.

In addition to mimicing existing APIs, please refer to the :doc:`kubos-standards` doc for more specific coding rules.

Documentation
-------------

Documentation for new device APIs should be added to the `docs/apis/device-api` folder.

Each API will have two docs:

    - ``<new-api>_api.rst`` - Doxygen-generated API doc
    - ``<new-api>.rst`` - API's users guide

These docs should be added to the table of contents in `docs/apis/device-api/index.rst`.

To include the new files in doc generation:

    - Add an entry to ``breathe_projects`` in `docs/conf.py`
    - Add an entry to ``DOCS_DIRS`` in `tools/gendocs.py`

Doxygen
~~~~~~~

.. note:: This applies to APIs written in C. Rust APIs might function differently

Within the new API's folder, create a ``docs`` subfolder and add a ``Doxyfile`` file.
Feel free to copy ``Doxyfile`` from another API, just change the ``PROJECT_NAME`` value.

Within each header file of the API, add the following block to the top of the file in order for Doxygen to be able to process it::

    /**
     * @defgroup <project-name> <API description>
     * @addtogroup <project-name>
     * @{
     */

And then add this to the bottom of the file::

    /* @} */
    
Within the header files, all items should be documented using `Doxygen's formatting <http://www.doxygen.nl/manual/docblocks.html>`__.

The ``<new-api>_api.rst`` doc should contain the declarations needed for the API documentation generated by
Doxygen to be picked up and included in the final HTML.

Users Guide
~~~~~~~~~~~

The API's users guide should give an overview of the capabilities of the API.

It should cover things like:

    - Run-time configuration options
    - Complex functions
    - Available telemetry items (i.e. anything returned by a "get" function)

The API's main audience will be service developers, so write the documentaton with them in mind.

Doc Verification
~~~~~~~~~~~~~~~~

In order to generate the documentation locally, navigate to the top level of your copy of the Kubos repo and run ``tools/gendocs.py``.

This will generate the documentation HTML files in a new ``html`` folder, which you can then use to verify your new docs display as intended.

To verify your docs:

    - Make sure that the two new ``*.rst`` files are accessible through normal page clicks if you start at the top-level ``index.html``
    - Verify that any new hyperlinks work as intended
    - Make sure that ``tools/gendocs.py`` runs successfully without throwing any errors or warnings. Fix all warnings until the script runs cleanly.

Testing
-------

Despite the fact that it's usually done last and frequently simply ignored in the face of time-constraints,
creating the API's unit and integration tests is still very important.

The unit tests will be run by CircleCI with each code change.

The integration tests will normally be run as an automated suite.

Unit Tests
~~~~~~~~~~

API unit tests should cover at least the following cases:

    - Good cases for all functions
    - Null pointer cases for each function pointer argument
    - Out-of-bounds cases for each function argument which is limited by more than its size (ex. ``uint8_t`` but max value of 3)

C
^

Unit tests for APIs written in C are run using `CMocka <https://api.cmocka.org/>`__.

The C API should contain a ``test`` folder with a subfolder containing the test set/s (most APIs will only have one test set).

Within each test set should be three files:

    - ``<test-set>.c`` - The file containing the actual tests
    - ``sysfs.c`` - Stub functions for the underlying `sysfs` calls
    - ``stubs.cmake`` - Makes the stub functions available to the test builder/runner


Unit tests can be run locally by navigating to the test folder under the API folder,
creating a ``build`` dir in the test folder and running ``cmake .. && make``.

To run the tests the same way that CircleCI does, navigate to the top level of the Kubos repo and issue this command::

    $ python $PWD/tools/build.py --all-tests
    
Rust
^^^^

Rust has `native support for unit tests <https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html>`__.

This can be leveraged by running ``cargo test`` from within the module folder.

Python
^^^^^^

Python's ``unittest`` and ``mock`` packages should be used to create unit tests
for Python APIs.

Integration Tests
~~~~~~~~~~~~~~~~~

All integration tests live within `test/integration/linux`. The API's integration test should be a new Kubos project within that folder.

The project should test each function exposed by the API.

Results should be written to a file on the target board. Any errors should be written to both the results file and ``stderr``.

At the completion of the test, a success or failure message should be printed to ``stdout``/``stderr``.
This message can then be used by ``test_runner.py`` to determine if the test passed.

See the `integration test's README <https://github.com/kubos/kubos/tree/master/test/integration/linux>`__ for more information about running automation tests.

Manual Integration Tests
^^^^^^^^^^^^^^^^^^^^^^^^

Some device functionality might not lend itself to automated testing. For instance, testing a radio's ability to receive a message.

In this case, create a new document with the manual test cases. Each case should have execution steps and expected output.
Put this doc in the API's `test` folder.
