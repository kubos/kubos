Registering a Mission Application
=================================

The Kubos applications service is responsible for monitoring and managing all mission applications
for a system.

This tutorial walks the user through:

    - Registering a new application
    - Requesting the applications service to start the application
    - Updating the application to a newer version
    - Verifying what versions of an application have been registered

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
They must be the only files in this directory in order for the service to be able to complete the
registration process.

The mutating can return the following fields:

    - ``app``

        - ``uuid`` - The unique identifier for our newly registered application. This will be used for
          all future interaction with our application.
        - ``name`` - The name of the registered application, taken from the manifest file
        - ``version`` - The version number of this particular iteration of the application, taken
          from the manifest file
        - ``author`` - The author information for the application, taken from the manifest files
        - ``path`` - The abosolute path of the newly register application file

    - ``active`` - Specifies whether the newly registered application is the current active version
      of the application which will be used when the service attempts to run it. This should always
      return ``true`` when returned by this mutation

We'll be interacting with the OBC from our SDK instance using the netcat utility.
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

Now let's create a new version of our application.

We'll change the "OnCommand logic" string to "Updated OnCommand logic", and then update our `manifest.toml`
file to change the ``version`` key from ``"1.0"`` to ``"2.0"``.

After compiling (for Rust) and transferring the new files into a new folder, `/home/kubos/example-app-2`,
we can register the updated application::
 
    mutation {
        register(path: "/home/kubos/example-app-2") {
            app {
                uuid,
                name,
                version
            }
        }
    }

The returned UUID should match our original UUID::

    {
        "app": {
            "uuid": "60ff7516-a5c4-4fea-bdea-1b163ee9bd7a",
            "name": "example-mission-app",
            "version": "2.0"
        }
    }

Verifying
---------

We can now query the service to see all of the registered applications and versions::

    {
        apps {
            active,
            app {
                uuid,
                name,
                version
            }
    }

The response should show the two versions of our app, with the latest version being marked as active::

    {
        "apps": [
            { 
                "active": false,
                "app": {
                    "uuid": "60ff7516-a5c4-4fea-bdea-1b163ee9bd7a",
                    "name": "example-mission-app",
                    "version": "1.0"
                }
            },
            { 
                "active": true,
                "app": {
                    "uuid": "60ff7516-a5c4-4fea-bdea-1b163ee9bd7a",
                    "name": "example-mission-app",
                    "version": "2.0"
                }
            },
        ]
    }