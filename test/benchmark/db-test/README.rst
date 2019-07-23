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

``db-test`` can be run with an optional ``-i`` argument to specify the number of times each
test loop is run. For example, to test with 2000 iterations::

    $ ./db-test -c config.toml -i 1000


Tests
-----
The results of each test will be printed in a simple table that includes the test's name, the
average per-iteration execution time in microseconds (us), and the total execution time for all
iterations in microseconds.

Example::

   /home/kubos # ./db-test -c tlmdb-config.toml -i 1000
   NAME                           | Avg (us)   | Total (us)
   --------------------------------------------------------
   local_db_api_insert            | 50460      | 50460353
   local_db_api_insert_bulk       | 213        | 213957
   remote_gql_insert              | 64356      | 64356103
   remote_gql_insert_bulk         | 9608       | 9608876
   remote_udp_insert              | 87         | 87930
   Cleaned up 4423 test entries

Each test result is printed as soon as it is finished.

If any errors are encountered during a test, the program will panic.

Stand-Alone Database Inserts
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The ``local_db_api_insert`` and ``local_db_api_insert_bulk`` tests measure the average amount of
time it takes to do a single, or bulk database insert, in microseconds using the Kubos Telemetry
Database API directly (on a local database)

GraphQL Database Inserts
~~~~~~~~~~~~~~~~~~~~~~~~

The ``remote_gql_insert`` and ``remote_gql_insert_bulk`` tests measure the average amount of time
it takes to request that the telemetry database service perform a database insert via a GraphQL
mutation request.

UDP Send-Only
~~~~~~~~~~~~~

The ``remote_udp_test``  test measures the average amount of time it takes to send a UDP message
to the telemetry database service.  It mimics the logic used when sending a direct UDP database
insert message.
