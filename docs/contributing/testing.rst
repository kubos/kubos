Testing
=======

TODO: Fill out the rest of this

Unit Tests
~~~~~~~~~~

API unit tests should cover at least the following cases:

    - Good cases for all functions
    - Null pointer cases for each function pointer argument
    - Out-of-bounds cases for each function argument which is limited by more than its size (ex. ``uint8_t`` but max value of 3)

Rust
^^^^

Rust has `native support for unit tests <https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html>`__.

This can be leveraged by running ``cargo test`` from within the module folder.

Python
^^^^^^

Python's ``unittest`` and ``mock`` packages should be used to create unit tests
for Python APIs.

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
