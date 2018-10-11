Rust Example Mission Application
================================

This script shows the basic layout a mission application written in Rust will use in order
to be successfully registered and run using the Kubos mission applications service.

It uses the Rust app API in order to reduce the amount of boilerplate code required.

Features:

    - Logging data to files
    - Requesting data from a service with a GraphQL query
    - Storing data into the telemetry database with a GraphQL mutation
    - Processing additional arguments beyond just ``-r``

Usage
-----

::

    rust-mission-app -r run_level [options]

Required Arguments::

    -r, --run_level     The run level logic which should be executed. Must be "OnBoot" or "OnCommand"

Optional Arguments::

    -s, --cmd_string    Command argument string passed into OnCommand behavior specifying which
    subcommand logic to execute
    -i, --cmd_int       When executing the 'safemode' subcommand, specifies how long the program
    should sleep for, in seconds
