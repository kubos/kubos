Configuring Services in KubOS
=============================

- concept of TOML files
- default location
- how to specify a custom config file when starting a service `-c` opt
- common parameters ({service.addr})
- custom parameters
- how to find a service's config options
- how to use the config.toml file when writing your own services

All Kubos services rely on a configuration file to determine certain run-time settings,
referred to as the ``config.toml`` file.

By default, this file lives in ``/home/system/etc/config.toml``.



Common Config Options
---------------------

All Kubos services will have a ``[{service}.addr]`` section.

For example::

    [kubos-monitor.addr]
    ip = "0.0.0.0"
    port = 8006

This section defines the IP address and port to be used for receiving :doc:`graphql` requests over HTTP.

Many hardware services will utilize a ``bus`` parameter which defines the particular peripheral bus
that the subsystem is connected to.

For example::

    [mai400-service]
    bus = "/dev/ttyS5"
    
This tells the MAI-400 service that the device is connected to the UART bus ``ttyS5``

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
        
In this configuration, I am defining several custom options for my hypothetical
:doc:`payload service <payload-services>`, ``my-payload-service``:

    - ``watchdog-timeout`` defines the interval at which my service should kick the payload's watchdog
    - ``[my-payload-service.device]`` denotes a subsection of options devoted to the payload's I2C
      configuration

        - ``bus`` is the I2C bus my payload is connected to
        - ``addr`` is the decimal value of the payload's I2C address

My service can then use these configuration options like so:

Python
~~~~~~

.. code-block:: python

    from kubos_service.config import Config
    import i2c
    import thread
    
    # Get the configuration options for my service out of the `config.toml` file
    config = Config("my-payload-service")
    
    # Get the watchdog timeout value
    timeout = config.raw['watchdog-timeout']
    
    # Start a thread which will kick the watchdog at the given interval
    thread.start_new_thread(watchdog_kick, timeout)
    
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
    
    # Get the configuration options for my service out of the `config.toml` file
    let config = Config::new("my-payload-service");
    
    # Get the watchdog timeout value
    let timeout = config.get("timeout").parse::<u8>().expect("Unable to get timeout value");
    
    # Start a thread which will kick the watchdog at the given interval
    thread::spawn(move || {
        loop {
            kick_watchdog();
            thread::sleep(Duration::from_secs(timeout));
        }    
    });
    
    # Get the I2C information
    let bus = config.get("device.bus");
    let addr = config.get("device.addr").parse::<u8>().expect("Unable to get I2C address");
    
    # Set up the bus connection
    let i2c = rust_i2c::Connection::from_path(&bus, addr);
    
    # Send a command to the device
    let command = rust_i2c::Command { cmd: 0x70, data: vec!() };
    i2c.write(command);    

Using Custom Config Files
-------------------------

By default, all services will attempt to read their configuration options from
``/home/system/etc/config.toml``.

A custom file location may be provided by specifying the path in the ``-c`` option when starting
the service.

For example::

    # /usr/sbin/kubos-monitor-service -c /home/kubos/my-config.toml