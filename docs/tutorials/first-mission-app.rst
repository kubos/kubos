Creating Your First Mission Application
=======================================

This tutorial guides the user through the process of creating a basic mission application using
Python2.7.

At the end of the tutorial, the user will have a mission application which is capable of querying
the monitor service for current system memory usage and then storing that data into the telemetry
database.

.. note:: 

    The iOBC does not support Python. If this is the board which you are using,
    please refer to the `example Rust mission application <https://github.com/kubos/kubos/blob/master/examples/rust-mission-app/src/main.rs>`__
    for the specific application code. The rest of this document should still be useful for the
    high-level concepts which are involved when developing a mission application.

Pre-Requisites
--------------

- :doc:`Install the Kubos SDK <../installation-docs/sdk-installing>`
- Have an OBC available with both Python and SSH capabilities
  (preferably with an :doc:`installation of Kubos Linux <../installation-docs/index>`)

    - :ref:`Configuring Ethernet <ethernet>`

- Have the monitor service and telemetry database service running on the target OBC
  (this happens by default when running KubOS)

Mission Application Overview
----------------------------

Mission applications are user-created programs which are used to control satellite behavior and
execute mission logic.

These applications are registered with the :doc:`applications service <../app-docs/app-service>`,
which is responsible for tracking versioning, kicking off applications at boot time, and controlling
application upgrades and rollbacks.

Run Levels
~~~~~~~~~~

Run levels allow users the option to define differing behaviors depending on when and how their
application is started.

Each application must have a definition for each of the available run levels:

    - OnBoot - Defines logic which should be executed automatically at system boot time
    - OnCommand - Defines logic which should be executed when the :ref:`application is started manually <start-app>`

When the application is first called, the requested run level will be checked,
and then the corresponding run level function will be called.

Laying Out the Framework
------------------------

We'll be creating a new file for this tutorial, ``my-mission-app.py``.

In order to allow the applications service to run our mission application, we'll need to start by
placing the following line at the top of our new file::

    #!/usr/bin/env python
    
This allows the file to be run like a normal executable, ``./my-mission-app.py``, rather than needing
to explicitly call the Python interpreter with ``python my-mission-app.py``.

Since we'll be calling the file as an executable, we'll also need to update the file permissions::

    $ chmod +x my-mission-app.py

We'll then define our OnBoot and OnCommand run level functions with a basic print statement:

.. code-block:: python

    def on_boot():
        
        print "OnBoot logic"
        
    def on_command():
        
        print "OnCommand logic"

And, finally, we'll define our main function which will check for an ``-r`` command line argument
and then call the appropriate run level function based on the input:

.. code-block:: python
    
    import argparse
    import sys

    def main():
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot()
        elif args.run == 'OnCommand':
            on_command()
        else:
            print "Unknown run level specified"
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

.. note::
    
    This ``-r`` argument is used by the applications service, so must be included in all
    mission applications

All together, it should look like this:

.. code-block:: python

    #!/usr/bin/env python
    
    import argparse
    import sys
    
    def on_boot():
        
        print "OnBoot logic"
        
    def on_command():
        
        print "OnCommand logic"
    
    def main():
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot()
        elif args.run == 'OnCommand':
            on_command()
        else:
            print "Unknown run level specified"
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

We can test this program locally to verify that it's working as expected::

    $ ./my-mission-app.py -r OnBoot
    OnBoot logic
    $ ./my-mission-app.py -r OnCommand
    OnCommand logic

Adding Logging
--------------

When our mission application is running in-flight, we likely won't have constant access to ``stdout``.

As a result, it would be better if we were also routing our messages to a log file.
That way we can check the status of our application at our discretion.

Kubos Linux uses `rsyslog <https://www.rsyslog.com/>`__ to automatically route log messages to the
appropriate log file and then rotate those files when they become too large.

All user applications should setup their logging to write to the user facility.
This will cause all log messages to be routed to files in ``/home/system/log/apps``

.. note::

    Log files are traditionally stored in ``/var/log``. ``/var/log`` has been set up as a symlink to
    ``/home/system/log``.
    
Within this directory, there may be several files:

    - ``debug.log`` - Records all log messages
    - ``info.log`` - Records log messages with a priority of ``info`` or higher
    - ``warn.log`` - Records log messages with a priority of ``warn`` or higher

Additionally, there may be files which match one of the above names, but are suffixed with a time
stamp.
For example, ``debug.log.2018.12.01-00.12.07``.
These are archived log files. Each log file has a maximum file size.
Once this size is reached, the current file is renamed as an archive file and a new log file is started.
By default, five archive files of each log type will be retained.
If a new archive file is created and there are already five files, the oldest will be deleted.

More information about the logging infrastructure can be found in the
:doc:`Kubos Linux logging doc <../os-docs/logging>`.

Logging should be setup like so:

.. code-block:: python

    import logging
    from logging.handlers import SysLogHandler
    
    # Create a new logger. We'll give it a name that matches our application
    logger = logging.getLogger('my-mission-app')
    
    # Set the lowest log level which should be routed to rsyslog for processing
    logger.setLevel(logging.INFO)
    
    # Prefix all messages with the application name so that SysLog will set the
    # `programname` and `APP-NAME` property values accordingly.
    # This way we can easily see that the messages came from this application when viewing the log
    formatter = logging.Formatter('my-mission-app: %(message)s')
    
    # We'll send our messages to the standard Unix domain socket for logging (`/dev/log`)
    # Since this is a user program, we'll use the LOG_USER facility
    syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
    
    # Set the message formatting for this log handler
    syslog.setFormatter(formatter)

    # We also want to echo all our log messages to stdout, for easy viewing
    stdout = logging.StreamHandler(stream=sys.stdout)
    # Set the message formatting for this log handler
    stdout.setFormatter(formatter)
    
    # Finally, add our handlers to our logger
    logger.addHandler(syslog)
    logger.addHandler(stdout)
    
    # Write a test message
    logger.info("Test Message")


Our new file should look like this:

.. code-block:: python

    #!/usr/bin/env python
    
    import argparse
    import logging
    from logging.handlers import SysLogHandler
    import sys
    
    def on_boot(logger):
        
        logger.info("OnBoot logic")
        
    def on_command(logger):
        
        logger.info("OnCommand logic")
    
    def main():
    
        logger = logging.getLogger('my-mission-app')
        logger.setLevel(logging.INFO)
        formatter = logging.Formatter('my-mission-app: %(message)s')
        
        syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
        syslog.setFormatter(formatter)
        
        stdout = logging.StreamHandler(stream=sys.stdout)
        stdout.setFormatter(formatter)
        
        logger.addHandler(syslog)
        logger.addHandler(stdout)
        
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot(logger)
        elif args.run == 'OnCommand':
            on_command(logger)
        else:
            logger.error("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()
        
After transferring the file to the target OBC, we can log in to the OBC and test that the logging
works::

    $ scp my-mission-app.py kubos@10.0.2.20:/home/kubos
    kubos@10.0.2.20's password: ********
    my-mission-app.py                                    100%   970    1.0KB/s   00:00
    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # ./my-mission-app.py -r OnBoot
    my-mission-app: OnBoot logic
    /home/kubos # ./my-mission-app.py -r OnBoot
    my-mission-app: OnBoot logic
    /home/kubos # ./my-mission-app.py -r OnCommand
    my-mission-app: OnCommand logic
    /home/kubos # cd /var/log/apps
    /home/system/log/apps # ls
    debug.log  info.log
    /home/system/log/apps # cat info.log
    1970-01-01T03:23:08.491359+00:00 Kubos my-mission-app:<info> OnBoot logic
    1970-01-01T03:24:00.334330+00:00 Kubos my-mission-app:<info> OnBoot logic
    1970-01-01T03:27:20.841483+00:00 Kubos my-mission-app:<info> OnCommand logic
    
Kubos Services and GraphQL
--------------------------

A major component of most mission applications will be interacting with
:doc:`Kubos services <../services/index>`.

These services provided interfaces to underlying hardware and other system resources.

All services work by consuming `GraphQL <http://graphql.org/>`__ requests over UDP, running the
requested operation, and then returning a JSON response.

GraphQL is a query language which allows users to create readable requests which will return only
the data they specify.

GraphQL requests come in two varieties: queries and mutations.

Queries
~~~~~~~

GraphQL queries perform informational, read-only operations. For example, a query might request that
an underlying piece of hardware be contacted for its current temperature or last data reading.

An example query for the telemetry database service might look like this::

    {
        telemetry(subsystem: "EPS") {
            timestamp,
            parameter,
            value
        }
    }

This translates to "please fetch all of the stored telemetry entries for the EPS subsystem and
return only their timestamp, parameter, and value values."

The response might look like this::

    {
        "telemetry": [
            {
                "timestamp": 1100,
                "parameter": "voltage",
                "value": "4.4"
            },
            {
                "timestamp": 1100,
                "parameter": "current",
                "value": "0.25"
            },
            {
                "timestamp": 1002,
                "parameter": "voltage",
                "value": "4.5"
            },
            {
                "timestamp": 1002,
                "parameter": "current",
                "value": "0.20"
            }
        ]
    }

Mutations
~~~~~~~~~

GraphQL mutations perform actions which can be invasive or destructive, for example, writing data to
a file or rebooting a hardware device.

An example mutation for the telemetry database service might look like this::

    mutation {
        insert(subsystem: "GPS", parameter: "lock_status", value: "good") {
            success,
            errors
        }
    } 

This translates to "please create a new telemetry database entry for the GPS subsystem's lock status
parameter with a value of 'good'. Return the overall success of the operation and any errors."

Worth noting, all mutation requests are prefixed with ``mutation`` to quickly indicate to the service
what kind of action is being requested.

A successful response should look like this::

    {
        "insert": {
            "success": true,
            "errors": ""
        }
    }

If the request failed, the response might look like this::

    {
        "insert": {
            "success": false,
            "errors": "Failed to connect to database"
        }
    }
    
Schemas
~~~~~~~

Each service has a schema which defines all of its queries and mutations.

Users should refer to these to determine what actions are available for each service and how their
requests should be structured.

Documentation for Kubos services can be found within the :doc:`services <../services/index>` section.

For example, links to the schemas for all of the pre-built hardware services can be found
:ref:`here <pre-built-services>`.

Querying a Service
------------------

For this tutorial, we'll be querying the :doc:`monitor service <../services/monitor-service>` for
the current amount of available memory.

The monitor service is a unique hardware service which communicates with the OBC itself in order to
obtain information about current processes running and the amount of memory both available and
generally present on the system.
It is unique because it is not tied to a particular hardware device and can, instead, be run on any
supported OBC.
Worth noting, the process of communicating with this service is the same as communicating with any
other core or hardware service.

We intend for this to be an ad-hoc action, so we'll be adding code to the on-command section of
our program.

The service's ``memInfo`` query has the following schema::

    {
        MemInfo {
            total: Int,
            free: Int,
            available: Int,
            lowFree: Int,
        }
    }

This indicates that there are four possible return fields, however, the lack of an exclamation mark
means if any of them are not available on the system (for example, ``lowFree`` isn't available on
all systems), it will be omitted.

To make the communication process simpler, we'll be using the :doc:`Python app API <../app-docs/python-app-api>`
to send our GraphQL requests.

For each request, it:

    - Looks up the UDP port of the service name which is given from the system's `config.toml` file
    - Wraps the given request into a proper UDP packet and sends it to the target service
    - Parses the response message and checks for errors
    - Returns the message payload if the request was successful

To start, we'll import the API and create a constant for readability::

    import app_api
    
    SERVICES = app_api.services()
    
Then, we'll create the query we want to send, specifying only the item that we are interested in::

    request = '{ memInfo { available } }'

Next, we'll send the request to the monitor service::

    response = SERVICES.query(service="monitor-service", query=request)
    
And finally, we'll parse the result to get our current available memory quantity::

    data = response["memInfo"]
    available = data["available"]
    logger.info("Current available memory: %d kB \r\n" % (available))

After adding error handling, our program should look like this:

.. code-block:: python

    #!/usr/bin/env python

    import argparse
    import app_api
    import logging
    from logging.handlers import SysLogHandler
    import sys
    
    SERVICES = app_api.Services()
    
    def on_boot(logger):
        
        logger.info("OnBoot logic")
        
    def on_command(logger):

        request = '{ memInfo { available } }'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            logger.error("Something went wrong: " + str(e) + "\r\n")
            sys.exit(1)
        
        data = response["memInfo"]
        available = data["available"]
        
        logger.info("Current available memory: %d kB \r\n" % (available))
    
    def main():
        logger = logging.getLogger('my-mission-app')
        logger.setLevel(logging.INFO)
        formatter = logging.Formatter('my-mission-app: %(message)s')
        
        syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
        syslog.setFormatter(formatter)
        
        stdout = logging.StreamHandler(stream=sys.stdout)
        stdout.setFormatter(formatter)
        
        logger.addHandler(syslog)
        logger.addHandler(stdout)
    
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot(logger)
        elif args.run == 'OnCommand':
            on_command(logger)
        else:
            logger.error("Unknown run level specified\r\n")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()
    
Transferring the program to our OBC and running it should look like this::

    $ scp my-mission-app.py kubos@10.0.2.20:/home/kubos
    kubos@10.0.2.20's password: ********
    my-mission-app.py                                     100% 1078     1.1KB/s   00:00
    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # ./my-mission-app.py -r OnCommand
    my-mission-app: Current available memory: 496768 kB
    /home/kubos # cat /var/log/apps/debug.log
    1970-01-01T03:23:08.491359+00:00 Kubos my-mission-app:<info> Current available memory: 496768 kB

Writing Data to the Telemetry Database
--------------------------------------

Now that we have a data point, we need to save it somewhere useful.
The telemetry database is the main storage location for all telemetry data.
The :doc:`telemetry database service <../services/telemetry-db>` is the preferred interface point
for storing and retrieving that data.

We'll be using the service's ``insert`` mutation in order to add a new telemetry entry.
This operation is a mutation rather than a query, because it will cause the system to perform a write,
rather than simply reading data.

The mutation has the following schema::
    
    mutation {
        insert(timestamp: Integer, subsystem: String!, parameter: String!, value: String!) { 
            success: Boolean!, 
            errors: String!
        }
    }
    
This indicates that there are four possible input parameters, all of which are required except for
``timestamp``, and two return fields which, when requested, will always return a value.

Our mutation will have the following parameters:

    - subsystem: "OBC" - Indicating that our data point corresponds to the main OBC
      (other subsystem names might be things like "EPS" or "payload")
    - parameter: "available_mem" - Indicating that our data point represents the current amount of
      available memory
    - value - The data value which was returned from our previous query

All together, our request should look like this::

    request = '''
        mutation {
            insert(subsystem: "OBC", parameter: "available_mem", value: "%s") {
                success,
                errors
            }
        }
        ''' % (available)

Like before, we'll now use the app API to send our request, but this time we'll be sending to
the telemetry database service rather than the monitor service::

    response = SERVICES.query(service="telemetry-service", query=request)

Finally, we'll check the response to make sure the operation finished successfully::

    data = response["insert"]
    success = data["success"]
    errors = data["errors"]
    
    if success == False:
        logger.error("Telemetry insert encountered errors: " + str(errors) + "\r\n")
    else:
        logger.info("Telemetry insert completed successfully")

With some additional error handling, our final application looks like this:

.. code-block:: python

    #!/usr/bin/env python
    
    import argparse
    import app_api
    import logging
    from logging.handlers import SysLogHandler
    import sys
    
    SERVICES = app_api.Services()
    
    def on_boot(logger):
        
        logger.info("OnBoot logic")
        
    def on_command(logger):
        
        request = '{memInfo{available}}'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            logger.error("Something went wrong: " + str(e) + "\r\n")
            sys.exit(1)
        
        data = response["memInfo"]
        available = data["available"]
        
        logger.info("Current available memory: %s kB \r\n" % (available))
        
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
            logger.error("Something went wrong: " + str(e) + "\r\n")
            sys.exit(1)
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            logger.error("Telemetry insert encountered errors: " + str(errors) + "\r\n")
            sys.exit(1)
        else:
            logger.info("Telemetry insert completed successfully")
    
    def main():
        logger = logging.getLogger('my-mission-app')
        logger.setLevel(logging.INFO)
        formatter = logging.Formatter('my-mission-app: %(message)s')
        
        syslog = SysLogHandler(address='/dev/log', facility=SysLogHandler.LOG_USER)
        syslog.setFormatter(formatter)
        
        stdout = logging.StreamHandler(stream=sys.stdout)
        stdout.setFormatter(formatter)
        
        logger.addHandler(syslog)
        logger.addHandler(stdout)
        
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot(logger)
        elif args.run == 'OnCommand':
            on_command(logger)
        else:
            logger.error("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

Transferring the program to our OBC and running it should look like this::

    $ scp my-mission-app.py kubos@10.0.2.20:/home/kubos
    kubos@10.0.2.20's password: ********
    my-mission-app.py                                     100% 1814     1.8KB/s   00:00
    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # ./my-mission-app.py -r OnCommand
    my-mission-app: Current available memory: 497060 kB
    my-mission-app: Telemetry insert completed successfully
    /home/kubos # cat /var/log/apps/debug.log
    1970-01-01T03:23:08.491359+00:00 Kubos my-mission-app:<info> Current available memory: 496768 kB
    1970-01-01T03:23:13.246358+00:00 Kubos my-mission-app:<info> Current available memory: 497060 kB
    1970-01-01T03:23:13.867534+00:00 Kubos my-mission-app:<info> Telemetry insert completed successfully

.. note::

    If you'd like to double-check the results, you could add an additional action which sends a
    ``telemetry`` query to the telemetry database service to fetch the entries which were just added.
    
Creating the Manifest File
--------------------------

In order for the applications service to properly maintain versioning information, we'll need to
create a new file, `manifest.toml`, to accompany our mission app.

This file has the following key values:

- ``name`` - The name of the application
- ``version`` - The version number of the application
- ``author`` - The author of the application

Our file should look like this::

    name = "my-mission-app.py"
    version = "1.0"
    author = "Me"

Next Steps
----------

- Registering a mission application with the applications service
- Writing a deployment application