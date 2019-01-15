Creating Your Communications Service
====================================

All satellites must be capable of communicating with the ground.
The method used for this communication varies greatly from mission to mission.
As a result, a communications service which facilitates passing messages between the radio/s and the
rest of the satellite must be constructed on an individual basis.

This tutorial walks the user through the process of using the :doc:`communications service framework <TODO>`
in order to create a basic hardware service for a radio.
To allow testing, the radio connection will be simulated by a UART connection.

Overview
--------

- brief overview of how the comms service works
- description of steps of how to write a comms service

Configuration
-------------

Before we write any actual code, we want to update our system's ``config.toml`` file.

We'll name our service `radio-service`.
Under this new service name, we'll be adding two sections:

- A ``radio-service.addr`` section to define the IP and port for the GraphQL endpoint of our service
- A ``radio-service.comms`` section to define our communications settings

For the address section, we'll use the internal IP address, ``0.0.0.0``, and port 8020.

In the comms section, we'll define our satellite's IP address as ``0.0.0.0``, since internally our
tutorial satellite's services all use different ports on the same IP address, and our ground IP
address as ``192.168.0.1``.

We'll set our request timeout to one second.

The other options will be omitted, so the default values will be used.
This means that the service will use ports 13100 through 13149 to process incoming messages and no
downlink endpoints will be created.

The final ``config.toml`` section should look like this::

    [radio-service.addr]
    ip = 0.0.0.0
    port = 8020

    [radio-service.comms]
    timeout = 1000
    ground_ip = 192.168.0.1
    satellite_ip = 0.0.0.0

Writing the Service
-------------------

Now we can start our actual tutorial project.
All work will be done within a new Rust project, ``radio-service``, which is created by running
``cargo new --bin radio-service``.

Cargo.toml
~~~~~~~~~~

Edit the ``Cargo.toml`` file to have the following dependencies::

    comms-service = { git = https://github.com/kubos/kubos }
    failure = "0.1.2"
    kubos-service = { git = https://github.com/kubos/kubos }
    kubos-system = { git = https://github.com/kubos/kubos }
    log = "^0.4.0"
    log4rs = "0.8"
    log4rs-syslog = "3.0"
    serial = "0.4"
    
All the dependencies, with the exception of ``serial``, should be common to all services
implementing the communications service framework.

The ``serial`` dependency is included in order to provide support for UART communication.

.. note::

    This tutorial was written using the 2018 edition of Rust

Helper Functions
~~~~~~~~~~~~~~~~

There are a few helper functions which we'll need to set up for our main program to use.

There are three radio-specific functions which we'll need to define: initialization, write, and read.

Additionally, we'll need to set up our logging so that status and error messages can be properly
recorded.

Logging
^^^^^^^
We'll start by initializing our logging::

    use log::*;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs_syslog::SyslogAppender;
    
    // Initialize logging for the service
    // All messages will be routed to syslog and echoed to the console
    fn log_init() -> ServiceResult<()> {
        // Use custom PatternEncoder to avoid duplicate timestamps in logs.
        let syslog_encoder = Box::new(PatternEncoder::new("{m}"));
        // Set up logging which will be routed to syslog for processing
        let syslog = Box::new(
            SyslogAppender::builder()
                .encoder(syslog_encoder)
                .openlog(
                    "radio-service",
                    log4rs_syslog::LogOption::LOG_PID | log4rs_syslog::LogOption::LOG_CONS,
                    log4rs_syslog::Facility::Daemon,
                )
                .build(),
        );
    
        // Set up logging which will be routed to stdout
        let stdout = Box::new(ConsoleAppender::builder().build());
    
        // Combine the loggers into one master config
        let config = log4rs::config::Config::builder()
            .appender(log4rs::config::Appender::builder().build("syslog", syslog))
            .appender(log4rs::config::Appender::builder().build("stdout", stdout))
            .build(
                log4rs::config::Root::builder()
                    .appender("syslog")
                    .appender("stdout")
                    // Set the minimum logging level to record
                    .build(log::LevelFilter::Debug),
            )?;
    
        // Start the logger
        log4rs::init_config(config)?;
    
        Ok(())
    }
    
    fn main() -> ServiceResult<()> {
        // Initialize logging for the programs
        log_init()?;
        
        Ok(())
    }

Serial Initialization
^^^^^^^^^^^^^^^^^^^^^

The initialization function will need to create a connection to the serial port and set the ports
communication settings. It should return the final connection object in a mutex, since it will need
to be shared across multiple threads.

.. code-block::

    use std::sync::{Arc, Mutex};

    const BUS: &str = "/dev/ttyS2";
    const TIMEOUT: Duration = Duration::from_millis(100);
    
    // Initialize the serial bus connection for reading and writing from/to the "radio"
    pub fn serial_init() -> ServiceResult<Arc<Mutex<RefCell<serial::SystemPort>>>> {
        // Define our serial settings
        let settings = serial::PortSettings {
            baud_rate: serial::Baud115200,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };
    
        // Open a connection to the serial port
        let mut port = serial::open(BUS)?;
    
        // Save our settings
        port.configure(&settings)?;
        port.set_timeout(TIMEOUT)?;
    
        // Wrap the port in a mutex so that multiple threads can access it
        let conn = Arc::new(Mutex::new(RefCell::new(port)));
    
        Ok(conn)
    }
    
Write
^^^^^

Writing to the "radio" is straight-forward in this case.
There's no need to encapsulate the data in a radio-specific protocol (like AX.25) before writing it.

The function should take two arguments: the data to write and the serial port to write to.
We'll need to take ownership of the mutex and then perform a UART write.

.. code-block::

    // The write function that the comms service will use to write messages to the "radio"
    //
    // This function may be called from either a message handler thread or from a downlink endpoint
    pub fn write(conn: Arc<Mutex<RefCell<serial::SystemPort>>>, msg: &[u8]) -> ServiceResult<()> {
        let conn = match conn.lock() {
            Ok(val) => val,
            Err(e) => bail!("Failed to take mutex: {:?}", e),
        };
        let mut conn = conn.try_borrow_mut()?;
    
        conn.write(msg).and_then(|num| {
            debug!("Wrote {} bytes to radio", num);
            Ok(())
        })?;
    
        Ok(())
    }

Read
^^^^

TODO

.. code-block::

    // The read function that the comms service read thread will call to wait for messages from the
    // "radio"
    //
    // Returns once a message has been received
    pub fn read(conn: Arc<Mutex<RefCell<serial::SystemPort>>>) -> ServiceResult<Vec<u8>> {
        loop {
            // Note: These brackets force the program to release the serial port's mutex so that any
            // threads waiting on it in order to perform a write may do so
            {
                // Take ownership of the serial port
                let conn = match conn.lock() {
                    Ok(val) => val,
                    Err(e) => {
                        error!("Failed to take mutex: {:?}", e);
                        panic!();
                    }
                };
                let mut conn = conn.try_borrow_mut()?;
    
                // Loop until either a full message has been received or a non-timeout error has occured
                //
                // Note: This program was written for the Beaglebone Black. The BBB UART driver
                // (8250_omap.c) has a peculiar behavior where it will only read, at most, 48 bytes at
                // a time before triggering an interrupt and returning the bytes to the `read` caller.
                // As a result, we'll continue to make `read` calls until either a) we read less than
                // 48 bytes in one go, or b) the read call returns a timeout
                let mut packet = vec![];
                loop {
                    let mut buffer: Vec<u8> = vec![0; MAX_READ];
                    match conn.read(buffer.as_mut_slice()) {
                        Ok(num) => {
                            buffer.resize(num, 0);
                            packet.append(&mut buffer);
    
                            debug!("Read {} bytes from radio", packet.len());
    
                            if num < MAX_READ {
                                return Ok(packet);
                            }
                        }
                        Err(ref err) => match err.kind() {
                            ::std::io::ErrorKind::TimedOut => {
                                if packet.len() > 0 {
                                    return Ok(packet);
                                } else {
                                    break;
                                }
                            }
                            other => bail!("Radio read failed: {:?}", other),
                        },
                    };
                }
            }
    
            // Sleep for a moment so that other threads have the chance to grab the serial port mutex
            thread::sleep(Duration::from_millis(10));
        }
    }
    
Main Logic
~~~~~~~~~~

Now that the helper functions are in place, we can set up our main service logic.

Our project will need to:

    - Start logging
    - Intialize the connection with the serial port
    - Fetch the configuration settings from the ``config.toml`` file
    - Setup the final communication configuration
    - Start the communication service thread
    - Start the GraphQL endpoint logic which will loop forever to keep program from ending

Configuration
^^^^^^^^^^^^^

After setting up logging, we'll want to fetch our service's configuration settings from the
``config.toml`` file and extract the communications settings::
    
    fn main() -> ServiceResult<()> {
        // Initialize logging for the program
        log_init()?;
    
        // Get the main service configuration from the system's config.toml file
        let service_config = kubos_system::Config::new("uart-comms-service");
    
        // Pull out our communication settings
        let config = CommsConfig::new(service_config);
    }

Communication Initialization
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Now we'll be setting up our instance of the |CommsControlBlock|, which is the main control
structure used by the communications service framework in order to store all of the settings and
communication components.

The |CommsControlBlock| contains five elements:

    - A pointer to the function used to read messages from the radio
    - A list of pointers to functions used to write messages to the radio
    - The entity used to connect to the radio to read messages
    - The entity used to connect to the radio to write messages
    - The communication settings extracted in the previous step

Since we're using a single UART port for our communication, the read and write entities will be the
same: the initialized port structure.

Our read function pointer will correspond with the ``read`` helper function we created previously.

The write function list will consist of a single entry: the ``write`` helper function.

The initialization should look like this::

    fn main() -> ServiceResult<()> {
        // Initialize logging for the program
        log_init()?;

        // Get the main service configuration from the system's config.toml file
        let service_config = kubos_system::Config::new("uart-comms-service");

        // Pull out our communication settings
        let config = CommsConfig::new(service_config);
        
        // Initialize the serial port
        let conn = serial_init()?;
    
        // In this instance, reading and writing are done over the same connection,
        // so we'll just clone the UART port connection
        let read_conn = conn.clone();
        let write_conn = conn;
        
        // Tie everything together in our final control block
        let control = CommsControlBlock::new(
            Some(Arc::new(comms::read)),
            vec![Arc::new(comms::write)],
            read_conn,
            write_conn,
            config,
        );
    }

Starting Communication
^^^^^^^^^^^^^^^^^^^^^^

Finally, we can start our communication threads.
We'll use the ``CommsService::start<TODO>`` function, passing it our control block as well as a
|CommsTelemetry| instance to use for recording communication metrics.

For the moment, we'll put a loop at the end of our program to keep from exiting.

.. code-block::

    fn main() -> ServiceResult<()> {
        // Initialize logging for the program
        log_init()?;
        
        // Get the main service configuration from the system's config.toml file
        let service_config = kubos_system::Config::new("uart-comms-service");
        
        // Pull out our communication settings
        let config = CommsConfig::new(service_config);
    
        // Initialize the serial port
        let conn = serial_init(BUS)?;
    
        // Set up the comms configuration
        // In this instance, reading and writing are done over the same connection,
        // so we'll just clone the UART port connection
        let read_conn = conn.clone();
        let write_conn = conn;
    
        let control = CommsControlBlock::new(
            Some(Arc::new(read)),
            vec![Arc::new(write)],
            read_conn,
            write_conn,
            config,
        );
    
        // Start the comms service thread
        CommsService::start(control, Arc::new(Mutex::new(CommsTelemetry::default())))?;
        
        // TODO: Start the GraphQL service
        loop {}
    }

Testing
-------

`nc`

GraphQL
-------

Fetching telemetry and other standard hardware service stuff

.. |comms-service| raw:: html

    <a href="../rust-docs/comms_service/index.html" target="_blank">Framework Rust documentation</a>
    
.. |CommsControlBlock| raw:: html

    <a href="../rust-docs/comms_service/struct.CommsControlBlock.html" target="_blank">CommsControlBlock</a>

.. |CommsTelemetry| raw:: html

    <a href="../rust-docs/comms_service/struct.CommsTelemetry.html" target="_blank">CommsTelemetry</a>