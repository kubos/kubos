Telemetry Database Benchmark Tests
==================================

This project performs the benchmark tests on different features of the telemetry database.

It may be run either from the Kubos SDK or on an OBC running KubOS.

Pre-Requisites
--------------

The system must have a ``config.toml`` file available for this program to read.
If one is not present at the default location (``/home/system/etc/config.toml``), 
it may be provided with the ``-c {config.toml path}`` command-line argument.

Note: An example ``config.toml`` file is included in this project.

The telemetry database service must be running on the same system where the tests will be run.

Configuration
-------------

The ``main.rs`` file contains a constant, ``ITERATIONS``, which controls the number of times
each test loop is run. This value may be updated as you see fit.

Tests
-----

Stand-Alone Database Inserts
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

This test measures the average amount of time it takes to do a single database insert, in microseconds.

The test will output the following line:: 

    Average insert time after {x} runs: {y} us
    
Where ``{x}`` indicates the number of times the test loop was run and ``{y}`` indicates the average
time taken for the insert action within each loop iteration.

If any errors are encountered, the program will panic.

GraphQL Database Inserts
~~~~~~~~~~~~~~~~~~~~~~~~

This test measures the average amount of time it takes to request that the telemetry database
service perform a database insert via a GraphQL mutation request.

The test will output the following line:: 

    Average mutation time after {x} runs: {y} us
    
Where ``{x}`` indicates the number of times the test loop was run and ``{y}`` indicates the average
time taken for the insert action within each loop iteration.

If any errors are encountered, the program will panic.

UDP Send-Only
~~~~~~~~~~~~~

This test measures the average amount of time it takes to send a UDP message to the telemetry database service.
It mimics the logic used when sending a direct UDP database insert message.

The test will output the following line:: 

    Average UDP send time after {x} runs: {y} us
    
Where ``{x}`` indicates the number of times the test loop was run and ``{y}`` indicates the average
time taken for the insert action within each loop iteration.

If any errors are encountered, the program will panic.