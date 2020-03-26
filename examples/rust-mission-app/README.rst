Rust Example Mission Application
================================

This script shows the basic layout a mission application written in Rust will use in order
to be successfully registered and run using the Kubos mission applications service.

It uses the Rust app API in order to reduce the amount of boilerplate code required.

Features:

    - Logging data to files
    - Requesting data from a service with a GraphQL query
    - Storing data into the telemetry database with a GraphQL mutation
    - Processing additional command-line options

Usage
-----

::

    rust-mission-app [options]

Optional Arguments::

    -c, --config        Optional path to a config file
    -s, --cmd_string    Command argument string passed into OnCommand behavior specifying which
    subcommand logic to execute
    -i, --cmd_int       When executing the 'safemode' subcommand, specifies how long the program
    should sleep for, in seconds

.. note::

    When starting a Rust-based app from within the Kubos SDK manually, the default ``/etc/kubos-config.toml`` config
    file will likely not exist, and so you will need to provide a config file. It will be similar to this::

        $ cargo run -- -c /home/vagrant/kubos/tools/local_config.toml

    The ``--`` characters make sure that the following parameters are passed to the underlying
    program, rather than to ``cargo``.
