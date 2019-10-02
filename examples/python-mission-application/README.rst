Python Example Mission Application
==================================

This project demonstrates what a basic mission application might look like.

Features:

    - Logging data to files
    - Requesting data from a service with a GraphQL query
    - Storing data into the telemetry database with a GraphQL mutation
    - Processing additional command-line options

Usage
-----

::

    python-mission-application.py [options]

Optional Arguments::

    -s, --cmd_string    Command argument string passed into OnCommand behavior specifying which
    subcommand logic to execute
    -i, --cmd_int       When executing the 'safemode' subcommand, specifies how long the program
    should sleep for, in seconds
