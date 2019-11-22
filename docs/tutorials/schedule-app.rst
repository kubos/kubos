Scheduling a Mission Application's Execution
============================================

The Kubos :doc:`scheduler service <../ecosystem/services/scheduler>` is
responsible for scheduling applications for execution.

This tutorial walks the user through:

    - Creating a new schedule mode
    - Creating a task list specifying the application execution details
    - Importing the task list into the scheduler for execution
    - Activating the schedule mode to begin execution

This tutorial is written to be run entirely within your local development
environment, however, you may also interact with the scheduler service on
your OBC by setting up its :ref:`ethernet connection <ethernet>`.

Setup
-----

This tutorial will build on the :doc:`application registration tutorial <app-register>`.
It assumes the application, monitor, and telemetry services are running. It also assumes
the :doc:`example mission application <first-mission-app>` exists and is registered
with the application service.

This tutorial will use the following example directories:

    - ``/home/user/kubos`` - Cloned copy of the kubos responsible
    - ``/home/user/kubos/schedules`` - Directory used by the schedules service
      to store schedule information

- Navigate to ``/home/user/kubos`` (or your preferred copy of the kubos repo).

- Run the following command to start the scheduler service in the background
  (the service may need to be built first, which will take several minutes to complete)::

    $ cargo run -p scheduler-service -- -c tools/local_config.toml &

GraphiQL
--------

All Kubos services which provide an HTTP interface have a special endpoint which
can be used to send and receive GraphQL data via an in-browser graphical
interface, GraphiQL.

To access this endpoint for the scheduler service, make sure the service has
started, then open a web browser and navigate to ``http://127.0.0.1:8010/graphiql``.
This URL assumes that the stock version of ``local_config.toml`` is in use.

Creating a Mode
---------------

Modes are the highest level of schedule organization. They are made up of lists of tasks
which are loaded into the scheduler for future execution. Each mode is represented by a
directory in the file system which can hold any number of task lists.

To create a new scheduler mode, we use the service's ``createMode`` mutation.
It has the following schema::

    mutation {
        createMode(name: String!) {
            success: Boolean,
            errors: String
        }
    }

The ``name`` input specifies the name of the mode to be created.

The creation action will create an empty, inactive mode that the scheduler
may interact with.

Our creation mutation should look like this::

    mutation {
        createMode(name: "nominal") {
            success
            errors
        }
    }

The response should look like this::

    {
        "data": {
            "createMode": {
                "success": true,
                "errors": ""
            }
        }
    }

Creating and Importing a Task List
----------------------------------

Task lists are the individual files stored in modes which contain the actual
schedule specification. Each task list contains a list of one or more tasks
to be scheduled and executed.

We'll go ahead and create a new task list which schedules ``my-mission-app``
to execute every 10 seconds, after an initial 10 second delay.

Create a new file called ``my-mission.json`` with the following contents::

    {
        "tasks": [
            {
                "description": "Execute mission logic",
                "delay": "10s",
                "period": "10s",
                "app": {
                    "name": "my-mission-app"
                }
            }
        ]
    }

The ``tasks`` list is required in each task list and holds all task specifications.
Each task requires a ``description``, one of either ``delay``, ``period``, or
``time`` to specify execution time, and ``app`` to specify the details of the app
execution. More details on the task list specification can be found
:ref:`here <schedule-specification>`.

To import a task list, we use the service's ``importTaskList`` mutation.
It has the following schema::

    mutation {
        importTaskList(path: String!, name: String!, mode:String!): {
            success: Boolean,
            errors: String
        }
    }

Our import mutation should look like this::

    mutation {
        importTaskList(name: "my-mission", path: "/home/user/kubos/my-mission.json", mode: "nominal") {
            success
            errors
        }
    }

The response should look like this::

    {
        "data": {
            "importTaskList": {
                "success": true,
                "errors": ""
            }
        }
    }

Activating the Schedule
-----------------------

The scheduler may only have one active mode at a time. The active mode is the one whose
tasks are loaded into the scheduler for future execution. New modes are inactive by default.
In order to load up our new task list and execute our example mission app, we must activate
the newly created mode.

To activate a mode, we use the service's ``activateMode`` mutation. It has the
following schema::

    mutation {
        activateMode(name: String!): {
            success: Boolean,
            errors: String
        }
    }

Our activation mutation should look like this::

    mutation {
        activateMode(name: "nominal") {
            success
            errors
        }
    }

The response should look like this::

    {
        "data": {
            "activateMode": {
                "success": true,
                "errors": ""
            }
        }
    }

At this point our mode and task list have been loaded into the scheduler and will begin execution.
The console where you started the app service should show the app's execution messages after 10
seconds and then every 10 seconds after that::

    Successfully pinged monitor service