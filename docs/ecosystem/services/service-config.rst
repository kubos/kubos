Configuring Services in KubOS
=============================

All Kubos services rely on a configuration file to determine certain runtime settings,
referred to as the ``config.toml`` file.

By default, this file lives in ``/etc/kubos-config.toml``.

Discovering Config Options
--------------------------

All Kubos :doc:`core services <core-services>` have a `Configuration` section in their repective
doc page which details the available configuration options.

All :ref:`hardware services <pre-built-services>` provided by Kubos document their configuration
options within their generated doc page.

Common Config Options
---------------------

All Kubos services will have a ``[{service}.addr]`` section.

For example::

    [kubos-monitor.addr]
    ip = "0.0.0.0"
    port = 8030

This section defines the IP address and port to be used for receiving :doc:`graphql` requests over HTTP.

In general, the ports being used follow the following convention:

- Kubos core services use ports 8000-8079
- Communications services use ports 8080-8099 for their downlink ports
- Hardware services use ports 8100 and up

Many hardware services will utilize a ``bus`` parameter which defines the particular peripheral bus
that the subsystem is connected to.

For example::

    [mai400-service]
    bus = "/dev/ttyS5"
    
This tells the MAI-400 service that the device is connected to the UART bus ``ttyS5``

Using Custom Config Files
-------------------------

By default, all services will attempt to read their configuration options from
``/etc/kubos-config.toml``.
This file is auto-generated when the KubOS image is built and lives in the root file system so that
it can be restored during the :doc:`OS recovery process <../linux-docs/kubos-linux-recovery>`.

.. warning::

    Any ad-hoc config changes made to the ``/etc/kubos-config.toml`` file will be lost if the OS is
    upgraded or restored.

As a result, if you would like to add or change any configuration options outside of the
:doc:`KubOS build process <../../deep-dive/klb/configuring-kubos>`, you should create a custom
config file within the user data partition.

This custom file location may be provided by specifying the path in the ``-c`` option when starting
a service.

For example::

    $ /usr/sbin/kubos-monitor-service -c /home/kubos/my-config.toml
    
.. note::

    When starting a Rust-based service from within the Kubos SDK, the config file should be passed
    like so::
    
        $ cargo run -- -c /home/kubos/my-config.toml
        
    The ``--`` characters make sure that the following parameters are passed to the underlying
    program, rather than to ``cargo``.
    
    
Creating Custom Config Options
------------------------------

Custom configuration options may be added to the ``config.toml`` file by following the
`TOML format <https://en.wikipedia.org/wiki/TOML>`__.

This format is intended to help create a simple, easy-to-read, configuration file.

All options should be placed under the header of the service which will be using them.
Sub-headers may be added as desired.

For example::

    [my-payload-service]
    watchdog-timeout = 60
        
    [my-payload-service.addr]
    ip = "0.0.0.0"
    port = 8100
    
    [my-payload-service.device]
    bus = "/dev/i2c-1"
    addr = 60
        
In this configuration, we are defining several custom options for a hypothetical
:doc:`payload service <payload-services>`, ``my-payload-service``:

    - ``watchdog-timeout`` defines the interval at which the service should kick the payload's watchdog
    - ``[my-payload-service.device]`` denotes a subsection of options devoted to the payload's I2C
      configuration

        - ``bus`` is the I2C bus the payload is connected to
        - ``addr`` is the decimal value of the payload's I2C address

Our service can then use these configuration options like so:

Python
~~~~~~

.. code-block:: python

    from kubos_service.config import Config
    import i2c
    import threading
    
    # Get the configuration options for the service out of the `config.toml` file
    config = Config("my-payload-service")
    
    # Get the watchdog timeout value
    timeout = config.raw['watchdog-timeout']
    
    # Start a thread which will kick the watchdog at the given interval
    threading.Thread(target=watchdog_kick, args=(timeout,)).start()
    
    # Get the I2C information
    bus = config.raw['device']['bus']
    addr = config.raw['device']['addr']
    
    # Set up the bus connection (actually only needs the bus number, which is the last character)
    i2c = i2c.I2C(bus[:-1])
    
    # Send a command to the device
    i2c.write(addr, [0x70])

Rust
~~~~

.. code-block:: rust

    use kubos_service::Config;
    use std::thread;
    use std::time::Duration;
    
    // Get the configuration options for the service out of the `config.toml` file
    let config = Config::new("my-payload-service").unwrap();

    // Get the watchdog timeout value
    let timeout = config
        .get("watchdog-timeout")
        .and_then(|val| val.as_integer())
        .expect("Unable to get timeout value");

    // Start a thread which will kick the watchdog at the given interval
    thread::spawn(move || loop {
        kick_watchdog();
        thread::sleep(Duration::from_secs(timeout as u64));
    });

    // Get the I2C information
    let device = config.get("device").unwrap();
    let bus = device["bus"].as_str().expect("Unable to get I2C bus");
    let addr = device["addr"].as_integer().expect("Unable to get I2C address");

    // Set up the bus connection
    let i2c = rust_i2c::Connection::from_path(&bus, addr as u16);

    // Send a command to the device
    let command = rust_i2c::Command {
        cmd: 0x70,
        data: vec![],
    };
    i2c.write(command);
    