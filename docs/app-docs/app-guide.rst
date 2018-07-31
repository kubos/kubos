KubOS Mission Applications Development Guide
============================================

Overview
--------

In order to be compatible with the applications service, mission applications must comply with the applications framework:

- The application should have separate handler functions for each supported run level
- The application must be packaged with a manifest file which details the name, version, and author information for the binary
    
Once an application has been built, it should be transferred to the system, along with its manifest, and then :ref:`registered with the applications service <register-app>`.

APIs
----

Users may write applications in the language of their choice.
However, Kubos provides APIs to assist and simplify with application development for use with one of our preferred languages.

Our supported languages for mission applications are:

.. toctree::
    :maxdepth: 1
    
    Python <python-app-api>
    Rust <rust-app-api>
    
These APIs abstract the run level definitions and provide helper functions for use when querying other system and hardware services.
    
Run Levels
----------

Run levels allow users the option to define differing behaviors depending on when and how their application is started.

Each application should have a definition for each of the available run levels:

    - OnBoot
    - OnCommand

When the application is first called, the run level will be fetched,
and then the corresponding run level function will be called.

It is acceptable to only have a single set of logic no matter which run level is specified.
In this case, each of the run level options should simply call the common logic function.

On Command
~~~~~~~~~~

The ``OnCommand`` run level defines logic which should be executed when the :ref:`application is started manually <start-app>`.

For example, a user might want a custom batch of telemetry to be gathered and returned occassionally.
Rather than sending individual telemetry requests, they could code their application to take care of the work,
so then they only have to send a single query in order to trigger the process.

On Boot
~~~~~~~

The ``OnBoot`` run level defines logic which should be executed when the applications service is started at system boot time.

This run level is frequently used for setting up continuous fetching and processing of data from the other system services and hardware.
For instance, an application might be set up to fetch the current time from a GPS device and then pass that information through to the ADCS device.

.. todo::

    On Deploy
    //~~~~~~~~~
    
    The ``on-deploy`` run level defines the deployment logic for the system.
    
    This is a special level which will be run when the system is started if deployment has not completed successfully yet.
    
    Two U-Boot variables help keep track of this: `deployed` - boolean indicating whether deployment has been completed or not,
    and `deploy_start` - a timestamp that's generated the first time deployment is started. This is used to keep track of the
    delay required between initial launch and when deployment is allowed to begin.

.. _app-manifest:

Application Manifest
--------------------

In order for the applications service to properly maintain versioning information, each application should be registered along with a manifest file, `manifest.toml`.

This file must have the following key values:

- ``name`` - The name of the application
- ``version`` - The version number of the application
- ``author`` - The author of the application

For example::

    name = "mission-app"
    version = "1.1"
    author = "Me"

Example Walkthrough
-------------------

Let's walkthrough the process of creating, installing, and updating an application
on a Beaglebone Black target OBC.

Creating
~~~~~~~~

For our application, we'll start by creating a simple skeleton application, 
containing only the required run level handlers and some simple debug statements.

Rust
^^^^

If we want our application to be written in Rust, we'll start by creating a new project::

    $ cargo new --bin example-mission-app
    $ cd example-mission-app

We'll update the `src/main.rs` file to have the following::

    #[macro_use]
    extern crate kubos_app;
    
    use kubos_app::*;
    use std::fs;
    
    struct MyApp;
    
    impl AppHandler for MyApp {
        fn on_boot(&self) {
            fs::write("/home/kubos/test-output", "OnBoot logic\r\n").unwrap();
        }
        fn on_command(&self) {
            fs::write("/home/kubos/test-output", "OnCommand logic\r\n").unwrap();
        }
    }
    
    fn main() {
        let app = MyApp;
        app_main!(&app);
    }

And then update the `config.toml` file to add the `kubos-app` dependency ::

    [dependencies]
    kubos-app = { path = "../../apis/app-api/rust" }
    
And then compile the project for the Beaglebone Black target::

    $ cargo build --target arm-unknown-linux-gnueabihf --release
    
The compiled binary will be in `example-mission-app/target/arm-unknown-linux-gnueabihf/release/example-mission-app`

Python
^^^^^^

If we want our application to be written in Python, we'll create a single new file, `example-mission-app`,
making sure to include ``#!/usr/bin/env python`` at the top of the file::

    #!/usr/bin/env python
    
    import argparse
    
    def on_boot():
        
        file = open("/home/kubos/test-output","w+")
        file.write("OnBoot logic\r\n")
        
    def on_command():
        
        file = open("/home/kubos/test-output","w+")
        file.write("OnBoot logic\r\n")
    
    def main():
        parser = argparse.ArgumentParser()
        
        parser.add_argument('--run', '-r', nargs=1, default='OnCommand')
        
        args = parser.parse_args()
        
        if args.run[0] == 'OnBoot':
            on_boot()
        else:
            on_command()
        
    if __name__ == "__main__":
        main()
        
And then we'll update the file permissions to allow execution::

    $ chmod +x example-mission-app
    
.. note::

    We're foregoing the usual ".py" extension so that the file name is the same as the Rust example file name. 

Manifest
^^^^^^^^

No matter which language we use, we'll need a companion `manifest.toml` file::

    name = "example-mission-app"
    version = "1.0"
    author = "Kubos User"

Transferring
~~~~~~~~~~~~

Next, we'll transfer the `example-mission-app` file and the `manifest.toml` file into a new
directory, `/home/kubos/example-app`, on the OBC. 

Registering
~~~~~~~~~~~

Once both files have been transferred, we can register the application using the ``register`` query::

    mutation {
        register(path: "/home/kubos/example-app") {
            app {
                uuid,
                name,
                version
            }
        }
    }

The response JSON should look like this::

    {
        "app": {
            "uuid": "60ff7516-a5c4-4fea-bdea-1b163ee9bd7a",
            "name": "example-mission-app",
            "version": "1.0"
        }
    }
    
.. note:: The UUID will be a custom value for each application which is registered

Starting
~~~~~~~~

We'll go ahead and start our app now to verify it works::

    mutation {
        startApp(uuid: "60ff7516-a5c4-4fea-bdea-1b163ee9bd7a", runLevel: "OnCommand")
    }
    
The response JSON should contain a number indicating the PID of our started application.

To verify that the app ran successfully, we'll check the contents of our new `test-output` file::

    $ cat /home/kubos/test-output
    OnCommand logic

Updating
~~~~~~~~

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
~~~~~~~~~

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