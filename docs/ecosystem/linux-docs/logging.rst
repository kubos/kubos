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

    $outchannel service_debug,/var/log/kubos-debug.log,100000,/home/system/kubos/log-rotate.sh kubos-debug.log
    daemon.debug :omfile:$service_debug
    
Any of these components may be updated within their respective files in order to change the policy
to match the user's desired behavior.

Log Creation
~~~~~~~~~~~~

Log files are traditionally stored in ``/var/log``.

``/var/log`` has been set up as a symlink to ``/home/system/log``.
This way log files reside in permanent storage and will be preserved through OS upgrades.

Messages written using the `daemon` facility will be routed to files with the naming scheme
``kubos-*.log``.
This facility is used by all :ref:`Kubos services <service-docs>`.

Messages written using the `user` facility will be routed to files with the naming scheme
``app-*.log``.
This facility should be used by all :doc:`mission applications <../../mission-dev/index>`.

For each of these naming schemes, the following files may be created:

- ``*-debug.log`` - Records all log messages
- ``*-info.log`` - Records log messages with a priority of ``info`` or higher
- ``*-warn.log`` - Records log messages with a priority of ``warn`` or higher

On the SDK, logs can be found in ``/var/log/syslog``.

.. _log-rotation:

Log Rotation
~~~~~~~~~~~~

The ``/home/system/kubos/log-rotate.sh`` script is used to execute the rotation behavior.

By default, all debug log files have a maximum size of 100KB and all other log files have a maximum
size of 10KB.
This value can be updated by changing the max size parameter of the appropriate logging policy.

Once this size is reached, the current file is renamed as an archive file and a new log file is
started. Archive files use their original name, but are suffixed with the current timestamp.
For example, ``kubos-debug.log.2018.12.01-00.12.07``.

By default, nine archive files of each log type will be retained.
If a new archive file is created and there are already five files, the oldest will be deleted.
This value is controlled by the ``MAX_COUNT`` variable in the ``log-rotate.sh`` script.

Examples
--------

The following examples show how to set up and use the logging capabilities.

Rust
~~~~

Rust programs will use the standard `log framework crate <https://docs.rs/log/0.4.6/log/>`__ in
conjunction with a crate capable of writing syslog messages. The ``kubos-system`` crate provides
a `common interface <https://github.com/kubos/kubos/blob/master/apis/system-api/src/logger.rs>`__
for initializing the syslog interface. The ``kubos-service`` crate re-exports this
interface for usage when building services:

.. code-block:: rust

    use failure::{Error, SyncFailure};
    use kubos_service::Logger;
    use log::{debug, error};
    
    fn main() -> Result<(), Error> {
        // Set the application/program name
        Logger::init("log-test").unwrap();
    
        debug!("this is a debug {}", "message");
        error!("this is an error!");
        Ok(())
    }

Utilizing the ``kubos_system::logger`` interface also exposes two optional command line arguments:

- The ``--stdout`` flag will enable logging to stdout.
- The ``-l log-level`` flag will control the verbosity of the logging. The following 
  log levels are available: ``error``, ``warn``, ``info``, and ``debug``.

Python
~~~~~~

Python programs will import two things: the main `logging library <https://docs.python.org/3/library/logging.html>`__
and the `SysLogHandler log handler <https://docs.python.org/3/library/logging.handlers.html#sysloghandler>`__.

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
