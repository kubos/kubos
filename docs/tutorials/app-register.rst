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
The OnCommand log file should be changed from "oncommand-output" to "/home/kubos/oncommand-output".
The OnBoot log file should be changed from "onboot-output" to "/home/kubos/onboot-output".

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

Registering
-----------

To register an application, we use the service's ``register`` mutation.
It has the following schema::

     mutation {
        register(path: String!) {
            app: {
                uuid: String!,
                name: String!,
                version: String!,
                author: String!,
                path: String!
            },
            active: Bool
        }
     }
     
The ``path`` input parameter specifies the directory where the application and manifest files reside.
They **must be the only files in this directory** in order for the service to be able to complete the
registration process.

The mutation can return the following fields:

    - ``app``

        - ``uuid`` - The unique identifier for our newly registered application. This will be used for
          all future interaction with our application
        - ``name`` - The name of the registered application, taken from the manifest file
        - ``version`` - The version number of this particular iteration of the application, taken
          from the manifest file
        - ``author`` - The author information for the application, taken from the manifest file
        - ``path`` - The abosolute path of the newly registered application file

    - ``active`` - Specifies whether the newly registered application is the current active version
      of the application which will be used when the service attempts to run it. This value should
      always be ``true`` when returned by this mutation

We'll be interacting with the OBC from our SDK instance using the `netcat <https://linux.die.net/man/1/nc>`__ utility.
By default, the applications service uses port 8000.

Our registration process should look like this::

    $ echo "mutation {register(path: \"/home/kubos/my-app\"){app{uuid,name,path}}}" | nc -uw1 10.0.2.20 8000
    {"errs":"","msg":{"register":{"app":{"name":"my-mission-app.py","path":"/home/system/kubos/apps/8052dbe9-bab1-428e-8414-fb72b4af90bc/1.0/my-mission-app.py","uuid":"8052dbe9-bab1-428e-8414-fb72b4af90bc"}}}}

Adding a bit of formatting, the response looks like this::

    {
        "errs": "",
        "msg": {
            "register": {
                "app": {
                    "name":"my-mission-app.py",
                    "path":"/home/system/kubos/apps/8052dbe9-bab1-428e-8414-fb72b4af90bc/1.0/my-mission-app.py",
                    "uuid":"8052dbe9-bab1-428e-8414-fb72b4af90bc"
                }
            }
        }
    }

We can break down the resulting file path like so:

    - ``/home/system/kubos/apps`` - This is the default directory that the applications service uses to
      save all registered applications
    - ``8052dbe9-bab1-428e-8414-fb72b4af90bc`` - This is the generated UUID of our application, which
      is echoed in the ``uuid`` response field
    - ``1.0`` - Our manifest file specified that this was version 1.0 of our application
    - ``my-mission-app.py`` - Our application file

Starting
--------

We'll go ahead and start our app now to verify it works using the ``startApp`` mutation.
It has the following schema::

    mutation {
        startApp(uuid: String!, runLevel: String!): Int!
    }

The ``uuid`` input parameter specifies the UUID of the application which should be started.
The ``runLevel`` input parameter specifies which run case should be called; it must be either
"OnBoot" or "OnCommand".

The mutation returns the process ID of the started application.

Using the UUID returned from our registration, our request should look like this::

    $ echo "mutation {startApp(uuid: \"8052dbe9-bab1-428e-8414-fb72b4af90bc\", runLevel: \"OnCommand\")}" \
    > | nc -uw1 10.0.2.20 8000
    {"errs":"","msg":{"startApp":501}}

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
 
    $ echo "mutation {register(path: \"/home/kubos/my-app\"){app{uuid,name,path}}}" | nc -uw1 10.0.2.20 8000

The returned UUID should match our original UUID::

    {
        "errs": "",
        "msg": {
            "register": {
                "app": {
                    "name":"my-mission-app.py",
                    "path":"/home/system/kubos/apps/8052dbe9-bab1-428e-8414-fb72b4af90bc/2.0/my-mission-app.py",
                    "uuid":"8052dbe9-bab1-428e-8414-fb72b4af90bc"
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
        apps(uuid: String, name: String, version: String, active: Bool): [{
            app: {
                uuid: String!,
                name: String!,
                version: String!,
                author: String!,
                path: String!
            },
            active: Bool
        }]
    }
    
By default, the query will return information about all versions of all registered applications.
The queries input fields can be used to filter the results:

    - ``uuid`` - Specifies that the service should only return entries with this UUID
    - ``name`` - Returns entries with this specific application file name
    - ``version`` - Returns only entries with the specified version
    - ``active`` - Returns only the current active version of the particular application

The query has the following response fields:

    - ``app``

        - ``uuid`` - The unique identifier for the application
        - ``name`` - The name of the application file
        - ``version`` - The version number of this particular iteration of the application
        - ``author`` - The author information for the application
        - ``path`` - The abosolute path of the registered application file

    - ``active`` - Specifies whether this iteration of the application is the current active version
      which will be used when the service attempts to run it

We want to query the service to make sure that:

    - We have two registered versions of our application
    - Version 2.0 is the current active version

Our request should look like this::

    $ echo "{apps(uuid:\"8052dbe9-bab1-428e-8414-fb72b4af90bc\"){active,app{name,version}}}" | nc -uw1 10.0.2.20 8000    

The response should look like this::

    {
        "errs": "",
        "msg": {
            "apps": [
                {
                    "active":false,
                    "app": {
                        "name":"my-mission-app.py",
                        "version":"1.0"
                    }
                },
                {
                    "active":true,
                    "app": {
                        "name":"my-mission-app.py",
                        "version":"2.0"
                    }
                }
            ]
        }
    }