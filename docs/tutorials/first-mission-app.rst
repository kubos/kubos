Creating Your First Mission Application
=======================================

This tutorial guides the user through the process of creating a basic mission application using
Python3.

At the end of the tutorial, the user will have a mission application which is capable of querying
the monitor service for current system memory usage and then storing that data into the telemetry
database.

You should be able to go through this tutorial entirely within your development environment.
You do not need to have an OBC available.

.. note:: 

    The iOBC does not support Python. If this is the board which you are using,
    please refer to the `example Rust mission application <https://github.com/kubos/kubos/blob/master/examples/rust-mission-app/src/main.rs>`__
    for the specific application code. The rest of this document should still be useful for the
    high-level concepts which are involved when developing a mission application.

Setup
-----

- :doc:`Install the Kubos SDK <../sdk-docs/sdk-installing>` or set up the dependencies
  required for a :doc:`local dev environment <../getting-started/local-setup>`
- If you have not done so already, create a clone of the `KubOS source repo <https://github.com/kubos/kubos>`__::

    $ git clone https://github.com/kubos/kubos
    
- Navigate to the kubos source directory and run the following commands to start the monitor service
  and telemetry database service in the background (the services may need to be built first, which
  will take several minutes to complete)::
  
    $ cargo run --bin monitor-service -- -c tools/default_config.toml &
    $ cargo run --bin telemetry-service -- -c tools/default_config.toml &
    
- Navigate back out to the development directory of your choosing.
  This tutorial will use ``/home/user/my-app`` as the example development directory and will assume
  that the cloned kubos repo is in ``/home/user/kubos``.

Mission Application Overview
----------------------------

Mission applications are user-created programs which are used to control satellite behavior and
execute mission logic.

These applications are registered with the :doc:`applications service <../ecosystem/services/app-service>`,
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

    #!/usr/bin/env python3
    
This allows the file to be run like a normal executable, ``./my-mission-app.py``, rather than needing
to explicitly call the Python interpreter with ``python my-mission-app.py``.

Since we'll be calling the file as an executable, we'll also need to update the file permissions::

    $ chmod +x my-mission-app.py

We'll then define our OnBoot and OnCommand run level functions with a basic print statement:

.. code-block:: python

    def on_boot():
        
        print("OnBoot logic")
        
    def on_command():
        
        print("OnCommand logic")

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
            print("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

.. note::
    
    This ``-r`` argument is used by the applications service, so must be included in all
    mission applications

All together, it should look like this:

.. code-block:: python

    #!/usr/bin/env python3
    
    import argparse
    import sys
    
    def on_boot():
        
        print("OnBoot logic")
        
    def on_command():
        
        print("OnCommand logic")
    
    def main():
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        
        args = parser.parse_args()
        
        if args.run == 'OnBoot':
            on_boot()
        elif args.run == 'OnCommand':
            on_command()
        else:
            print("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

We'll now run the program to verify that it's working as expected::

    $ ./my-mission-app.py -r OnBoot
    OnBoot logic
    $ ./my-mission-app.py -r OnCommand
    OnCommand logic
    
Kubos Services and GraphQL
--------------------------

A major component of most mission applications will be interacting with
:ref:`Kubos services <service-docs>`.

These services provided interfaces to underlying hardware and other system resources.

All services work by consuming `GraphQL <http://graphql.org/>`__ requests over HTTP, running the
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

Documentation for Kubos services can be found within the :ref:`services <service-docs>`
section.

For example, links to the schemas for all of the pre-built hardware services can be found
:ref:`here <pre-built-services>`.

Determining Service URLs
------------------------

In order to communicate with a service, we need to know where to send our messages.

All services rely on a configuration file, ``config.toml``, in order to determine which IP and port
they should bind a listener thread to.

By default, this file is located in ``/home/system/etc/config.toml``.
Since we're running these tutorials locally, that file location likely doesn't exist, so instead we
are using the ``tools/default_config.toml`` file in our cloned copy of the kubos repo.

We'll need to pass our application this path when we go to run it locally.

Querying a Service
------------------

For this tutorial, we'll be querying the :doc:`monitor service <../ecosystem/services/monitor-service>` for
the current amount of available memory.

The monitor service is a unique hardware service which communicates with the OBC itself in order to
obtain information about current processes running and the amount of memory both available and
generally present on the system.
It is unique because it is not tied to a particular hardware device and can, instead, be run on any
supported OBC (or in this instance, the local dev environment).
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

To make the communication process simpler, we'll be using the :doc:`Python app API <../ecosystem/apps/python-app-api>`
to send our GraphQL requests.

For each request, it:

    - Looks up the HTTP address of the service name which is given from the system's
      :doc:`config.toml <../ecosystem/services/service-config>` file
    - Wraps the given request into a proper HTTP packet and sends it to the target service
    - Parses the response message and checks for errors
    - Returns the message payload if the request was successful

To start, we'll import the API::

    import app_api

Then, we'll add a new command line option ``-c`` to allow us to pass a non-default config file for
testing purposes::

    parser.add_argument('--config', '-c')
    
    args = parser.parse_args()
    
    if args.config is not None:
        global SERVICES
        SERVICES = app_api.Services(args.config)
    else:
        SERVICES = app_api.services()
    
Then, we'll create the query we want to send, specifying only the item that we are interested in::

    request = '{ memInfo { available } }'

Next, we'll send the request to the monitor service::

    response = SERVICES.query(service="monitor-service", query=request)
    
And finally, we'll parse the result to get our current available memory quantity::

    data = response["memInfo"]
    available = data["available"]
    print("Current available memory: %d kB" % (available))

After adding error handling, our program should look like this:

.. code-block:: python

    #!/usr/bin/env python3

    import argparse
    import app_api
    import sys
    
    def on_boot():
        
        print("OnBoot logic")
        
    def on_command():

        request = '{ memInfo { available } }'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            print("Something went wrong: " + str(e))
            sys.exit(1)
        
        data = response["memInfo"]
        available = data["available"]
        
        print("Current available memory: %d kB" % (available))
    
    def main():
    
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        parser.add_argument('--config', '-c')
        
        args = parser.parse_args()
        
        if args.config is not None:
            global SERVICES
            SERVICES = app_api.Services(args.config)
        else:
            SERVICES = app_api.services()
        
        if args.run == 'OnBoot':
            on_boot()
        elif args.run == 'OnCommand':
            on_command()
        else:
            print("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()
    
If we run our program, the output should look like this::

    $ ./my-mission-app.py -r OnCommand -c ../kubos/tools/default_config.toml
    Current available memory: 496768 kB

Writing Data to the Telemetry Database
--------------------------------------

Now that we have a data point, we need to save it somewhere useful.
The telemetry database is the main storage location for all telemetry data.
The :doc:`telemetry database service <../ecosystem/services/telemetry-db>` is the preferred interface point
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
        print("Telemetry insert encountered errors: " + str(errors))
    else:
        print("Telemetry insert completed successfully")

With some additional error handling, our final application looks like this:

.. code-block:: python

    #!/usr/bin/env python3
    
    import argparse
    import app_api
    import sys
    
    def on_boot():
        
        print("OnBoot logic")
        
    def on_command():
        
        request = '{memInfo{available}}'
        
        try:
            response = SERVICES.query(service="monitor-service", query=request)
        except Exception as e: 
            print("Something went wrong: " + str(e))
            sys.exit(1)
        
        data = response["memInfo"]
        available = data["available"]
        
        print("Current available memory: %s kB" % (available))
        
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
            print("Something went wrong: " + str(e) )
            sys.exit(1)
            
        data = response["insert"]
        success = data["success"]
        errors = data["errors"]
        
        if success == False:
            print("Telemetry insert encountered errors: " + str(errors))
            sys.exit(1)
        else:
            print("Telemetry insert completed successfully")
    
    def main():
        
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r')
        parser.add_argument('--config', '-c')
        
        args = parser.parse_args()
        
        if args.config is not None:
            global SERVICES
            SERVICES = app_api.Services(args.config)
        else:
            SERVICES = app_api.Services()
        
        if args.run == 'OnBoot':
            on_boot()
        elif args.run == 'OnCommand':
            on_command()
        else:
            print("Unknown run level specified")
            sys.exit(1)
        
    if __name__ == "__main__":
        main()

If we run our program, the output should look like this::

    $ ./my-mission-app.py -r OnCommand -c ../kubos/tools/default_config.toml
    Current available memory: 497060 kB
    Telemetry insert completed successfully
    
Creating the Manifest File
--------------------------

In order for the applications service to properly maintain versioning information, we'll need to
create a new file, `manifest.toml`, to accompany our mission app.

This file has the following key values:

- ``name`` - The name of the application
- ``executable`` - (Optional) The name of the file to be called to begin application execution
- ``version`` - The version number of the application
- ``author`` - The author of the application

Our file should look like this::

    name = "my-mission-app"
    executable = "my-mission-app.py"
    version = "1.0"
    author = "Me"

Next Steps
----------

- :doc:`Running an application on an OBC <first-obc-project>`
- :doc:`Registering a mission application with the applications service <app-register>`
- :doc:`Fetching telemetry data from the database <querying-telemetry>`
