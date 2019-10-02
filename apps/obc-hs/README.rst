OBC Housekeeping App
====================

This application is responsible for checking the operating status of the main OBC and taking
certain corrective and cleanup actions when necessary.

This application is written in such a way that it can be run on any supported OBC with minimal
changes.
Please refer to the following 'Configuration' section for settings which should be updated for your
particular system prior to execution.

It is expected that the OBC's scheduler will be configured to run this app at a regular interval
(recommended: every hour) in order to maintain the health of the system.

Requirements
------------

In order to properly function, the telemetry database service and monitor service should be
running on your target OBC.

Configuration
-------------

All configuration options are defined as constant values at the top of the `src/main.rs` file.

Before this application can be run on your particular stack, a couple configuration options should be
set:

- ``COMMS_SERVICE`` - (Default: ``"local-comms-service"``) The name of the comms service which should
  be used to send an emergency beacon if the OBC's filesystem becomes unrecoverably corrupted.
  This beacon may be disabled by making this value an empty string (``""``).
- ``DOWNLINK_PORT`` - (Default: ``14011``) The specific comms service port which the emergency
  beacon should be sent to. This port should match what is specified in the comms service's
  configuration.

A few other configuration settings are present which may be optionally updated:

- ``CONFIG_PATH`` - (Default: ``""/home/system/etc/config.toml"``) Location of the system's
  configuration file.
- ``TELEMETRY_AGE`` - (Default: 1 week) The maximum age allowed for a telemetry entry. Once this age
  is exceeded, the entry will be removed.
- ``RAM_{NOMINAL|HIGH|CRITICAL}`` - (Default: 50%|70%|80%) RAM usage thresholds. Informational log
  messages will be generated based on the current RAM threhold being hit. If the usage exceeds the
  ``RAM_CRITICAL`` value, recovery actions will be executed.
- ``DISK_{NOMINAL|HIGH|CRITICAL}`` - (Default: 50%|70%|80%) Disk usage thresholds. Informational log
  messages will be generated based on the current disk threhold being hit. If the usage exceeds the
  ``DISK_CRITICAL`` value, recovery actions will be executed.
- ``QUERY_TIMEOUT`` - (Default: 200ms) The length of time the app should wait for a response from a
  service after sending a GraphQL request.

Housekeeping Tasks
------------------

These are the specific housekeeping tasks which are run at the given interval (default: every hour).

If you would like to omit a particular task, comment out its section in `src/main.rs`.

Clean DB
~~~~~~~~

The ``clean_db`` function deletes all telemetry database entries which have exceeded the threshold
age (default: 1 week).

Check Memory Usage
~~~~~~~~~~~~~~~~~~

The ``check_mem`` function gets the current percentage of RAM and disk (permanent storage) which are
in use and checks them against the given threshold values (nominal, high, critical).

If the RAM usage percentage exceeds the critical threshold (default: 80%), then the system is
forcibly rebooted.
The assumption in this case is that a rogue process has an undiscovered memory leak.
As a result, rebooting the system will free the memory back up.

If the disk usage percentage exceeds the critical threshold (default: 80%), then the ``clean_db``
function is called again, however the threshold age is lowered to a smaller value (default: 1 day).
Reducing the amount of storage used by the telemetry database is the only common way of reducing
system storage usage.
This function should be updated to add functionality if there are other areas of storage which can
be cleaned up for your particular system.

Check Filesystem Status
~~~~~~~~~~~~~~~~~~~~~~~

The ``check_fs`` function verifies that the system can still read and write to/from the user data
partition.
If this check fails, then we can determine that the file system has become corrupted.
In this case, an emergency beacon will be sent out over the specified communications device.

Other failed checks simply write an error message to the system logs, however these logs normally
reside within the user data partition. As a result, if the file system becomes corrupted the logs
are no longer accessible, so we send a beacon instead.

Check for System Reset
~~~~~~~~~~~~~~~~~~~~~~

The ``check_reset`` function checks if the OBC has recently experienced a reboot.
If so, an error message is generated.

Worth noting, this will issue an error message whenever the system is first started, even if
power-on/reboot was done on purpose.
There's not an easy way to tell the difference between intentional and spurious reboots.

Ping Services
~~~~~~~~~~~~~

All Kubos services provide a "ping" request as part of their GraphQL schema.
The ``ping_services`` function submits this ping request to all defined services and records which
services fail to return the "pong" response.