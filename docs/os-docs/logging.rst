Logging
=======

Kubos Linux uses `rsyslog <https://www.rsyslog.com/>`__ to automatically route log messages to the
appropriate log file and then rotate those files when they become too large.

Default Behavior
----------------

Two logging policy files are included in ``/etc/rsyslog.d/``:

    - ``kubos-apps.conf`` - Specifies the logging and rotation policies for mission applications
    - ``kubos-services.conf`` - Specifies the logging and rotation policies for services

Within those files, and the main ``/etc/rsyslog.conf`` file which controls other system logs, each
policy has the following format::

    $outchannel {policy_name},/var/log/{destination_file},{max_size},{log_rotation_script}
    {facility}.{priority} :omfile:${policy_name}
    
An example policy looks like this::

    $outchannel service_debug,/var/log/kubos/debug.log,1000000,/home/system/kubos/kubos-rotate.sh debug.log
    daemon.debug :omfile:$service_debug
    
Any of these components may be updated within their respective files in order to change the policy
to match the user's desired behavior.

Log Creation
~~~~~~~~~~~~

Log files are traditionally stored in ``/var/log``.

``/var/log`` has been set up as a symlink to ``/home/system/log``.
This way log files reside in permanent storage and will be preserved through OS upgrades.

Messages written using the `daemon` facility will be routed to the appropriate log file in the
``/home/system/log/kubos`` folder.
This facility is used by all :doc:`Kubos services <../services/index>`.

Messages written using the `user` facility will be routed to the appropriate log file in the
``/home/system/log/apps`` folder.
This facility should be used by all :doc:`mission applications <../app-docs/index>`.

Within each of these sub-directories, the following files may be automatically created:

- ``debug.log`` - Records all log messages
- ``info.log`` - Records log messages with a priority of ``info`` or higher
- ``warn.log`` - Records log messages with a priority of ``warn`` or higher

Log Rotation
~~~~~~~~~~~~

The ``/home/system/kubos/kubos-rotate.sh`` script is used to execute the rotation behavior.

All log files have a maximum size of 1MB by default. This value can be updated by changing the
max size parameter of the appropriate logging policy.

Once this size is reached, the current file is renamed as an archive file and a new log file is
started. Archive files use their original name, but are suffixed with the current timestamp.
For example, ``debug.log.2018.12.01-00.12.07``.

By default, five archive files of each log type will be retained.
If a new archive file is created and there are already five files, the oldest will be deleted.
This value is controlled by the ``MAX_COUNT`` variable in the ``kubos-rotate.sh`` script.

Examples
--------

The following examples show how to set up and use the logging capabilities.

Rust
~~~~

Rust programs will use two crates to create syslog messages: `log <https://docs.rs/log/0.4.6/log/>`__
and `syslog <https://docs.rs/syslog/4.0.1/syslog/>`__

.. code-block:: rust

    #[macro_use]
    extern crate failure;
    #[macro_use]
    extern crate log;
    extern crate syslog;
    
    use failure::{Error, SyncFailure};
    use syslog::Facility;
    
    fn main() -> Result<(), Error> {
        if let Err(error) = syslog::init(
            // Log using the User facility (Kubos services will use LOG_DAEMON)
            Facility::LOG_USER,
            // Set the minimum log level we care about
            log::LevelFilter::Debug,
            // Set the application/program name
            Some("log-test"),
            // Have to do `map_err(SyncFailure::new)` in order to convert
            // error-chain Error into something `failure` can handle
        ).map_err(SyncFailure::new)
        {
            eprintln!("Failed to start logging: {}", error);
        }
    
        debug!("this is a debug {}", "message");
        error!("this is an error!");
        Ok(())
    }


Python
~~~~~~

Python programs will import two things: the main `logging library <https://docs.python.org/2/library/logging.html>`__
and the `SysLogHandler log handler <https://docs.python.org/2/library/logging.handlers.html#sysloghandler>`__.

.. code-block:: python

    import logging
    from logging.handlers import SysLogHandler
    
    # Create a new logger. The name here is unimportant
    logger = logging.getLogger('log-test')
    logger.setLevel(logging.DEBUG)
    
    # We'll send our messages to the standard Unix domain socket for logging.
    # Since this is a user program, we'll use the LOG_USER facility
    handler = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
    
    # Prefix all messages with the application name so that SysLog will set the
    # programname and APP-NAME property values accordingly, allowing us to filter
    # by application, if we so choose
    formatter = logging.Formatter('log-test: %(message)s')
    
    handler.formatter = formatter
    logger.addHandler(handler)
    
    logger.info("Test Message")
