Creating Your First Mission Application
=======================================

This tutorial guides the user through the process of creating a basic mission application using
Python2.7.

At the end of the tutorial, the user will have a mission application which is capable of querying
the monitor service for current system memory usage and then storing that data into the telemetry
database.

.. note:: 

    The iOBC does not support Python. If this is the board which you are using,
    please refer to TODO

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

In order to be compatible with the applications service, mission applications must comply with the
applications framework:

- The application should have separate handler functions for each supported run level
- The application must be packaged with a manifest file which details the name, version, and author
  information for the binary


Run Levels
~~~~~~~~~~

Run levels allow users the option to define differing behaviors depending on when and how their
application is started.

Each application should have a definition for each of the available run levels:

    - OnBoot
    - OnCommand

When the application is first called, the run level will be fetched,
and then the corresponding run level function will be called.

It is acceptable to only have a single set of logic no matter which run level is specified.
In this case, each of the run level options should simply call the common logic function.

On Command
^^^^^^^^^^

The ``OnCommand`` run level defines logic which should be executed when the :ref:`application is started manually <start-app>`.

For example, a user might want a custom batch of telemetry to be gathered and returned occassionally.
Rather than sending individual telemetry requests, they could code their application to take care of
the work, so then they only have to send a single query in order to trigger the process.

On Boot
^^^^^^^

The ``OnBoot`` run level defines logic which should be executed when the applications service is
started at system boot time.

This run level is frequently used for setting up continuous fetching and processing of data from the
other system services and hardware.
For instance, an application might be set up to fetch the current time from a GPS device and then
pass that information through to the ADCS device.


Laying Out the Framework
------------------------

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
        
    if __name__ == "__main__":
        main()

All together, it should look like this:

.. code-block:: python

    #!/usr/bin/env python
    
    import argparse
    
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

As a result, it would be better if we were routing all our messages to a log file.
That way we can check the status of our application at our discretion.

Because our on-boot logic will perform different tasks than our on-command logic, we'll have two
separate logging files, ``onboot-output`` and ``oncommand-output``.

Additionally, we don't know how many times our mission application will be called before we're able
to check the logs, so we'll open the files in "append" mode.

Our new file should look like this:

.. code-block:: python

    #!/usr/bin/env python
    
    import argparse
    
    def on_boot():
        
        file = open("onboot-output", "a+")
        file.write("OnBoot logic\r\n")
        
    def on_command():
        
        file = open("oncommand-output","a+")
        file.write("OnCommand logic\r\n")
    
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
        
    if __name__ == "__main__":
        main()
        
If we run the program locally, we can check that the files are being successfully created::

    $ ./my-mission-app.py -r OnBoot
    $ ./my-mission-app.py -r OnBoot
    $ cat onboot-output
    OnBoot logic
    OnBoot logic
    $ ./my-mission-app.py -r OnCommand
    $ cat oncommand-output
    OnCommand logic
    
GraphQL
-------

TODO

Querying a Service
------------------

From this point on, we'll be testing on the target OBC, rather than locally.

For this tutorial, we'll be querying the monitor service for the current amount of available memory.
We intend for this to be an ad-hoc action, so we'll be adding code to the on-command section of
our program.

The service has the following schema::

    TODO
    
To make the communication process simpler, we'll be using the Python app API to send our GraphQL
requests.

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
    file.write("Current available memory: %d kB \r\n" % (available))

After adding error handling, our program should look like this::

    #!/usr/bin/env python

    import argparse
    import app_api
    
    SERVICES = app_api.Services()
    
    def on_boot():
        
        file = open("onboot-output", "a+")
        file.write("OnBoot logic\r\n")
        
    def on_command():
        
        file = open("oncommand-output","a+")
        
        request = '{ memInfo { available } }'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            file.write("Something went wrong: " + str(e) + "\r\n")
            print "OnCommand logic encountered errors"
            exit()
        
        data = response["memInfo"]
        available = data["available"]
        
        file.write("Current available memory: %d kB \r\n" % (available))
        
        print "OnCommand logic completed successfully"
    
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
        
    if __name__ == "__main__":
        main()
    
Writing Data to the Telemetry Database
--------------------------------------

Now that we have a data point, we need to save it somewhere useful.
The telemetry database is the main storage location for all telemetry data.
The telemetry database _service_ is the preferred interface point for storing and retrieving that data.

We'll be using the service's ``insert`` mutation in order to add a new telemetry entry.
This operation is a mutation rather than a query, because it will cause the system to perform a write,
rather than simply reading data.

The mutation has the following schema::
    
    TODO

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

And finally, we'll check the response to make sure the operation finished successfully::

    data = response["insert"]
    success = data["success"]
    errors = data["errors"]
    
    if success == False:
        print "Telemetry insert encountered errors: " + str(errors)

With some additional error handling, our final application looks like this::

    #!/usr/bin/env python
    
    import argparse
    import app_api
    
    SERVICES = app_api.Services()
    
    def on_boot():
        
        file = open("onboot-output", "a+")
        file.write("OnBoot logic\r\n")
        
    def on_command():
        
        file = open("oncommand-output","a+")
        
        request = '{memInfo{available}}'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            file.write("Something went wrong: " + str(e) + "\r\n")
            print "OnCommand logic encountered errors"
            exit()
        
        data = response["memInfo"]
        available = data["available"]
        
        file.write("Current available memory: %s kB \r\n" % (available))
        
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
            file.write("Something went wrong: " + str(e) + "\r\n")
            print "OnCommand logic encountered errors"
            exit()
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            file.write("Telemetry insert encountered errors: " + str(errors) + "\r\n")
            print "OnCommand logic encountered errors"
        else :
            print "OnCommand logic completed successfully"
    
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
        
    if __name__ == "__main__":
        main()

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