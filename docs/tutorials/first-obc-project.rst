Running a KubOS Project on an OBC
=================================

Once you have a KubOS project set up, you'll eventually want to test it on actual hardware.

This tutorial guides the user through the process of adding logging to a KubOS project
(a crucial component when running in-flight) and then installing and running it on a target OBC.

.. note:: 

    The iOBC does not support Python. If this is the board which you are using,
    please refer to the :doc:`../getting-started/using-rust`
    doc to get your project built and running on the OBC.

Setup
-----

- :doc:`Install the Kubos SDK <../sdk-docs/sdk-installing>` or set up the dependencies
  required for a :doc:`local dev environment <../getting-started/local-setup>`
- Have an OBC available with both Python and SSH capabilities
  (preferably with an :doc:`installation of Kubos Linux <../obc-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- We'll start with the base project from the :doc:`first mission app <first-mission-app>` tutorial,
  except we'll tweak our monitor service query to instead fetch the current amount of available
  memory

Setting Up Logging
------------------

When our mission application is running in-flight, we likely won't have constant access to ``stdout``.

As a result, it would be better if we were also routing our messages to a log file.
That way we can check the status of our application at our discretion.

Kubos Linux uses `rsyslog <https://www.rsyslog.com/>`__ to automatically route log messages to the
appropriate log file and then rotate those files when they become too large.

All user applications should setup their logging to write to the user facility.
This will cause all log messages to be routed to files in ``/home/system/log``,

.. note::

    Log files are traditionally stored in ``/var/log``. ``/var/log`` has been set up as a symlink to
    ``/home/system/log``.
    
Within this directory, there may be several files:

    - ``app-debug.log`` - Records all log messages
    - ``app-info.log`` - Records log messages with a priority of ``info`` or higher
    - ``app-warn.log`` - Records log messages with a priority of ``warn`` or higher

Additionally, there may be files which match one of the above names, but are suffixed with a time
stamp.
For example, ``app-debug.log.2018.12.01-00.12.07``.
These are archived log files. Each log file has a maximum file size.
Once this size is reached, the current file is renamed as an archive file and a new log file is started.
By default, nine archive files of each log type will be retained.
If a new archive file is created and there are already nine files, the oldest will be deleted.

More information about the logging infrastructure can be found in the
:doc:`Kubos Linux logging doc <../ecosystem/linux-docs/logging>`.

For ease-of-use, the Python applications API contains a helper function, ``logging_setup``,
which will make all of the system calls required in order to set up the logger for the application.
All the user needs to do is specify the name of the application which should be used when generating
log messages.

Logging should be setup like so:

.. code-block:: python

    import app_api
    
    logger = app_api.logging_setup("mission-app")
    
    # Write a test message
    logger.info("Test Message")

We'll update all informational messages to use ``logger.info`` instead of ``print``, and then all
error messages to use ``logger.error``.

Our resulting project code should look like this::

    #!/usr/bin/env python3
    
    import argparse
    import app_api
    import sys
    
    def main():
    
        logger = app_api.logging_setup("my-mission-app")
        
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--config', '-c')
        
        args = parser.parse_args()
        
        if args.config is not None:
            global SERVICES
            SERVICES = app_api.Services(args.config)
        else:
            SERVICES = app_api.Services()

        args = parser.parse_args()
        
        request = '{memInfo{available}}'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            logger.error("Something went wrong: " + str(e))
            sys.exit(1)
        
        data = response["memInfo"]
        available = data["available"]
        
        logger.info("Current available memory: %s kB" % (available))
        
        request = '''
            mutation {
                insert(subsystem: "OBC", parameter: "available_mem", value: "%s") {
                    success,
                    errors
                }
            }
            ''' % (available)
        
        try:
            response = SERVICES.query(service="telemetry-service", query=request)
        except Exception as e: 
            logger.error("Something went wrong: " + str(e))
            sys.exit(1)
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            logger.error("Telemetry insert encountered errors: " + str(errors))
            sys.exit(1)
        else:
            logger.info("Telemetry insert completed successfully")
        
    if __name__ == "__main__":
        main()

Logging in to KubOS
-------------------

By default, KubOS comes with a user account, ``kubos``, with the default password ``Kubos123``.

Log into your OBC using SSH and its configured IP address. Enter the password when prompted.

For example::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********

If this is your first time connecting to the board via SSH, you may be prompted to confirm
the target IP's authenticity. Enter "yes" if this occurs::

    $ ssh root@10.0.2.20
    The authenticity of host '10.0.2.20 (10.0.2.20)' can't be established.
    ECDSA key fingerprint is SHA256:ir2TC+iML+MJ5Cb3cxTReWI69aX6EtPysFQzWleKc+8.
    Are you sure you want to continue connecting (yes/no)? yes
    Warning: Permanently added '10.0.2.20' (ECDSA) to the list of known hosts.
    kubos@10.0.2.20's password: ********

Please confirm that you are able to connect to the board via SSH from you development environment
before proceeding with the next step. If you are unable to do so, please verify that your OBC's
network connection has been :ref:`successfully configured and activated <ethernet>`.

Once you are logged in to the OBC, you can use the ``exit`` command to end the SSH connection and
return to your host computer.

Transferring the Project to a Target OBC
----------------------------------------

We can now transfer the project to the ``kubos`` user home directory on the target OBC using SCP.
From your local command line, run the following (be sure to replace ``10.0.2.20`` with your OBC's
IP address)::

    $ scp my-mission-app.py kubos@10.0.2.20:/home/kubos
    kubos@10.0.2.20's password: ********
    my-mission-app.py                                     100% 1814     1.8KB/s   00:00
    
Running the Project on the Target OBC
-------------------------------------

Once the project has been transferred, we can log in to the OBC and run it::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # ./my-mission-app.py
    my-mission-app: Current available memory: 497060 kB
    my-mission-app: Telemetry insert completed successfully
    /home/kubos # cat /var/log/app-debug.log
    1970-01-01T03:23:13.246358+00:00 Kubos my-mission-app:<info> Current available memory: 497060 kB
    1970-01-01T03:23:13.867534+00:00 Kubos my-mission-app:<info> Telemetry insert completed successfully
    
Next Steps
----------

:doc:`Registering a mission application with the applications service <app-register>`