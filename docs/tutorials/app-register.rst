Registering a Mission Application
=================================

The Kubos :doc:`applications service <../app-docs/app-service>` is responsible for monitoring and
managing all mission applications for a system.

This tutorial walks the user through:

    - Registering a new application
    - Sending a request to the applications service to start the application
    - Updating the application to a newer version
    - Verifying what versions of the application have been registered

Setup
-----

We'll be using the example application from the :doc:`mission application tutorial <first-mission-app>`.

However, we'll need to update the log files to use an absolute path.
Decide on an appropriate log location and then update the paths in your code.
The OnCommand log file should be changed from "oncommand-output" to "/path/to/oncommand-output".
The OnBoot log file should be changed from "onboot-output" to "/path/to/onboot-output".

In order to register it, we'll first need to log in to the OBC to set up a folder for the
application files::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # mkdir my-app

We can then transfer our application file, ``my-mission-app.py``, and our manifest file,
``manifest.toml``, to the new folder::

    $ scp my-mission-app.py kubos@10.0.2.20:/home/kubos/my-app
    kubos@10.0.2.20's password: ********
    my-mission-app.py                                     100% 1814     1.8KB/s   00:00
    $ scp manifest.toml kubos@10.0.2.20:/home/kubos/my-app
    kubos@10.0.2.20's password:
    manifest.toml                                         100%   56     0.1KB/s   00:00
    
Our application is now ready to be registered.

.. _graphiql:

GraphiQL
--------

All Kubos services which provide an HTTP interface have a special endpoint which can be used to
send and receive GraphQL data via an in-browser graphical interface, GraphiQL.

This graphical interface makes it easier to create and consume more lengthy GraphQL requests.

To access this endpoint, make sure that your OBC is available from your host computer via an IP
address, then open a web browser and navigate to ``http://{ip}:{port}/grapiql``.
The ``ip`` and ``port`` parameters should match the IP address of the OBC and the port belonging to
the service you wish to query.

The resulting interface should look like this:

.. figure:: ../images/graphiql.png

From here, you can enter any valid GraphQL query or mutation on the left-hand side and then run
the request by clicking the triangle button.
The resulting JSON response will be displayed on the right-hand side:

.. figure:: ../images/graphiql_ping.png

Please navigate to ``http://{ip}:8000/graphiql`` in order to communicate with the applications
service for this tutorial.

.. note::

    You may also send GraphQL requests by using the `curl <https://linux.die.net/man/1/curl>`__
    facility. Requests should be sent as POST operations, specifying the GraphQL request inside the
    body of the message under the "query" parameter. The content-type for the message should be
    "application/json".
    
    For example::
    
        $ curl 10.0.2.20:8008 -H "Content-Type: application/json" --data "{\"query\":\"{ping}\"}"

Registering
-----------

To register an application, we use the service's ``register`` mutation.
It has the following schema::

     mutation {
        register(path: String!) {
            success: Bool!,
            errors: String,
            entry: {
                app: {
                    name: String!,
                    version: String!,
                    author: String!,
                    executable: String!
                },
                active: Bool
            }
        }
     }
     
The ``path`` input parameter specifies the directory *on the OBC* where the application and manifest
files reside.
The registration process will copy all of the contents at that path, so care should be taken to
ensure that only the desired application files are present.

The mutation can return the following fields:

    - ``success`` - Indicating the overall result of the register operation
    - ``errors`` - Any errors which were encountered while registering the application

    - ``entry`` - The registration information about the newly registered application.
      Will be empty if the registration process fails

        - ``app``

            - ``name`` - The name of the registered application, taken from the manifest file
            - ``version`` - The version number of this particular iteration of the application, taken
              from the manifest file
            - ``author`` - The author information for the application, taken from the manifest file
            - ``executable`` - The absolute path of the file which will kick off execution of the
              newly registered application file

        - ``active`` - Specifies whether the newly registered application is the current active version
          of the application which will be used when the service attempts to run it. This value should
          always be ``true`` when returned by this mutation

We'll be interacting with the OBC from our SDK instance using the service's GraphiQL interface.
By default, the applications service uses port 8000.

Our registration mutation should look like this::

    mutation {
      register(path: "/home/kubos/my-app") {
        success,
        errors,
        entry {
          app {
            name
            executable
          }
        }
      }
    }
    
The response should like this::

    {
      "data": {
        "register": {
          "success": true,
          "errors": "",
          "entry": {
            "app": {
              "name": "my-mission-app",
              "executable": "/home/system/kubos/apps/my-mission-app/1.0/my-mission-app.py"
            }
          }
        }
      }
    }

We can break down the resulting executable path like so:

    - ``/home/system/kubos/apps`` - This is the default directory that the applications service uses to
      save all registered applications
    - ``my-mission-app`` - The name of our application
    - ``1.0`` - Our manifest file specified that this was version 1.0 of our application
    - ``my-mission-app.py`` - Our application file

Starting
--------

We'll go ahead and start our app now to verify it works using the ``startApp`` mutation.
It has the following schema::

    mutation {
        startApp(name: String!, runLevel: String!): {
            success: Bool!
            errors: String,
            pid: Int
        }
    }

The ``name`` input parameter specifies the name of the application which should be started.
The ``runLevel`` input parameter specifies which run case should be called; it must be either
"OnBoot" or "OnCommand".

The mutation returns three fields:

    - ``success`` - Indicating the overall result of the operation
    - ``errors`` - Any errors which were encountered while starting the application
    - ``pid`` - The PID of the started application. This will be empty if any errors are encountered

Our request should look like this::

    mutation {
      startApp(name: "my-mission-app", runLevel: "OnCommand") {
        success,
        pid
      }
    }

And the response should look like this::

    {
      "data": {
        "startApp": {
          "success": true,
          "pid": 575
        }
      }
    }

To verify that the app ran successfully, we'll check the contents of our log file::

    $ ssh kubos@10.0.2.20
    kubos@10.0.2.20's password: ********
    /home/kubos # cat oncommand-output
    Current available memory: 496768 kB

Updating
--------

After looking at our log output, it would be nice if our log message included the timestamp of
when the system memory was checked.

Let's add the ``datetime`` module to our file with ``import datetime`` and then update the log line like so:

.. code-block:: python

    file.write("%s: Current available memory: %s kB \r\n" % (str(datetime.datetime.now()), available))

Since this is a new version of our application, we'll then need to update our ``manifest.toml``
file to change the ``version`` key from ``"1.0"`` to ``"2.0"``.

After transferring both of the files into our remote folder, ``/home/kubos/my-app``,
we can register the updated application using the same ``register`` mutation as before::

    mutation {
      register(path: "/home/kubos/my-app") {
        success,
        errors,
        entry {
          app {
            name
            executable
          }
        }
      }
    }

The response should look almost identical::

    {
        "errors": "",
        "data": {
            "register": {
                "success": true,
                "errors": "",
                "entry": {
                    "app": {
                        "name":"my-mission-app",
                        "executable":"/home/system/kubos/apps/my-mission-app/2.0/my-mission-app.py",
                    }
                }
            }
        }
    }
    
After running our app again with the ``startApp`` mutation, our log file should now look like this:

.. code-block:: none

    /home/kubos # cat oncommand-output
    Current available memory: 496768 kB
    1970-01-01 01:11:23.947890: Current available memory: 496952 kB

Verifying
---------

We can now query the service to see the registered versions of our application using the ``apps`` query.

The query has the following schema::

    {
        apps(name: String, version: String, active: Bool): [{
            app: {
                name: String!,
                version: String!,
                author: String!,
                executable: String!
            },
            active: Bool
        }]
    }
    
By default, the query will return information about all versions of all registered applications.
The queries input fields can be used to filter the results:

    - ``name`` - Returns entries with this specific application file name
    - ``version`` - Returns only entries with the specified version
    - ``active`` - Returns only the current active version of the particular application

The query has the following response fields:

    - ``app``

        - ``name`` - The name of the application
        - ``version`` - The version number of this particular iteration of the application
        - ``author`` - The author information for the application
        - ``executable`` - The absolute path of the file which will kick off execution of the
          registered application file

    - ``active`` - Specifies whether this iteration of the application is the current active version
      which will be used when the service attempts to run it

We want to query the service to make sure that:

    - We have two registered versions of our application
    - Version 2.0 is the current active version

Our request should look like this::

    {
      apps(name: "my-mission-app") {
        active
        app {
          name
          version
        }
      }
    }

The response should look like this::

    {
        "data": {
            "apps": [
                {
                    "active":false,
                    "app": {
                        "name":"my-mission-app",
                        "version":"1.0"
                    }
                },
                {
                    "active":true,
                    "app": {
                        "name":"my-mission-app",
                        "version":"2.0"
                    }
                }
            ]
        }
    }